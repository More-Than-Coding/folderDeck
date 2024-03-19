import { invoke } from '@tauri-apps/api/tauri'

/**
 * openFinder - open path in finder
 * @param {String} path - Path to open file or directory
 * @returns
 */
export const openFinder = async (path) =>
  await invoke('open_directory', { path })
