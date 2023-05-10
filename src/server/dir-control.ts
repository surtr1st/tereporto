import { invoke } from '@tauri-apps/api';

export function useDirectoryControl() {
  const openSelectedDir = async (directory: string) => {
    try {
      await invoke('open_selected_directory', { directory });
    } catch (e) {
      console.log(e);
    }
  };

  return {
    openSelectedDir,
  };
}
