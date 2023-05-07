<script setup lang="ts">
interface IRadio {
  name?: string;
  label?: string;
  checked?: boolean;
}
defineProps<IRadio>();
</script>

<template>
  <label
    :for="name"
    class="tp__radio"
  >
    {{ label }}
    <input
      type="radio"
      :checked="checked"
      :name="name"
    />
    <span class="tp__checkmark" />
  </label>
</template>

<style scoped>
/* The container */
.tp__radio {
  display: block;
  position: relative;
  padding-left: 30px;
  margin-top: 1px;
  cursor: pointer;
  font-size: 15px;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  background: inherit;
}

/* Hide the browser's default radio button */
.tp__radio input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
}

/* Create a custom radio button */
.tp__checkmark {
  position: absolute;
  top: 0;
  left: 0;
  height: 22px;
  width: 22px;
  background-color: var(--darker-neutral);
  border-radius: 50%;
  transition: all 150ms;
}

/* On mouse-over, add a grey background color */
.tp__radio:hover input ~ .tp__checkmark {
  background-color: var(--light-neutral-gray);
}

/* When the radio button is checked, add a blue background */
.tp__radio input:checked ~ .tp__checkmark {
  background-color: #4f42ff;
}

/* Create the indicator (the dot/circle - hidden when not checked) */
.tp__checkmark:after {
  content: '';
  position: absolute;
  display: none;
}

/* Show the indicator (dot/circle) when checked */
.tp__radio input:checked ~ .tp__checkmark:after {
  display: block;
}

/* Style the indicator (dot/circle) */
.tp__radio .tp__checkmark:after {
  top: 6.2px;
  left: 6.2px;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: white;
}
</style>
