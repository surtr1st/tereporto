<script setup lang="ts">
import { removeQuotes } from '../helpers';
interface IDescriptive {
  title?: string;
  description?: string | string[];
  onAction?: () => void | Promise<void>;
}
defineProps<IDescriptive>();
</script>

<template>
  <span class="tp__descriptive">
    <h3>{{ title }}</h3>
    <p
      class="tp__link"
      @click="onAction"
    >
      "{{
        Array.isArray(description) && Array.of(description).length > 1
          ? description.join('...')
          : removeQuotes(Array.of(description).join(''))
      }}"
    </p>
    <slot />
  </span>
</template>

<style scoped>
.tp__descriptive {
  width: 65%;
  padding: 1.2rem;
  background: inherit;
}
.tp__descriptive > h3 {
  background: inherit;
}

.tp__descriptive > p {
  font-style: italic;
  color: #884452;
  font-size: 13px;
  background: inherit;
}

.tp__descriptive > p:hover {
  cursor: pointer;
}
.tp__link {
  background: inherit;
  border-radius: 5px;
  padding: 0.1rem;
}
.tp__link:hover {
  background: var(--neutral-gray);
  transition: ease-in 150ms;
  transition-delay: 100ms;
}
</style>
