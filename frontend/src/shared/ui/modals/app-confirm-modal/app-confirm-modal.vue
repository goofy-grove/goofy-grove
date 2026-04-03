<script setup lang="ts">
import AppButton from '../../app-button.vue';
import { AppModal, type AppModalSlots } from '../app-modal';
import type { AppConfirmModalEmits, AppConfirmModalProps } from './types';

const {
  showClose = true,
  showHeader = true,
  disableMove = false,
  showCancel = true,
  showConfirm = true,
  isLoading = false,
  ...props
} = defineProps<AppConfirmModalProps>();

defineEmits<AppConfirmModalEmits>();

const slots = defineSlots<AppModalSlots>();
</script>

<template>
  <app-modal
    v-bind="{ ...props, showClose, showHeader, disableMove }"
    @close="$emit('close')"
  >
    <template v-if="!slots.title" #title>
      {{ $t('modals.confirm.title') }}
    </template>

    <template v-else #title>
      <slot name="title" />
    </template>

    <div class="app-modals-confirm">
      <div class="app-modals-confirm__content">
        <slot />
      </div>

      <div class="app-modals-confirm__action-buttons">
        <app-button
          class="app-modals-confirm__action-buttons__button"
          :disabled="isLoading"
          @click="$emit('close')"
        >
          {{ $t('modals.confirm.cancel') }}
        </app-button>
        <app-button
          class="app-modals-confirm__action-buttons__button"
          color="success"
          :disabled="isLoading"
          :is-loading="isLoading"
          @click="$emit('confirm')"
        >
          {{ $t('modals.confirm.confirm') }}
        </app-button>
      </div>
    </div>
  </app-modal>
</template>

<style scoped lang="scss">
.app-modals-confirm {
  display: flex;
  flex-direction: column;
  height: 100%;
  flex-grow: 1;
  padding: 8px;
  gap: 8px;

  &__action-buttons {
    margin-top: auto;
    display: flex;
    gap: 8px;

    &__button {
      flex-grow: 1;
    }
  }
}
</style>
