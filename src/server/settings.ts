import { invoke } from '@tauri-apps/api';
import { Settings } from '../types';

export function useSettings() {
  const load = async () => {
    try {
      return await invoke<Settings>('load_settings');
    } catch (e) {
      console.log(e);
    }
  };

  const save = async (options: Map<string, string>) => {
    try {
      await invoke('save_settings', { options });
    } catch (e) {
      console.log(e);
    }
  };

  return { load, save };
}
