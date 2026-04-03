<script setup lang="ts">
interface AppInputProps {
  label?: string;
  id?: string;
  multiline?: boolean;
  disabled?: boolean;
}

const modelValue = defineModel<string>();

defineProps<AppInputProps>();
</script>

<template>
  <div class="app-input">
    <label :for="id" v-if="label">{{ label }}</label>

    <textarea
      v-if="multiline"
      :id="id"
      :disabled="disabled"
      class="app-input__input"
      v-model="modelValue"
      v-bind="$attrs"
    />

    <input
      v-else
      :id="id"
      :disabled="disabled"
      class="app-input__input"
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

    &:-webkit-autofill,
    &:-webkit-autofill:hover,
    &:-webkit-autofill:active,
    &:-webkit-autofill:focus {
      -webkit-box-shadow: 0 0 0 100px #333 inset !important;
      box-shadow: 0 0 0 100px #333 inset !important;
      -webkit-text-fill-color: var(--text-color) !important;
      caret-color: var(--text-color) !important;
      transition: background-color 5000s;
    }

    &:hover,
    &:-webkit-autofill:hover {
      border: 1px solid #464646 !important;
    }

    &:focus,
    &:-webkit-autofill:focus {
      background-color: #434343 !important;
      border: 1px solid var(--text-color);
    }

    &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }
}
</style>
