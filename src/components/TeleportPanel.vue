<template>
  <div class="tp__teleport">
    <slot />
  </div>
</template>

<style scoped>
.tp__teleport {
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
