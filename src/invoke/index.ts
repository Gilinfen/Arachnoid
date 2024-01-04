import { invoke } from '@tauri-apps/api'
import { InvokeArgs } from '@tauri-apps/api/tauri'

export const InvokeFn = [
  'execute_python_script',
  'init_python_path',
  'get_chrome_version_command',
  'download_chromedriver',
  'update_json_command',
  'read_json_command',
  'get_os_info',
  'use_verify_signature',
  'app_ready',
  'close_app',
] as const

export const tyInvoke = async <T>(
  cmd: (typeof InvokeFn)[number],
  args?: InvokeArgs
): Promise<T> => invoke<T>(cmd, args)
