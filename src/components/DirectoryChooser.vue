<script setup lang="ts">
import Button from './Button.vue';
import { open } from '@tauri-apps/api/dialog';

interface IDirectoryChooser {
  label?: string;
  name?: string;
}
defineProps<IDirectoryChooser>();

async function openFileChooser() {
  const selected = await open({
    directory: true,
    multiple: true,
  });
  if (Array.isArray(selected)) return;
  if (selected === null) return;
  else return;
}
</script>

<template>
  <Button
    :label="label"
    color="darker"
    larger
    rounded
    :name="name"
    @click="openFileChooser"
  />
</template>
