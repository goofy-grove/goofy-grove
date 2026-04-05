import { AuthProvider } from './auth-provider';

export const withAuth = (component: () => React.ReactNode) => () => {
  return <AuthProvider>{component()}</AuthProvider>;
};
