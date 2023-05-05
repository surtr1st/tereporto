<script setup lang="ts">
interface ITitleHeader {
  title?: string;
}
defineProps<ITitleHeader>();
</script>

<template>
  <span class="tp__title-header">
    <h2>{{ title }}</h2>
  </span>
</template>

<style scoped>
.tp__title-header {
  display: grid;
  place-items: center;
  height: 72px;
  background: inherit;
  color: white;
  width: 300px;
}
</style>
