<script setup lang="ts">
import type { SExpression } from '@/sexpression'
import keyboard from './iso-azerty.svg?raw'

import { onMounted, ref, computed, watch } from 'vue'

type KeyRenderDescriptor = [string, string, Record<string, string>?]

const props = defineProps<{
  layer: string
  layerDefs: Record<string, Record<string, SExpression>>
}>()

const wrapper = ref<HTMLDivElement | null>(null)

// some custom key renderings, and also pseudo-keys for special actions
type KeyRenderDescriptorPartial = string | KeyRenderDescriptor
function mouse(label: string, more: Record<string, string> = {}) {
  return [label, 'cyan', { bg: '#00007FAA', ...more }] as KeyRenderDescriptor
}
const customKeys: Record<string, KeyRenderDescriptorPartial> = {
  // arrows
  left: '⬅',
  up: '⬆',
  down: '⬇',
  rght: '➡',
  // mouse motion
  ':mouseleft': mouse('⬅'),
  ':mouseup': mouse('⬆'),
  ':mousedown': mouse('⬇'),
  ':mouseright': mouse('➡'),
  ':wheelleft': mouse('⇜'),
  ':wheelup': mouse('⇜', { rotate: '90' }),
  ':wheeldown': mouse('⇝', { rotate: '90' }),
  ':wheelright': mouse('⇝'),
  // custom look for shortcuts
  'A-S-tab': ['⎇⇤', 'darkorange'],
  'A-tab': ['⎇⇢', 'darkorange'],
  'A-C-left': ['⭅', 'darkorange'], //['🖥⇠', 'darkorange'],
  'A-C-right': ['⭆', 'darkorange'], //['🖥⇢', 'darkorange'],
  tab: '↹',
  enter: '↵',
  backspace: '←',
  'S-del': 'cut',
  'S-ins': 'paste',
  'C-ins': 'copy',
  mlft: mouse('🖯'),
  mrgt: mouse('🖰'),
  mmid: mouse('🖱'),
  kp1: '1',
  kp2: '2',
  kp3: '3',
  kp4: '4',
  kp5: '5',
  kp6: '6',
  kp7: '7',
  kp8: '8',
  kp9: '9',
  kp0: '0',
  'kp-': '-',
  'kp+': '+',
  'kp*': '*',
  'kp/': '/',
  'kp.': '.',
}
function getCustomKey(k: string): KeyRenderDescriptorPartial {
  const v = customKeys[k]
  if (v === undefined) {
    throw new Error('Unknown custom key: ' + k)
  }
  if (typeof v === 'string') {
    return [v, 'darkred']
  }
  return v
}

// utilities
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
function isString(v: string | SExpression | undefined): v is string {
  return typeof v === 'string'
}

// map kanata key id to svg element id
const keyToIdPatch: Record<string, string> = {
  '`': 'kbackquote',
  '-': 'kdash',
  '=': 'kequals',
  '[': 'klbracket',
  ']': 'krbracket',
  ';': 'ksemicolon',
  ',': 'klt',
  '.': 'kgt',
  '/': 'kslash',
  '\\': 'kbackslash',
  '\'': 'kquote',
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

// Transform an SExpression value (as parsed) into a KeyRenderDescriptor (for rendering)
const BASEMODS = Object.fromEntries('sft ctl alt'.split(' ').map((m) => [[`r${m}`, `${m}`], [`l${m}`,  `${m}`]]).flat())
BASEMODS['ralt'] = 'altgr'
BASEMODS['lmet'] = 'super'
BASEMODS['rmet'] = 'super'
function valueToValue(vv: SExpression): KeyRenderDescriptor {
  let v: KeyRenderDescriptorPartial | SExpression = vv
  if (typeof v === 'string' && v in customKeys) {
    v = getCustomKey(v)
  }
  if (v[0] === 'multi' && typeof v[1] === 'string' && v[1] in customKeys) {
    console.log('Processing multi custom key', JSON.stringify(vv, null, 2))
    v = getCustomKey(v[1])
  }
  if (typeof v === 'string') {
    v = [v]
  }
  if (v.length < 2) {
    v.push('darkred')
  }
  if (isString(v[0]) && v[0].startsWith('movemouse-accel-')) {
    v = getCustomKey(':mouse' + v[0].replace(/.*-/, '')) as string[]
  }
  if (isString(v[0]) && v[0].startsWith('mwheel-')) {
    v = getCustomKey(':wheel' + v[0].replace(/.*-/, '')) as string[]
  }
  function handle(vv: string | SExpression | Record<string, string>): KeyRenderDescriptor | undefined {
    if (vv === 'XX') {
        return ['#', 'lightgray']
    } else if (typeof vv === 'string') {
      if (vv in BASEMODS) {
        return [BASEMODS[vv], 'green']
      } else {
        return ['⭘', 'green']
      }
    } else if (Array.isArray(vv)) {
      if (vv[0] === 'layer-while-held') {
        return [(vv[1] ?? '?') as string, 'purple']
      } else if (vv[0] === 'multi') {
        for (const part of vv.slice(1)) {
          const newv = handle(part)
          if (newv !== undefined) {
            return newv
          }
        }
        return undefined
      } else {
        return ['⭘', 'purple']
      }
    } else {
      return undefined
    }
  }
  if (v[0] === 'tap-hold-release' && v[4] !== undefined) {
    const newv = handle(v[4])
    if (newv !== undefined) {
      v = newv
    } else {
      v = ['⭘', 'red']
    }
  }
  return v as KeyRenderDescriptor
}

// The actual keyboard rendering patch (to patch the svg)
const patchDescriptor = computed(() => props.layerDefs[props.layer] || {})
const patch = computed<Record<string, KeyRenderDescriptor>>(() =>
  Object.fromEntries(
    Object.entries<SExpression>(patchDescriptor.value).map(([k, vv]) => {
      const v = valueToValue(vv)
      return [k, v]
    }),
  ),
)

function refreshModifications() {
  const k = wrapper.value
  if (!k) return
  k.querySelectorAll('.added').forEach((e) => e.remove())
  for (const [key, v] of Object.entries<KeyRenderDescriptor>(patch.value)) {
    const el = k.querySelector<SVGElement>(`#${keyToId(key)}`)
    if (el) {
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
      if (v[2]?.rotate) {
        text.setAttribute('transform', `rotate(${v[2].rotate}, ${x + w / 2}, ${y + h / 2})`)
      }
      let s = 22
      const getDx = (el: SVGElement) => attr(el, 'width', '0', (v) => ((parseFloat(v) * 1) / 2).toString())
      const l = v[0].length
      if (l > 8) {
        s = 5
      } else if (l > 5) {
        s = 10
      } else if (l > 2) {
        s = 14
      } else if (l > 1) {
        s = 22
      }
      text.setAttribute('font-size', `${s}`)
      text.setAttribute('dx', getDx(el))
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
  <details>
    <summary>Current layer patch: {{ layer }}</summary>
    <pre>{{ JSON.stringify(patch, null, 2) }}</pre>
  </details>
  <hr plate />
</template>
