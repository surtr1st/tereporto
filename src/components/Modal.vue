<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

interface IModal {
  name?: string;
  open?: boolean;
  onClose?: () => void | Promise<void>;
}
const { open, onClose } = defineProps<IModal>();

const modal = ref<HTMLDivElement | null>();
const closeButton = ref<HTMLSpanElement | null>();

function closeOnEscape(e: KeyboardEvent) {
  const ESC = 'Escape';
  if (e.key === ESC) {
    e.preventDefault();
    onClose!();
  }
}
function display() {
  if (modal.value) modal.value.style.display = 'block';
}
function close() {
  if (modal.value) modal.value.style.display = 'none';
}

onMounted(() => {
  if (open) display();
  closeButton.value?.addEventListener('click', close);
  window.addEventListener('keydown', closeOnEscape);
});
onUnmounted(() => {
  closeButton.value?.removeEventListener('click', close);
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
            <span
              ref="close"
              class="tp__modal--close"
            >
              &times;
            </span>
            <h2>Modal Header</h2>
          </div>
          <div class="tp__modal-body">
            <p>Some text in the Modal Body</p>
            <p>Some other text...</p>
          </div>
          <div class="tp__modal-footer">
            <h3>Modal Footer</h3>
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
  display: none; /* Hidden by default */
  position: fixed; /* Stay in place */
  z-index: 1; /* Sit on top */
  padding-top: 100px; /* Location of the box */
  left: 0;
  top: 0;
  width: 100%; /* Full width */
  height: 100%; /* Full height */
  overflow: auto; /* Enable scroll if needed */
  background-color: rgb(0, 0, 0); /* Fallback color */
  background-color: rgba(0, 0, 0, 0.4); /* Black w/ opacity */
}

/* Modal Content */
.tp__modal-content {
  position: relative;
  background-color: #fefefe;
  margin: auto;
  padding: 0;
  border: 1px solid #888;
  width: 80%;
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
  -webkit-animation-name: animatetop;
  -webkit-animation-duration: 0.4s;
  animation-name: animatetop;
  animation-duration: 0.4s;
}

/* The Close Button */
.tp__modal--close {
  color: white;
  float: right;
  font-size: 28px;
  font-weight: bold;
}

.tp__modal--close:hover,
.tp__modal--close:focus {
  color: #000;
  text-decoration: none;
  cursor: pointer;
}

.tp__modal-header {
  padding: 2px 16px;
  background-color: #5cb85c;
  color: white;
}

.tp__modal-body {
  padding: 2px 16px;
}

.tp__modal-footer {
  padding: 2px 16px;
  background-color: #5cb85c;
  color: white;
}
</style>
