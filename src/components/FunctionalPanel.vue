<template>
  <div class="tp__functional">
    <slot />
  </div>
</template>

<style scoped>
.tp__functional {
  width: 1012px;
  height: 96px;
  border-radius: 10px;
  background: var(--neutral-gray);
  padding: 1.2rem;
  color: white;
  display: flex;
  justify-content: space-evenly;
  align-items: center;
}
</style>
