import { invoke } from '@tauri-apps/api';
import { Storage, StorageResponse, StorageUpdateArgs } from '../types';

export function useStorage() {
  const getStorages = async () =>
    await invoke<StorageResponse[]>('get_storages');
  const createStorage = async (storages: Storage[]) =>
    await invoke<string>('create_storage', { storages });

  const updateStorage = async (args: StorageUpdateArgs) =>
    await invoke<string>('update_storage', { ...args });

  const removeStorage = async (filename: string) =>
    await invoke<string>('remove_storage', { filename });

  return {
    getStorages,
    createStorage,
    updateStorage,
    removeStorage,
  };
}
