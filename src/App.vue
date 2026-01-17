<script setup lang="ts">
import KB from './kb/KB.vue'
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
//import { emit } from '@tauri-apps/api/event'

import { ref } from 'vue';

const layer = ref('default');

listen('kanata-message', (event) => {
  console.log("Received kanata message:", event);
  const msg = JSON.parse(event.payload ?? '{}');
  const l = msg?.LayerChange?.new
  console.log("Layer change:", l)
  if (l) {
    layer.value = l
  }
});

listen('ping-pong', (event) => {
  console.log("Received ping-pong message:", event);
});

invoke('start_kanata_listener')

</script>

<template>
  <KB :layer="'singlehandmode'"></KB>
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
