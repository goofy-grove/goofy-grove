import { api } from '@/shared/api';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

export const useAuth = () => {
  const router = useRouter();
  const { t } = useI18n();

  const username = ref('');
  const password = ref('');

  const error = ref('');

  const authorize = async () => {
    error.value = '';

    if (!username.value || !password.value) {
      error.value = t('login.errors.enter_credentials');
      return;
    }

    try {
      await api.auth.authorize(username.value, password.value);

      router.push({ name: 'home' });
    } catch (err) {
      console.log({ err });
      error.value = t('login.errors.invalid_credentials');
    }
  };

  return {
    username,
    password,
    error,

    authorize,
  };
};
