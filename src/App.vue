<script setup lang="ts">
import KB from './kb/KB.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
//import { emit } from '@tauri-apps/api/event'
import { BaseDirectory, readTextFile } from '@tauri-apps/plugin-fs'

import { ref } from 'vue'
import { parse, type SExpression } from './sexpression'

// config
const configEntrypoint = 'test-conf/,,AULA.kbd'

// actual app data
const layer = ref('default')
const layerDefs = ref({} as Record<string, Record<string, SExpression>>)

// load config file and parse it as s-expression (lisp)
async function loadConfig(path: string) {
  const fileContent = await readTextFile("shared/ergonomics/kanata-companion/" +  path, { baseDir: BaseDirectory.Home })
  const fixedContent = fileContent.replace(/;;.*/g, '').replace(/(\s)([,'`])(\s)/g, '$1"$2"$3')
  console.log('Loaded config file:', fixedContent)
  const parsed = parse("(\n" + fixedContent + "\n)")

  // process includes
  let ientry = 0
  while (ientry < (parsed as any[]).length) {
    const entry = (parsed as any[])[ientry]
    if (Array.isArray(entry) && entry[0] === 'include') {
      const includePath = entry[1] as string
      const includedConfig = await loadConfig(path.replace(/\/[^\/]*$/, '/') + includePath)

      // merge included config into current parsed config
      for (const incEntry of includedConfig as any[]) {
        (parsed as any[]).splice(ientry + 1, 0, incEntry)
      }
      // remove the include entry
      (parsed as any[]).splice(ientry, 1)
    } else {
      ientry += 1
    }
  }
  console.log('Parsed config:', parsed)
  return parsed
}

function configToLayerDefs(parsed: any[]) {
  const defs: Record<string, Record<string, SExpression>> = {}
  for (const e of parsed) {
    if (Array.isArray(e) && e[0] === 'deflayermap') {
      const layerName = e[1] as string
      defs[layerName] = {}
      for (let i = 2; i < e.length; i+=2) {
        const key = e[i]
        const action = e[i+1]
        defs[layerName][key] = action
      }
    }
  }
  console.log('Layer definitions:', defs)
  layerDefs.value = defs
}

loadConfig(configEntrypoint).then(configToLayerDefs)

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
  <KB :layer="layer" :layer-defs="layerDefs"></KB>
  <div @click="invoke('ping_pong')">{{ layer }}</div>
</template>

<style>
#key-slots .added {
  stroke: none;
}
#key-slots :not(.added) {
  stroke: transparent;
}
</style>
