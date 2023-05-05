<template>
  <div class="tp__grid">
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
  place-items: center;
}
</style>
