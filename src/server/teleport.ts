import { invoke } from '@tauri-apps/api';
import { Teleport, TeleportResponse, TeleportUpdateArgs } from '../types';

export function useTeleport() {
  const getTeleports = async () => {
    try {
      return await invoke<TeleportResponse>('get_teleports');
    } catch (e) {
      console.log(e);
    }
  };

  const createTeleport = async (t: Teleport) => {
    try {
      const result = await invoke<string>('create_teleport', { t });
      console.log(result);
    } catch (e) {
      console.log(e);
    }
  };

  const updateTeleport = async (args: TeleportUpdateArgs) => {
    try {
      const result = await invoke<string>('update_teleport', { ...args });
      console.log(result);
    } catch (e) {
      console.log(e);
    }
  };

  return {
    getTeleports,
    createTeleport,
    updateTeleport,
  };
}
