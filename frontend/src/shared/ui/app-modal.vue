<script setup lang="ts">
// TODO: implement independent modal management
import { useDraggable, useElementSize, useMediaQuery } from '@vueuse/core';
import { computed, ref, useTemplateRef } from 'vue';
import { GrFormClose } from 'vue-icons-plus/gr';
import AppButton from './app-button.vue';

interface AppModalProps {
  initialX?: number;
  initialY?: number;
  isOpen: boolean;
}

interface AppModalEmits {
  (e: 'close'): void;
}

interface AppModalSlots {
  title: () => void;
  default: () => void;
}

defineEmits<AppModalEmits>();
defineSlots<AppModalSlots>();

const props = defineProps<AppModalProps>();
const modalRef = useTemplateRef('app-modal');
const modalHeaderRef = useTemplateRef('app-modal__header');

const isMobile = useMediaQuery('(max-width: 768px)');
const { width, height } = useElementSize(modalRef);
const isModalShouldBeExpanded = computed(
  () =>
    isMobile.value ||
    width.value > window.innerWidth ||
    height.value > window.innerHeight,
);

const { style } = useDraggable(modalRef, {
  initialValue() {
    return {
      x: props.initialX ?? window.innerWidth / 4,
      y: props.initialY ?? window.innerHeight / 4,
    };
  },
  handle: modalHeaderRef,
  containerElement: ref(document.body),
  disabled: isModalShouldBeExpanded,
});
</script>

<template>
  <Teleport to="body">
    <Transition name="bounce" :css="!isModalShouldBeExpanded">
      <div
        v-if="isOpen"
        ref="app-modal"
        :style="!isModalShouldBeExpanded ? style : {}"
        class="app-modal"
        :class="{ 'app-modal--expanded': isMobile }"
      >
        <div class="app-modal__header" ref="app-modal__header">
          <div class="app-modal__header__title">
            <slot name="title" />
          </div>

          <app-button
            class="app-modal__header__close-button"
            type="ghost"
            color="error"
            @click="$emit('close')"
          >
            <gr-form-close />
          </app-button>
        </div>

        <div class="app-modal__content">
          <slot />
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style lang="scss" scoped>
.app-modal {
  position: fixed;
  z-index: var(--z-index-modal);
  border: 1px solid #434343;
  background-color: #212121;
  border-radius: 16px;
  box-shadow: 0 0 40px rgba(0, 0, 0, 0.5);
  overflow: hidden;

  transition:
    width 0.5s ease,
    height 0.5s ease,
    border-radius 0.5s ease,
    box-shadow 0.5s ease;

  &--expanded {
    position: absolute;
    top: 0;
    left: 0;
    width: 100dvw;
    height: 100dvh;
    border-radius: 0;
    box-shadow: none;
    border: none;
  }

  &__content {
    padding: 8px;
  }

  &__header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    background-color: #292929;

    &__title {
      flex-grow: 1;
      text-align: center;
      max-width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__close-button {
      flex-shrink: 0;
      border-radius: 100%;
      width: 32px;
      height: 32px;
      padding: 0;
      display: flex;
      align-items: center;
      justify-content: center;
    }
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
