<script setup lang="ts">
import { isArray } from 'mathjs'
import keyboard from './iso-azerty.svg?raw'

import { onMounted, ref, computed, watch } from 'vue'

const props = defineProps<{
  layer: string
  layerDefs: Record<string, Record<string, string>>
}>()

const wrapper = ref<HTMLDivElement | null>(null)

type KeyRenderDescriptor = [string, string, Record<string, string>?]

const customKeys = {
  // arrows
  left: '⇠',
  up: '⇡',
  down: '⇣',
  rght: '⇢',
  // mouse motion
  ':mouseleft': ['⇠', 'cyan', { bg: '#00007FAA' }],
  ':mouseup': ['⇡', 'cyan', { bg: '#00007FAA' }],
  ':mousedown': ['⇣', 'cyan', { bg: '#00007FAA' }],
  ':mouseright': ['⇢', 'cyan', { bg: '#00007FAA' }],
  ':wheelleft': ['…', 'cyan', { bg: '#00007FAA' }],
  ':wheelup': ['…', 'cyan', { bg: '#00007FAA' }],
  ':wheeldown': ['…', 'cyan', { bg: '#00007FAA' }],
  ':wheelright': ['…', 'cyan', { bg: '#00007FAA' }],
  // custom look for shortcuts
  'A-S-tab': ['⎇⇤', 'darkorange'],
  'A-tab': ['⎇⇢', 'darkorange'],
  'A-C-left': ['⇠', 'darkorange'], //['🖥⇠', 'darkorange'],
  'A-C-right': ['⇢', 'darkorange'], //['🖥⇢', 'darkorange'],
  tab: '↹',
  enter: '↵',
  backspace: '←',
  'S-del': 'cut',
  'S-ins': 'paste',
  'C-ins': 'copy',
} as Record<string, string | KeyRenderDescriptor>

const patchDescriptor = computed(() => props.layerDefs[props.layer] || {})

const patch = computed<Record<string, KeyRenderDescriptor>>(() =>
  Object.fromEntries(
    Object.entries(patchDescriptor.value).map(([k, vv]) => {
      let v = vv as string | string[] | KeyRenderDescriptor
      if (typeof v === 'string' && v in customKeys) {
        v = customKeys[v]!
      }
      if (typeof v === 'string') {
        v = [v]
      }
      if (v.length < 2) {
        v.push('darkred')
      }
      return [k, v as KeyRenderDescriptor]
    }),
  ),
)

function attr<T>(
  el: Element,
  name: string,
  defaultValue: string,
  transform?: (v: string) => T,
): T | string {
  const v = el.getAttribute(name)
  if (v === null) return defaultValue
  if (!transform) return v
  return transform(v)
}
function attrFloat(el: Element, name: string, defaultValue: number): number {
  const v = el.getAttribute(name)
  if (v === null) return defaultValue
  return parseFloat(v)
}

const keyToIdPatch: Record<string, string> = {
  '`': 'kbackquote',
  '-': 'kdash',
  '=': 'kequals',
  '[': 'klbracket',
  ']': 'krbracket',
  ';': 'ksemicolon',
  "'": 'kquote',
  '.': 'kperiod',
  '/': 'kslash',
  '\\': 'kbackslash',
}
function keyToId(k: string) {
  if (k in keyToIdPatch) {
    return keyToIdPatch[k]!
  }
  if (k.includes(',')) {
    return 'NONE'
  }
  return 'k' + k
}

function valueToValue(v: [string, string, Record<string, string>?]): string[] {
  if (v[0] === 'tap-hold-release') {
    console.log('v4', v[4], typeof v[4])
    if (v[4] == 'XX') {
      v = ['#', 'lightgray']
    } else if (typeof v[4] === 'string') {
      v = ['⭘', 'green']
    } else {
      v = ['⭘', 'purple']
    }
  }
  if (v[0]?.startsWith('movemouse-accel-')) {
    v = customKeys[':mouse' + v[0].replace(/.*-/, '')] as string[]
  }
  if (v[0]?.startsWith('mwheel-')) {
    v = customKeys[':wheel' + v[0].replace(/.*-/, '')] as string[]
  }
  return v
}

function refreshModifications() {
  const k = wrapper.value
  if (!k) return
  k.querySelectorAll('.added').forEach((e) => e.remove())
  for (const [key, vraw] of Object.entries<[string, string, Record<string, string>?]>(patch.value)) {
    const el = k.querySelector<SVGElement>(`#${keyToId(key)}`)
    const v = valueToValue(vraw)
    if (el && isArray(v) && v.length >= 2 && v[0] !== undefined) {
      const x = attrFloat(el, 'x', 0)
      const y = attrFloat(el, 'y', 0)
      const w = attrFloat(el, 'width', 0)
      const h = attrFloat(el, 'height', 0)
      // possibly add background
      if (v[2]?.bg) {
        const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect')
        rect.setAttribute('x', x.toString())
        rect.setAttribute('y', y.toString())
        rect.setAttribute('width', w.toString())
        rect.setAttribute('height', h.toString())
        rect.setAttribute('fill', v[2].bg)
        rect.classList.add('added')
        el.parentNode?.insertBefore(rect, el)
      }
      // insert text after the box, same location
      const text = document.createElementNS('http://www.w3.org/2000/svg', 'text')
      text.setAttribute('x', x.toString())
      text.setAttribute('y', y.toString())
      text.setAttribute('width', w.toString())
      text.setAttribute('height', h.toString())
      text.setAttribute('dx', ((w * 2) / 3).toString())
      text.setAttribute('dy', ((h * 3) / 4).toString())
      text.setAttribute('text-anchor', 'middle')
      text.setAttribute('font-size', '30')
      text.textContent = v[0]
      text.setAttribute('fill', v[1]!)
      if (v[0].length > 2) { // reduce font size for long texts
        text.setAttribute('font-size', '15')
        text.setAttribute(
          'dx',
          attr(el, 'width', '0', (v) => ((parseFloat(v) * 1) / 2).toString()),
        )
      } else if (v[0].length > 1) {
        text.setAttribute('font-size', '22')
        text.setAttribute(
          'dx',
          attr(el, 'width', '0', (v) => ((parseFloat(v) * 1) / 2).toString()),
        )
      }
      for (const [kstyle, vstyle] of Object.entries(v[2] ?? {})) {
        text.style.setProperty(kstyle, vstyle)
      }
      text.classList.add('added')
      el.parentNode?.appendChild(text)
    }
  }
}

onMounted(refreshModifications)
watch(patch, refreshModifications)
</script>
<template>
  <div ref="wrapper" class="keyboard" v-html="keyboard"></div>
  <h2>{{ layer }}</h2>
  <pre>{{ JSON.stringify(layerDefs?.[layer], null, 2) }}</pre>
</template>
