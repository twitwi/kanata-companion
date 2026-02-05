<script setup lang="ts">
import KB from './kb/KB.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { readTextFile } from '@tauri-apps/plugin-fs'

import { ref, watchEffect, type Ref } from 'vue'
import { parse, type SExpression } from './sexpression'
import { useConfig } from './stores/config'
import { queryConfigPaths } from './kb/tools'

const config = useConfig()
await config.$tauri.start()

if (config.configFile === null || config.configFile === '') {
  await queryConfigPaths()
}

// actual app data
const layer = ref('default')
const layerDefs = ref({} as Record<string, Record<string, SExpression>>)
const rawConfig = ref({}) as Ref<SExpression>

// load config file and parse it as s-expression (lisp)
async function loadConfig(path: string) {
  const fileContent = await readTextFile(path)
  const fixedContent = fileContent.replace(/;;.*/g, '').replace(/(\s)([,'`])(\s)/g, '$1"$2"$3')
  console.log('Loaded config file:', fixedContent)
  const parsed = parse('(\n' + fixedContent + '\n)')
  console.log('Parsed config:', parsed)

  // process includes
  let ientry = 0
  while (ientry < (parsed).length) {
    const entry = (parsed)[ientry]
    if (Array.isArray(entry) && entry[0] === 'include') {
      const includePath = entry[1] as string
      const includedConfig = await loadConfig(path.replace(/\/[^\/]*$/, '/') + includePath)

      // merge included config into current parsed config
      for (const incEntry of includedConfig) {
        ; parsed.splice(ientry + 1, 0, incEntry)
      }
      // remove the include entry
      ; parsed.splice(ientry, 1)
    } else {
      ientry += 1
    }
  }

  // gather template defs
  const templateDefs: Record<string, SExpression> = {}
  for (const entry of parsed) {
    if (Array.isArray(entry) && entry[0] === 'deftemplate') {
      const templateName = `${entry[1]}`
      templateDefs[templateName] = entry.slice(2)
    }
  }
  console.log('Template definitions:', templateDefs)

  // process a list with possible template expansions
  const processEach = (entry: SExpression[]): SExpression[] => {
    const res = []
    for (const e of entry) {
      const pe = processTemplates(e)
      res.push(...pe)
    }
    return res
  }
  // recursive walker, expand templates, returns a list of entries
  const processTemplates = (entry: SExpression): SExpression[] => {
    if (Array.isArray(entry)) {
      if (entry[0] === 't!' || entry[0] === 'template-expand') {
        const templateName = `${entry[1]}`
        const templateDef = templateDefs[templateName]
        if (!templateDef) {
          console.warn('Template not found:', templateName)
          return [entry]
        }
        // clone the templateDef
        const clonedDef = JSON.parse(JSON.stringify(templateDef.slice(1)))
        // substitute parameters
        const argNames = templateDef[0] as SExpression
        for (const iarg in argNames) {
          const name = `${argNames[iarg]}`
          const value = entry[2 + Number(iarg)]!
          const replaceIn = (obj: string | SExpression): string | SExpression => {
            if (Array.isArray(obj)) {
              return obj.map(replaceIn)
            } else {
              return obj === `$${name}` ? value : obj
            }
          }
          for (let i = 0; i < clonedDef.length; i++) {
            clonedDef[i] = replaceIn(clonedDef[i])
          }
        }
        // simple recurse
        return processEach(clonedDef)
      } else {
        return [processEach(entry as SExpression[])]
      }
    }
    return [entry]
  }
  const processed = processEach(parsed as any[])
  console.log('Processed config with templates:', processed)

  rawConfig.value = processed
  return processed
}

function configToLayerDefs(parsed: any[]) {
  const defs: Record<string, Record<string, SExpression>> = {}
  for (const e of parsed) {
    if (Array.isArray(e) && e[0] === 'deflayermap') {
      const layerName = e[1] as string
      defs[layerName] = {}
      for (let i = 2; i < e.length; i += 2) {
        const key = e[i]
        const action = e[i + 1]
        defs[layerName][key] = action
      }
    }
  }
  return defs
  //$console.log('Layer definitions:', defs)
  //$layerDefs.value = defs
}

watchEffect(async () => {
  const newPath = config.configFile
  console.log('Config file changed to:', !!newPath)
  if (newPath) {
    const parsed = await loadConfig(newPath)
    layerDefs.value = configToLayerDefs(parsed)
  }
})

// interact with kanata backend
listen('kanata-message', (event) => {
  const msg = JSON.parse(event.payload?.toString() ?? '{}')
  const l = msg?.LayerChange?.new
  console.log('Received kanata message:', msg, l)
  if (l) {
    layer.value = l
  }
})

listen('ping-pong', (event) => {
  console.log('Received ping-pong message:', event)
})

invoke('start_kanata_listener')
</script>

<template>
  <template v-if="config.configFile">
    <KB :layer="layer" :layer-defs="layerDefs"></KB>
    <div @click="invoke('ping_pong')">{{ layer }}</div>
    <details>
      <summary>Settings</summary>
      <button @click="config.clearPaths()">clear</button>
      <button @click="queryConfigPaths()">change</button>
    </details>
    <details>
      <summary>Config file: {{ config.configFile }}</summary>
      <pre>{{ JSON.stringify(rawConfig, null, 2) }}</pre>
    </details>
    <details>
      <summary>Layer defs:</summary>
       <pre>{{ layerDefs }}</pre>
    </details>
  </template>
  <div v-else @click="queryConfigPaths()">
    Please select a configuration file:
    <button>Select configuration file</button>
  </div>
</template>

<style>
#key-slots .added {
  stroke: none;
}

#key-slots :not(.added) {
  stroke: transparent;
}

details {
  margin-top: 1em;
}
</style>
