import axios from 'axios';

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  withCredentials: true,
});

export const setUpAuthInterceptor = (
  getValiToken: () => string | null,
  refresh: () => Promise<void>,
) => {
  api.interceptors.request.use(async (config) => {
    if (config.skipAuth) {
      return config;
    }

    const token = getValiToken();

    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    } else {
      await refresh();
    }

    config.headers.Authorization = `Bearer ${getValiToken()}`;

    return config;
  });
};
