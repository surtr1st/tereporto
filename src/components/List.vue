<template>
  <ul>
    <slot />
  </ul>
</template>

<style scoped>
ul {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  background: var(--neutral-gray);
}
</style>
