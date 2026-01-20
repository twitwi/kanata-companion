import { useConfig } from '@/stores/config'
import { open } from '@tauri-apps/plugin-dialog'

export async function queryConfigPaths() {
  const config = useConfig()
  const configFile = await open({
    multiple: false,
    directory: false,
    defaultPath: config.suggestedFilePickerPath,
  })
  if (configFile) {
    config.configFile = configFile
  }
}
