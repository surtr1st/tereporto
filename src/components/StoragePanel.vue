<template>
  <div class="tp__storage">
    <slot />
  </div>
</template>

<style scoped>
.tp__storage {
  width: 500px;
  height: 384px;
  border-radius: 10px;
  overflow-y: auto;
  overflow-x: hidden;
  background: var(--neutral-gray);
  padding: 1.2rem;
  color: white;
}
</style>
