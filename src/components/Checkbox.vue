<script setup lang="ts">
interface ICheckbox {
  label?: string;
  checked?: boolean;
}
defineProps<ICheckbox>();
</script>

<template>
  <label class="tp__checkbox-label">
    {{ label }}
    <input
      type="checkbox"
      :checked="checked"
    />
    <span class="tp__checkmark"></span>
  </label>
</template>

<style scoped>
.tp__checkbox-label {
  display: block;
  position: relative;
  padding-top: 3.5px;
  padding-left: 35px;
  cursor: pointer;
  font-size: 15px;
  font-weight: 500;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  background: inherit;
}

/* Hide the browser's default checkbox */
.tp__checkbox-label input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

/* Create a custom checkbox */
.tp__checkmark {
  position: absolute;
  top: 0;
  left: 0;
  height: 30px;
  width: 30px;
  border-radius: 10px;
  background: var(--darker-neutral);
}

/* On mouse-over, add a grey background color */
.tp__checkbox-label:hover input ~ .tp__checkmark {
  background: var(--light-neutral-gray);
}

/* When the checkbox is checked, add a blue background */
.tp__checkbox-label input:checked ~ .tp__checkmark {
  background-color: #4f42ff;
}

/* Create the checkmark/indicator (hidden when not checked) */
.tp__checkmark:after {
  content: '';
  position: absolute;
  display: none;
}

/* Show the checkmark when checked */
.tp__checkbox-label input:checked ~ .tp__checkmark:after {
  display: block;
}

/* Style the checkmark/indicator */
.tp__checkbox-label .tp__checkmark:after {
  left: 11px;
  top: 7px;
  width: 5px;
  height: 10px;
  border: solid white;
  border-width: 0 3px 3px 0;
  -webkit-transform: rotate(45deg);
  -ms-transform: rotate(45deg);
  transform: rotate(45deg);
}
</style>
