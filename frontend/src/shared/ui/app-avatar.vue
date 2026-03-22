<script setup lang="ts">
import { ref } from 'vue';

interface AppAvatarEmits {
  (e: 'error', event: Event): void;
  (e: 'load', event: Event): void;
}

const emit = defineEmits<AppAvatarEmits>();

const isLoaded = ref(false);
const onError = (event: Event) => {
  isLoaded.value = false;

  emit('error', event);
};

const onLoad = (event: Event) => {
  isLoaded.value = true;

  emit('load', event);
};
</script>

<template>
  <div class="app-avatar">
    <img v-bind="$attrs" v-show="isLoaded" @error="onError" @load="onLoad" />
  </div>
</template>

<style lang="scss" scoped>
.app-avatar {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  border-radius: 100%;
  width: 48px;
  height: 48px;
  background-color: #333;
  border: 1px solid #464646;
}
</style>
