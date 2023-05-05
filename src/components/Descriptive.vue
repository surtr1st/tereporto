<script setup lang="ts">
interface IDescriptive {
  title?: string;
  description?: string;
}
defineProps<IDescriptive>();
</script>

<template>
  <span class="tp__descriptive">
    <h3>{{ title }}</h3>
    <p>"{{ description }}"</p>
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
  color: #7b7b7b;
  font-size: 13px;
  background: inherit;
}
</style>
