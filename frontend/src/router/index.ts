import { useUsersStore } from '@/stores/users';
import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/pages/home-page.vue'),
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/pages/login-page/index.vue'),
    }
  ],
});

router.beforeEach(async (to) => {
  if (to.name === 'login') {
    return
  }

  const userStore = useUsersStore();

  try {
    await userStore.getMe();
  } catch(err) {
    return { name: 'login' }
  }
});

export default router;
