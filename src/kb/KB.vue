<script setup lang="ts">
import keyboard from './iso-azerty.svg?raw'

import { onMounted, ref, computed, watch } from 'vue';

const props = defineProps<{
  layer: string
}>()

const wrapper = ref<HTMLDivElement | null>(null);

const customKeys = {
  // arrows
  'left': '⇠',
  'up': '⇡',
  'down': '⇣',
  'right': '⇢',
  // mouse motion
  ':mouseleft': ['⇠', 'cyan'],
  ':mouseup': ['⇡', 'cyan'],
  ':mousedown': ['⇣', 'cyan'],
  ':mouseright': ['⇢', 'cyan'],
  ':alttabtab': ['⎇⇤', 'orange'],
  ':alttab': ['⎇⇢', 'orange'],
  ':deskopleft': ['🖥⇠', 'orange'],
  ':deskopright': ['🖥⇢', 'orange'],
  'tab': '↹',
  'enter': '↵',
  'backspace': '←',
  'S-del': 'cut',
  'S-ins': 'paste',
  'C-ins': 'copy',
} as Record<string, string | [string, string]>;

const patchDescriptor = computed(() => {
  const l = props.layer
  return l === 'default' ? {}
  : l === 'singlehandmode' ? {
    // mouse keys
    's': ':deskopleft',
    'e': ':alttabtab',
    'd': ':alttab',
    'f': ':deskopright',
    'x': 'S-del',
    'c': 'C-ins',
    'v': 'S-ins',
    't': 'backspace',
    'g': 'tab',
  }
  : l === 'mouse' ? {
    // mouse keys
    's': ':mouseleft',
    'e': ':mouseup',
    'd': ':mousedown',
    'f': ':mouseright',
  }
  : l === 'fuckeys' ? {
    // function keys
    'f': 'F1',
    'd': 'F2',
    's': 'F3',
    'r': 'F4',
    'e': 'F5',
    'w': 'F6',
    'v': 'F7',
    'c': 'F8',
    'x': 'F9',
    'a': 'F10',
    'q': 'F11',
    'z': 'F12',
  }
  : l === 'main←' ? {
    // arrows
    'j': 'left',
    'i': 'up',
    'k': 'down',
    'l': 'right',
    'u': 'PgUp',
    'o': 'PgDn',
    'y': 'backspace',
    'h': 'enter',
    'n': 'tab',
  }
  : {}
})
const patch = computed(() => Object.fromEntries(Object.entries(patchDescriptor.value).map(
  ([k, v]) => {
    if (typeof v === 'string' && v in customKeys) {
      v = customKeys[v]
    }
    if (!Array.isArray(v)) {
      v = [v]
    }
    if (v.length < 2) {
      v.push('darkred')
    }
    return [k, v, "30"]
  }
)))

function attr<T>(el: Element, name: string, defaultValue: string, transform?: (v: string) => T): T | string {
  const v = el.getAttribute(name)
  if (v === null) return defaultValue
  if (!transform) return v
  return transform(v)

}

function refreshModifications() {
  const k = wrapper.value;
  if (!k) return;
  k.querySelectorAll('.added').forEach(e => e.remove())
  for (const [key, v] of Object.entries<[string, string]>(patch.value)) {
    const el = k.querySelector<SVGElement>(`#k${key}`)
    if (el) {
      // insert text after the box, same location
      const text = document.createElementNS("http://www.w3.org/2000/svg", "text")
      text.setAttribute("x", attr(el, "x", "0"))
      text.setAttribute("y", attr(el, "y", "0"))
      text.setAttribute("width", attr(el, "width", "0"))
      text.setAttribute("height", attr(el, "height", "0"))
      text.setAttribute("dx", attr(el, "width", "0", v => (parseFloat(v) * 2 / 3).toString()))
      text.setAttribute("dy", attr(el, "height", "0", v => (parseFloat(v) * 3 / 4).toString()))
      text.setAttribute("text-anchor", "middle")
      text.setAttribute("font-size", "30")
      text.textContent = v[0]
      text.setAttribute("fill", v[1])
      if (v[0].length > 2) {
        text.setAttribute("font-size", "15")
        text.setAttribute("dx", attr(el, "width", "0", v => (parseFloat(v) * 1 / 2).toString()))
      } else if (v[0].length > 1) {
        text.setAttribute("font-size", "22")
        text.setAttribute("dx", attr(el, "width", "0", v => (parseFloat(v) * 1 / 2).toString()))
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
</template>
