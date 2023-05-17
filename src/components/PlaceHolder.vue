<script setup lang="ts">
interface IPlaceHolder {
  title?: string;
}

defineProps<IPlaceHolder>();
</script>

<template>
  <fieldset>
    <legend>{{ title }}</legend>
    <slot />
  </fieldset>
</template>

<style scoped>
fieldset {
  border: none;
  margin-bottom: 10px;
}
legend {
  font-size: 16px;
  font-weight: 500;
  font-style: italic;
  margin-bottom: 10px;
  color: white;
}
</style>
