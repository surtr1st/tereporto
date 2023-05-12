<script setup lang="ts">
interface IGrid {
  type: 'default' | 'panel';
}
defineProps<IGrid>();
</script>

<template>
  <div
    v-if="type === 'default'"
    class="tp__grid"
  >
    <slot />
  </div>
  <div
    v-if="type === 'panel'"
    class="tp__grid-panel"
  >
    <slot />
  </div>
</template>
<style scoped>
.tp__grid {
  display: grid;
  grid-template-areas:
    'lside lheader lheader rheader rheader rside'
    'lside main main aside aside rside'
    'lside footer footer footer footer rside';
  gap: 10px;
  padding: 10px;
}

.tp__grid-panel {
  display: grid;
  grid-template-areas:
    'lside main aside rside'
    'lside main aside rside';
  gap: 10px;
  padding: 10px;
}
</style>
