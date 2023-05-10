<script setup lang="ts">
interface IFileSelection {
  label?: string;
  name?: string;
}
defineProps<IFileSelection>();
</script>

<template>
  <label
    class="tp__file-label"
    :for="name"
  >
    {{ label }}
  </label>
  <input
    type="file"
    :id="name"
    :name="name"
  />
</template>

<style scoped>
input[type='file'] {
  display: none;
}

.tp__file-label {
  min-width: 38px;
  min-height: 38px;
  width: 122px;
  height: 51px;
  border: none;
  font-weight: 500;
  background: var(--darker-color);
  color: white;
  border-radius: 10px;
  transition: 250ms ease-in-out all;
  display: grid;
  place-items: center;
}

.tp__file-label:hover,
.tp__file-label > label:hover,
.ftp__ile-label > label {
  cursor: pointer;
  background: var(--light-neutral-gray);
}

.tp__file-label > label {
  background: inherit;
  display: flex;
  gap: 0.25rem;
}
</style>
