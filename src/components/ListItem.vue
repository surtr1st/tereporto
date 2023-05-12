<script setup lang="ts">
import { StyleValue } from 'vue';

interface IListItem {
  color?: string;
}
const { color } = defineProps<IListItem>();

function setBorderStyle() {
  if (!color) return '0px solid transparent';
  return `1px solid ${color}`;
}

const listItemStyle: StyleValue = {
  'border-left': setBorderStyle(),
};
</script>

<template>
  <li :style="{ ...listItemStyle }">
    <slot />
  </li>
</template>

<style scoped>
li {
  width: 470px;
  height: 109px;
  display: flex;
  justify-content: center;
  align-items: center;
  background: var(--light-neutral-gray);
  border-radius: 10px;
  margin-bottom: 0.8rem;
  transition: all 150ms;
}

li:hover {
  background: var(--darker-neutral-gray);
  cursor: pointer;
}
</style>
