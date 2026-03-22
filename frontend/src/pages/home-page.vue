<script setup lang="ts">
import { PersonItem } from '@/components';
import { usePersonsStore } from '@/stores';

const personsStore = usePersonsStore();

personsStore.loadPersons();

</script>

<template>
  <h1>Index</h1>
  <div class="persons-list">
    <person-item
      v-for="person in personsStore.persons"
      :key="person.uid"
      :person="person"
    />
  </div>
</template>
