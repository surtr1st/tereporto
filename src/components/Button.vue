<script setup lang="ts">
import { ref } from 'vue';

interface IButton {
  name?: string;
  label?: string;
  color?: 'neutral' | 'darker' | 'danger';
  rounded?: boolean;
  larger?: boolean;
  icon?: boolean;
  onClick?: () => void | string | Promise<void | string>;
  disabled?: boolean;
}
const { label, larger, rounded, color, icon } = defineProps<IButton>();

const className = ref<string>('tp__button');
if (color === 'darker') className.value += ' tp__button--darker';
if (color === 'neutral') className.value += ' tp__button--neutral';
if (color === 'danger') className.value += ' tp__button--danger';
if (rounded) className.value += ' tp__button--rounded';
if (larger) className.value += ' tp__button--larger';
if (icon) className.value += ' tp__button-icon';
if (!label) className.value += ' tp__button-no-label';
</script>

<template>
  <button
    v-if="color === 'darker'"
    :class="className"
    :name="name"
    :aria-label="name"
    :title="label"
    @click="onClick"
    :disabled="disabled"
  >
    <slot />
    {{ label }}
  </button>
  <button
    v-else-if="color === 'neutral'"
    :class="className"
    :name="name"
    :aria-label="name"
    :title="label"
    @click="onClick"
    :disabled="disabled"
  >
    <slot />
    {{ label }}
  </button>
  <button
    v-else
    :class="className"
    :name="name"
    :aria-label="name"
    :title="label"
    @click="onClick"
    :disabled="disabled"
  >
    <slot />
    {{ label }}
  </button>
</template>

<style scoped>
.tp__button {
  min-width: 38px;
  min-height: 38px;
  border: none;
  margin: 0.1rem;
  transition: all 250ms;
  font-weight: 500;
  padding-left: 0.5rem;
  padding-right: 0.5rem;
}
.tp__button-no-label {
  width: 51px;
  height: 51px;
}
.tp__button-icon {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 3px;
}
.tp__button--larger {
  min-width: 125px;
  width: auto;
  max-width: 170px;
  height: 51px;
}
.tp__button:hover {
  cursor: pointer;
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

.tp__button--darker:disabled {
  background: #000000;
  cursor: not-allowed;
}

.tp__button--neutral {
  background: var(--neutral-gray);
  color: white;
}

.tp__button--neutral:hover {
  background: var(--light-neutral-gray);
}

.tp__button--neutral:disabled {
  background: #000000;
  cursor: not-allowed;
}

.tp__button--danger {
  background: var(--danger-color);
  color: black;
}

.tp__button--danger:hover {
  background: var(--light-danger-color);
}

.tp__button--danger:disabled {
  background: #000000;
  cursor: not-allowed;
}
</style>
