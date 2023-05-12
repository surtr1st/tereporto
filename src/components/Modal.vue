<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';

interface IModal {
  name?: string;
  title?: string;
  open?: boolean;
  nested?: boolean;
  onClose?: () => void | Promise<void>;
}
const { open, onClose } = defineProps<IModal>();

function closeOnEscape(e: KeyboardEvent) {
  const ESC = 'Escape';
  if (e.key === ESC) {
    e.preventDefault();
    onClose!();
  }
}

onMounted(() => {
  window.addEventListener('keydown', closeOnEscape);
});
onUnmounted(() => {
  window.removeEventListener('keydown', closeOnEscape);
});
</script>

<template>
  <Teleport to="body">
    <Transition name="fade">
      <div
        :id="name"
        :class="nested ? 'tp__nested-modal' : 'tp__modal'"
        v-if="open"
      >
        <div class="tp__modal-content">
          <div class="tp__modal-header">
            <h2>{{ title }}</h2>
            <span
              @click="onClose"
              class="tp__modal--close"
            >
              &times;
            </span>
          </div>
          <slot />
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.tp__modal {
  position: fixed;
  top: 0;
  left: 0;
  background: var(--dark-bg);
  border-radius: 0.7rem;
  width: 100%;
  height: 100%;
  z-index: 1000;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 0.2rem;
  box-shadow: 1px 2px black;
  background-color: rgb(0, 0, 0); /* Fallback color */
  background-color: rgba(0, 0, 0, 0.4); /* Black w/ opacity */
}

.tp__nested-modal {
  position: fixed;
  top: 0;
  left: 0;
  background: var(--dark-bg);
  border-radius: 0.7rem;
  width: 100%;
  height: 100%;
  z-index: 10000;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 0.2rem;
  box-shadow: 1px 2px black;
  background-color: rgb(0, 0, 0); /* Fallback color */
  background-color: rgba(0, 0, 0, 0.4); /* Black w/ opacity */
}
.fade-enter-active {
  animation: fade-in 100ms;
  animation-fill-mode: forwards;
  animation-timing-function: ease-in;
}
.fade-leave-active {
  animation: fade-out 100ms;
  animation-fill-mode: forwards;
  animation-timing-function: ease-out;
}

.tp__modal-content {
  position: relative;
  margin: auto;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 0.6rem;
  width: 40%;
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
  border-radius: 10px;
}

.tp__modal--close {
  color: white;
  float: right;
  font-size: 24px;
  font-weight: bold;
}

.tp__modal--close:hover,
.tp__modal--close:focus {
  text-decoration: none;
  cursor: pointer;
}

.tp__modal-header {
  display: flex;
  justify-content: space-between;
  padding: 0.7rem;
  border-radius: 10px;
}
.tp__modal-header > h2 {
  color: #fefefe;
}

.tp__modal-body {
  padding: 1rem;
  display: grid;
  height: 50vh;
  max-height: 70vh;
  place-items: center;
  overflow-y: auto;
  overflow-x: hidden;
  border-radius: 10px;
  padding: 1rem;
}

.tp__modal-footer {
  display: flex;
  justify-content: center;
  gap: 3rem;
  align-items: center;
  height: 20vh;
  max-height: 35vh;
  padding: 1.2rem;
  margin-top: 1rem;
  margin-bottom: 0.5rem;
  border-radius: 10px;
}
</style>
