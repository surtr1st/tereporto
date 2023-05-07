<script setup lang="ts">
import { ref } from 'vue';

enum TitleHeaderPosition {
  LEFT = 'left',
  RIGHT = 'right',
  CENTER = 'center',
  TOP = 'top',
  BOTTOM = 'bottom',
}
interface ITitleHeader {
  title?: string;
  textPosition?: 'left' | 'right' | 'center' | 'top' | 'bottom';
}
const { textPosition } = defineProps<ITitleHeader>();

const className = ref<string>('tp__title-header');
switch (textPosition) {
  case TitleHeaderPosition.CENTER:
    className.value += ' tp__title-header--center';
    break;
  case TitleHeaderPosition.LEFT:
    className.value += ' tp__title-header--left';
    break;
  case TitleHeaderPosition.RIGHT:
    className.value += ' tp__title-header--right';
    break;
  case TitleHeaderPosition.BOTTOM:
    className.value += ' tp__title-header--bottom';
    break;
  case TitleHeaderPosition.TOP:
    className.value += ' tp__title-header--top';
    break;
}
</script>

<template>
  <span :class="className">
    <h2>{{ title }}</h2>
  </span>
</template>

<style scoped>
.tp__title-header {
  height: 72px;
  background: inherit;
  color: white;
  width: 300px;
  display: flex;
  align-items: center;
}
.tp__title-header--left {
  justify-content: flex-start;
}
.tp__title-header--right {
  justify-content: flex-end;
}
.tp__title-header--center {
  justify-content: center;
}
</style>
