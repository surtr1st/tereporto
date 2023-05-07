<script setup lang="ts">
import { ref } from 'vue';

enum GridPosition {
  HEADER = 'header',
  LEFT_HEADER = 'lheader',
  RIGHT_HEADER = 'rheader',
  MAIN = 'main',
  ASIDE = 'aside',
  FOOTER = 'footer',
}
interface IGridItem {
  position?: 'header' | 'lheader' | 'rheader' | 'main' | 'aside' | 'footer';
}
const { position } = defineProps<IGridItem>();

const className = ref<string>('');
switch (position) {
  case GridPosition.HEADER:
    className.value = 'tp__grid-header';
    break;
  case GridPosition.LEFT_HEADER:
    className.value = 'tp__grid-left-header';
    break;
  case GridPosition.RIGHT_HEADER:
    className.value = 'tp__grid-right-header';
    break;
  case GridPosition.MAIN:
    className.value = 'tp__grid-main';
    break;
  case GridPosition.ASIDE:
    className.value = 'tp__grid-aside';
    break;
  case GridPosition.FOOTER:
    className.value = 'tp__grid-footer';
    break;
}
</script>

<template>
  <div :class="className">
    <slot />
  </div>
</template>
<style scoped>
.tp__grid-left-header {
  grid-area: lheader;
}
.tp__grid-right-header {
  grid-area: rheader;
}
.tp__grid-main {
  grid-area: main;
}
.tp__grid-aside {
  grid-area: aside;
}
.tp__grid-footer {
  grid-area: footer;
}
</style>
