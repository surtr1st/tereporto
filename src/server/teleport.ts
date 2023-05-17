import { invoke } from '@tauri-apps/api';
import { Teleport, TeleportResponse, TeleportUpdateArgs } from '../types';

export function useTeleport() {
  const getTeleports = async () => {
    try {
      return await invoke<TeleportResponse[]>('get_teleports');
    } catch (e) {
      console.log(e);
    }
  };

  const createTeleport = async (teleports: Teleport[]) => {
    try {
      const result = await invoke<string>('create_teleport', { teleports });
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

  const removeTeleport = async (filename: string) => {
    try {
      const result = await invoke<string>('remove_teleport', { filename });
      console.log(result);
    } catch (e) {
      console.log(e);
    }
  };

  return {
    getTeleports,
    createTeleport,
    updateTeleport,
    removeTeleport,
  };
}
