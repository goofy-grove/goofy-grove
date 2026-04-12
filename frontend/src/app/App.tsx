import { QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { RouterProvider } from '@tanstack/react-router';
import { StrictMode } from 'react';

import { AuthProvider, useAuth } from '@entities/auth';

import { WindowsProvider } from '@shared/ui';

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
