import { useNavigate, useRouterState } from '@tanstack/react-router';
import { useState } from 'react';

import { useAuth } from '@entities/auth';

export const useLoginForm = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const { login } = useAuth();

  const navigate = useNavigate();
  const routerState = useRouterState();

  const handleSubmit = async () => {
    setError('');
    setIsLoading(true);

    if (!username || !password) {
      setError('login.errors.enter_credentials');
      setIsLoading(false);

      return;
    }

    try {
      await login(username, password);
    } catch {
      setError('login.errors.invalid_credentials');
      setIsLoading(false);

      return;
    }

    setIsLoading(false);

    const search = routerState.location.search as { redirect?: string };

    await navigate({
      to: search.redirect ?? '/personas',
      replace: true,
    });
  };

  return {
    isLoading,
    error,
    username,
    password,

    setUsername,
    setPassword,
    handleSubmit,
  };
};
