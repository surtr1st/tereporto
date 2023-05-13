import { invoke } from '@tauri-apps/api';

export function useDirectoryControl() {
  const openSelectedDir = async (dir: string) => {
    try {
      await invoke('open_selected_directory', { dir });
    } catch (e) {
      console.log(e);
    }
  };

  return {
    openSelectedDir,
  };
}
