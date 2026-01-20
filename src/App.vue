<script setup lang="ts">
import MainView from './MainView.vue'
</script>

<template>
  <Suspense>
    <MainView />
    <template #fallback>
      <div>
        Not configured yet...
      </div>
    </template>
  </Suspense>
</template>
