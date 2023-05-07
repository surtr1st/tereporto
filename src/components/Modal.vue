<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';

interface IModal {
  name?: string;
  title?: string;
  open?: boolean;
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
        ref="modal"
        class="tp__modal"
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
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
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
.tp__modal {
  position: fixed;
  z-index: 1000;
  width: 100%;
  height: 100%;
  overflow: auto;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  background-color: rgb(0, 0, 0); /* Fallback color */
  background-color: rgba(0, 0, 0, 0.4); /* Black w/ opacity */
  border-radius: 10px;
}

.tp__modal-content {
  position: relative;
  margin: auto;
  padding: 0;
  width: 50%;
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
}
.tp__modal-header > h2 {
  color: #fefefe;
}
</style>
