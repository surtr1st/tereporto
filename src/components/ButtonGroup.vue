<script setup lang="ts"></script>

<template>
  <div class="tp__button-group">
    <slot />
  </div>
</template>

<style scoped>
.tp__button-group {
  display: flex;
  gap: 0.1rem;
  width: auto;
}
.tp__button-group:first-child {
  border-top-left-radius: 10px;
  border-bottom-left-radius: 10px;
}
.tp__button-group:last-child {
  border-top-right-radius: 10px;
  border-bottom-right-radius: 10px;
}
</style>
