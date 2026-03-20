import { api } from '@/shared/api';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

export const useAuth = () => {
  const router = useRouter();

  const username = ref('');
  const password = ref('');

  const error = ref('');

  const authorize = async () => {
    error.value = '';

    if (!username.value || !password.value) {
      error.value = 'Введите логин и пароль';
      return;
    }

    try {
      await api.auth.authorize(username.value, password.value);

      router.push({ name: 'home' });
    } catch (err) {
      console.log({ err });
      error.value = 'Неверный логин или пароль';
    }
  };

  return {
    username,
    password,
    error,

    authorize,
  };
};
