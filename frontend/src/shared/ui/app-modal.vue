<script setup lang="ts">
// TODO: implement independent modal management
import { useDraggable, useMediaQuery } from '@vueuse/core';
import { ref, useTemplateRef } from 'vue';

interface AppModalProps {
  initialX?: number;
  initialY?: number;
  isOpen: boolean;
}

interface AppModalEmits {
  (e: 'close'): void;
}

defineEmits<AppModalEmits>();

const props = defineProps<AppModalProps>();
const modalRef = useTemplateRef('app-modal');

const isMobile = useMediaQuery('(max-width: 768px)');
const { style } = useDraggable(modalRef, {
  initialValue: {
    x: props.initialX ?? (window.innerWidth - 600) / 2,
    y: props.initialY ?? (window.innerHeight - 400) / 2,
  },
  containerElement: ref(document.body),
  preventDefault: true,
  disabled: isMobile,
});
</script>

<template>
  <Teleport to="body">
    <Transition name="bounce">
      <div
        v-if="isOpen"
        ref="app-modal"
        :style="style"
        class="app-modal"
        :class="{ 'app-modal--expanded': isMobile }"
      >
        <button @click="$emit('close')">Close</button>

        <slot />
      </div>
    </Transition>
  </Teleport>
</template>

<style lang="scss" scoped>
.app-modal {
  position: fixed;
  z-index: var(--z-index-modal);
  width: 600px;
  height: 400px;
  border: 1px solid #434343;
  background-color: #212121;
  border-radius: 16px;
  box-shadow: 0 0 40px rgba(0, 0, 0, 0.5);

  transition:
    width 0.5s ease,
    height 0.5s ease,
    border-radius 0.5s ease,
    box-shadow 0.5s ease;

  &--expanded {
    position: absolute;
    top: 0 !important;
    left: 0 !important;
    width: 100%;
    height: 100%;
    border-radius: 0;
    box-shadow: none;
  }
}

.bounce-enter-active {
  animation: bounce-in 0.5s;
}

.bounce-leave-active {
  animation: bounce-in 0.5s reverse;
}

@keyframes bounce-in {
  0% {
    transform: scale(0);
  }
  50% {
    transform: scale(1.15);
  }
  100% {
    transform: scale(1);
  }
}
</style>
