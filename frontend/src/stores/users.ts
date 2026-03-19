import { api } from '@/shared/api';
import { defineStore } from 'pinia';
import { ref } from 'vue';

type User = {
  id: string;
  username: string;
};

export const useUsersStore = defineStore('users', () => {
  const currentUser = ref<User | null>(null);

  const getMe = async () => {
    const response = await api.user.getMe();

    if (!response.error) {
      currentUser.value = response.data;
    }
  };

  return {
    currentUser,

    getMe,
  };
});
