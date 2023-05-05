<script setup lang="ts">
interface IFlex {
  justifyContent?: 'space-around' | 'space-between' | 'space-evenly' | 'center';
  alignItems?: 'center';
  gap?: number;
}
const { justifyContent, alignItems, gap } = defineProps<IFlex>();
const flexStyle = {
  'justify-content': justifyContent,
  'align-items': alignItems,
  gap: `${gap}px`,
};
</script>

<template>
  <div
    class="tp__flex"
    :style="flexStyle"
  >
    <slot />
  </div>
</template>

<style scoped>
.tp__flex {
  display: flex;
}
</style>
