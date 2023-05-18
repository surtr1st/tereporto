import { invoke } from '@tauri-apps/api';
import { Teleport, TeleportResponse, TeleportUpdateArgs } from '../types';

export function useTeleport() {
  const getTeleports = async () =>
    await invoke<TeleportResponse[]>('get_teleports');

  const createTeleport = async (teleports: Teleport[]) =>
    await invoke<string>('create_teleport', { teleports });

  const updateTeleport = async (args: TeleportUpdateArgs) =>
    await invoke<string>('update_teleport', { ...args });

  const removeTeleport = async (filename: string) =>
    await invoke<string>('remove_teleport', { filename });

  return {
    getTeleports,
    createTeleport,
    updateTeleport,
    removeTeleport,
  };
}
