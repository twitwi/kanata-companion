import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useConfig = defineStore('settings', () => {
  const configFile = ref('')
  const previousConfigFile = ref<string | null>(null)
  function clearPaths() {
    if (configFile.value) {
      previousConfigFile.value = configFile.value
    }
    configFile.value = ''
  }
  const suggestedFilePickerPath = computed(() => {
    return configFile.value || previousConfigFile.value || undefined
  })
  return {
    configFile,
    previousConfigFile,
    suggestedFilePickerPath,
    clearPaths,
  }
})
