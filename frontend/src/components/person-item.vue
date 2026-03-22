<script setup lang="ts">
import { AppAvatar, AppModal } from '@/shared/ui';
import type { Person } from '@/stores';
import { ref } from 'vue';

interface PersonItemProps {
  person: Person;
}


const isModalOpen = ref(false);

const openModal = () => {
  isModalOpen.value = true;
};

defineProps<PersonItemProps>();
</script>

<template>
  <div class="person-item" @click="openModal">
    <app-avatar src="afsd" :alt="person.name" />

    <div class="person-item__info">
      <p class="person-item__info__name">{{ person.name }}</p>
      <p class="person-item__info__description">{{ person.description }}</p>
    </div>

    <app-modal :is-open="isModalOpen" @close="isModalOpen = false">
      <p>{{ person.name }}</p>
      <p>Modal</p>
    </app-modal>
  </div>
</template>

<style lang="scss" scoped>
.person-item {
  display: flex;
  padding: 8px;
  border-radius: 16px;
  border: 1px solid var(--border-color);
  cursor: pointer;

  &:hover {
    background-color: #292929;
  }

  &__info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 4px;
    max-width: 100%;
    overflow: hidden;

    &__name,
    &__description {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__name {
      font-size: 1.25rem;
      width: 100%;
    }

    &__description {
      max-width: min(100%, 200px);
      font-size: 1rem;
    }
  }
}
</style>
