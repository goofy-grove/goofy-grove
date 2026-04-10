import { StrictMode } from 'react';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider } from '@tanstack/react-router';

import { WindowsProvider } from '../shared/ui';
import { useAuth, AuthProvider } from '../entities/auth';

import { queryClient, router } from './lib';

const InnerApp = () => {
  const auth = useAuth();

  return <RouterProvider router={router} context={{ auth }} />;
};

export const App = () => {
  return (
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <WindowsProvider>
          <AuthProvider>
            <InnerApp />
          </AuthProvider>
        </WindowsProvider>

        {import.meta.env.DEV && <ReactQueryDevtools initialIsOpen={false} />}
      </QueryClientProvider>
    </StrictMode>
  );
};
