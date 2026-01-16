<script setup lang="ts">
import KB from './kb/KB.vue'
</script>

<template>
  <h1>Don't loose your mind!!!!!</h1>
  <KB :layer="'singlehandmode'"></KB>
</template>

<style>
#key-slots .added {
  stroke: none;
}
#key-slots :not(.added) {
  stroke: transparent;
}
</style>
