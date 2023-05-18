import { invoke } from '@tauri-apps/api';
import { Settings } from '../types';

export function useSettings() {
  const load = async () => await invoke<Settings>('load_settings');

  const save = async (options: Map<string, string>) =>
    await invoke<string>('save_settings', { options });

  return { load, save };
}
