import { invoke } from '@tauri-apps/api';
import { Storage, StorageResponse, StorageUpdateArgs } from '../types';

export function useStorage() {
  const getStorages = async () => {
    try {
      return await invoke<StorageResponse[]>('get_storages');
    } catch (e) {
      console.log(e);
    }
  };
  const createStorage = async (s: Storage) => {
    try {
      const result = await invoke<string>('create_storage', { s });
      console.log(result);
    } catch (e) {
      console.log(e);
    }
  };

  const updateStorage = async (args: StorageUpdateArgs) => {
    try {
      const result = await invoke<string>('update_storage', { ...args });
      console.log(result);
    } catch (e) {
      console.log(e);
    }
  };

  return {
    getStorages,
    createStorage,
    updateStorage,
  };
}
