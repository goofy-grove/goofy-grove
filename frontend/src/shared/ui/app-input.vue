<script setup lang="ts">
interface AppInputProps {
  label?: string;
  id?: string;
}

const modelValue = defineModel('modelValue');

defineProps<AppInputProps>();
</script>

<template>
  <div class="app-input">
    <label :for="id" v-if="label">{{ label }}</label>
    <input
      class="app-input__input"
      :id="id"
      v-model="modelValue"
      v-bind="$attrs"
    />
  </div>
</template>

<style lang="scss" scoped>
.app-input {
  display: flex;
  flex-direction: column;
  gap: 4px;

  &__input {
    padding: 10px 20px 10px 10px;
    border: 1px solid transparent;
    color: var(--text-color);
    background-color: #333;
    border-radius: 6px;
    outline: none;
    font-size: 1.2rem;

    &:hover {
      border: 1px solid #464646;
    }

    &:focus {
      background-color: #434343;
      border: 1px solid var(--text-color);
    }
  }
}
</style>
