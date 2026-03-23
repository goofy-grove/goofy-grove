<script setup lang="ts">
import AppButton from '../app-button.vue';
import type { AppModalEmits, AppModalProps } from './app-modal';
import { AppModal } from './app-modal';

type AppConfirmModalProps = AppModalProps & {
  showCancel?: boolean;
  showConfirm?: boolean;
};
type AppConfirmModalEmits = AppModalEmits & {
  (e: 'confirm'): void;
};

const {
  showClose = true,
  showHeader = true,
  disableMove = false,
  showCancel = true,
  showConfirm = true,
  ...props
} = defineProps<AppConfirmModalProps>();
defineEmits<AppConfirmModalEmits>();
</script>

<template>
  <app-modal
    v-bind="props"
    :show-close="showClose"
    :show-header="showHeader"
    :disable-move="disableMove"
    @close="$emit('close')"
  >
    <template #title>{{ $t('modals.confirm.title') }}</template>

    <div class="app-modals-confirm">
      <div class="app-modals-confirm__content">
        <slot />
      </div>

      <div class="app-modals-confirm__action-buttons">
        <app-button
          class="app-modals-confirm__action-buttons__button"
          @click="$emit('close')"
        >
          {{ $t('modals.confirm.cancel') }}
        </app-button>
        <app-button
          class="app-modals-confirm__action-buttons__button"
          color="success"
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
