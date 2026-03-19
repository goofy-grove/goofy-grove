<script setup lang="ts">
interface AppAlertProps {
  type?: 'error' | 'default';
}

withDefaults(defineProps<AppAlertProps>(), {
  type: 'default',
});
</script>

<template>
  <p :class="['app-alert', `app-alert--${type}`]" v-bind="$attrs">
    <slot />
  </p>
</template>

<style scoped lang="scss">
.app-alert {
  color: var(--text-color);
  padding: 8px 12px;
  font-size: 1.2rem;
  border-radius: 6px;

  &--error {
    --error-color: #ff7777;

    color: var(--error-color);
    background-color: #442222;
  }
}
</style>
