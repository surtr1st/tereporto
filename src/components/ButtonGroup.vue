<script setup lang="ts"></script>

<template>
  <div class="tp__button-group">
    <slot />
  </div>
</template>

<style>
.tp__button-group {
  gap: 5px;
  width: auto;
  float: left;
  background: inherit;
}
.tp__button-group::after {
  content: '';
  clear: both;
  display: table;
}
.tp__button-group button:is(:first-child) {
  border-top-left-radius: 10px;
  border-bottom-left-radius: 10px;
}
.tp__button-group button:is(:last-child) {
  border-top-right-radius: 10px;
  border-bottom-right-radius: 10px;
}
</style>
