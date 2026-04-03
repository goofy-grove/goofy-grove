import { ref } from "vue";

export class Person {
  constructor(
    public readonly uid: string,
    public name: string,
    public description: string,
    public readonly creatorUid: string,
  ) {}
}

export const usePersonsState = () => {
  const persons = ref<Person[]>([]);

  const addPerson = (person: Person[]) => {
    persons.value.push(...person);
  };

  return {
    persons,

    addPerson,
  };
}
