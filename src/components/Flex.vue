<script setup lang="ts">
import { StyleValue } from 'vue';

interface IFlex {
  justifyContent?:
    | 'space-around'
    | 'space-between'
    | 'space-evenly'
    | 'center'
    | 'flex-end'
    | 'flex-start';
  alignItems?: 'center' | 'flex-start' | 'flex-end';
  gap?: number;
  column?: boolean;
  marginTop?: number;
  marginBottom?: number;
  width?: number;
}
const {
  justifyContent,
  alignItems,
  gap,
  column,
  marginTop,
  marginBottom,
  width,
} = defineProps<IFlex>();

const flexStyle: StyleValue = {
  'justify-content': justifyContent ?? 'center',
  'align-items': alignItems ?? 'center',
  'flex-direction': column ? 'column' : 'row',
  gap: `${gap}px`,
  'margin-top': `${marginTop}px`,
  'margin-bottom': `${marginBottom}px`,
  width: `${width}px`,
};
</script>

<template>
  <div
    class="tp__flex"
    :style="{ ...flexStyle }"
  >
    <slot />
  </div>
</template>

<style scoped>
.tp__flex {
  display: flex;
}
</style>
