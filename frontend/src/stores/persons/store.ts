import { api } from '@/shared/api';
import { defineStore } from 'pinia';
import { Person, usePersonsState } from './state';

export const usePersonsStore = defineStore('persons', () => {
  const { persons, addPerson } = usePersonsState();

  const createPerson = async (name: string, description: string) => {
    const response = await api.persons.create(name, description);

    if (!response.error) {
      addPerson([
        new Person(
          response.data.uid,
          response.data.name,
          response.data.description,
          response.data.creator_uid,
        ),
      ]);
    }
  };

  const loadPersons = async () => {
    const response = await api.persons.getAll();

    if (!response.error) {
      addPerson(
        response.data.map((person) => {
          return new Person(
            person.uid,
            person.name,
            person.description,
            person.creator_uid,
          );
        }),
      );
    }
  };

  const updatePerson = (person: Person) => {
    const index = persons.value.findIndex((p) => p.uid === person.uid);

    if (index !== -1) {
      persons.value[index] = person;
    }
  };

  return {
    persons,

    createPerson,
    loadPersons,
    updatePerson,
  };
});
