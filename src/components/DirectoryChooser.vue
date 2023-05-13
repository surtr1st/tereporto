<script setup lang="ts">
import Button from './Button.vue';
import { open } from '@tauri-apps/api/dialog';

interface IDirectoryChooser {
  label?: string;
  name?: string;
  onSelect?: () => void | Promise<void>;
}
defineProps<IDirectoryChooser>();
const emit = defineEmits(['update:select']);

async function openFileChooser() {
  console.log(true);
  const selected = await open({
    directory: true,
    multiple: true,
  });
  if (selected === null) return;
  if (Array.isArray(selected) && selected.length > 1)
    emit('update:select', selected);
  if (selected.length === 1) emit('update:select', selected[0]);
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
