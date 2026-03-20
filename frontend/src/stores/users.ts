import { api } from '@/shared/api';
import { defineStore } from 'pinia';
import { ref } from 'vue';

class User {
  constructor(
    public readonly id: string,
    public readonly username: string,
  ) {}
}

export const useUsersStore = defineStore('users', () => {
  const currentUser = ref<User | null>(null);

  const getMe = async () => {
    const response = await api.users.getMe();

    if (!response.error) {
      currentUser.value = new User(response.data.id, response.data.username);
    }
  };

  return {
    currentUser,

    getMe,
  };
});
