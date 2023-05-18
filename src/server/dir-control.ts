import { invoke } from '@tauri-apps/api';

export function useDirectoryControl() {
  const openSelectedDir = async (dir: string) =>
    await invoke('open_selected_directory', { dir });

  return {
    openSelectedDir,
  };
}
