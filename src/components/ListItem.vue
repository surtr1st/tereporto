<template>
  <li>
    <slot />
  </li>
</template>

<style scoped>
li {
  width: 470px;
  height: 109px;
  display: flex;
  justify-content: center;
  align-items: center;
  background: var(--light-neutral-gray);
  border-radius: 10px;
  margin: 0.3rem;
  transition: all 150ms;
}

li:hover {
  background: var(--darker-neutral-gray);
  cursor: pointer;
}
</style>
