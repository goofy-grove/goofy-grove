<script setup lang="ts">
import {
  AppConfirmModal,
  AppInput,
  type AppConfirmModalProps,
  type AppModalEmits,
} from '@/shared/ui';
import { Person } from '@/stores';
import { ref } from 'vue';

interface PersonEditModalProps extends AppConfirmModalProps {
  person: Person;
}

interface PersonEditModalEmits extends AppModalEmits {
  (e: 'confirm', person: Person): void;
}

const {
  person,
  showClose = true,
  showHeader = true,
  disableMove = false,
  showCancel = true,
  showConfirm = true,
  isLoading,
  ...props
} = defineProps<PersonEditModalProps>();

const editedPerson = ref<Person>(
  new Person(person.uid, person.name, person.description, person.creatorUid),
);

defineEmits<PersonEditModalEmits>();
</script>

<template>
  <app-confirm-modal
    v-bind="{
      ...props,
      isLoading,
      showCancel,
      showClose,
      showConfirm,
      showHeader,
    }"
    class="app-person-edit-modal"
    @close="$emit('close')"
    @confirm="$emit('confirm', editedPerson)"
  >
    <template #title>{{ $t('person.modals.edit_title') }}</template>

    <div class="app-person-edit-modal__content">
      <app-input
        v-model="editedPerson.name"
        :disabled="isLoading"
        :label="$t('person.labels.name')"
      />
      <app-input
        class="app-person-edit-modal__content__description-input"
        multiline
        v-model="editedPerson.description"
        :disabled="isLoading"
        :label="$t('person.labels.description')"
      />
    </div>
  </app-confirm-modal>
</template>

<style lang="scss">
.app-person-edit-modal {
  min-width: 600px;
  min-height: 400px;

  &__content {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 600px;

    &__description-input {
      resize: vertical;
      min-height: 100px;
    }
  }
}
</style>
