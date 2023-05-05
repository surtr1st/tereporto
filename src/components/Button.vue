<script setup lang="ts">
import { ref } from 'vue';

interface IButton {
  name?: string;
  label?: string;
  color?: 'neutral' | 'darker' | 'danger';
  rounded?: boolean;
  onClick?: () => void | Promise<void>;
}
const { rounded, color } = defineProps<IButton>();
const className = ref('');
if (color === 'darker') className.value += 'tp__button--darker ';
if (color === 'neutral') className.value += 'tp__button--neutral ';
if (color === 'danger') className.value += 'tp__button--danger ';
if (rounded) className.value += ' tp__button--rounded';
</script>

<template>
  <button
    v-if="color === 'darker'"
    :class="className"
    :name="name"
    @click="onClick"
  >
    <slot />
    {{ label }}
  </button>
  <button
    v-else-if="color === 'neutral'"
    :class="className"
    :name="name"
    @click="onClick"
  >
    <slot />
    {{ label }}
  </button>
  <button
    v-else
    :class="className"
    :name="name"
    @click="onClick"
  >
    <slot />
    {{ label }}
  </button>
</template>

<style scoped>
button {
  min-width: 38px;
  max-width: 122px;
  min-height: 38px;
  max-height: 51px;
  border: none;
  margin: 0.1rem;
}
button:hover {
  cursor: pointer;
  transition: all 250ms;
}
.tp__button--rounded {
  border-radius: 10px;
}
.tp__button--darker {
  background: var(--darker-color);
  color: white;
}
.tp__button--darker:hover {
  background: var(--light-neutral-gray);
}
.tp__button--neutral {
  background: var(--neutral-gray);
  color: white;
}
.tp__button--neutral:hover {
  background: var(--light-neutral-gray);
}
.tp__button--danger {
  background: var(--danger-color);
  color: black;
}
.tp__button--danger:hover {
  background: var(--light-danger-color);
}
</style>
