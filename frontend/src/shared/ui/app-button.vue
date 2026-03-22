<script setup lang="ts">
interface ButtonProps {
  type?: 'default' | 'ghost';
  color?: 'default' | 'error';
}

withDefaults(defineProps<ButtonProps>(), {
  type: 'default',
});
</script>

<template>
  <button class="app-button" :class="`app-button--${type} app-button--color-${color}`">
    <slot />
  </button>
</template>

<style scoped lang="scss">
.app-button {
  --background-color: #333;
  --hover-color: #434343;

  padding: 10px 20px;
  border: none;
  color: var(--text-color);
  background-color: var(--background-color);
  border-radius: 6px;
  cursor: pointer;
  font-size: 1.2rem;

  transition: background-color 0.25s ease;

  &--ghost {
    --background-color: transparent;
    --hover-color: #333;
  }

  &--color-error {
    --background-color: #995555;
    --hover-color: #bb5555;
  }

  &--ghost.app-button--color-error {
    --background-color: transparent;
    --hover-color: #995555;
  }

  &:hover {
    background-color: var(--hover-color);
  }
}
</style>
