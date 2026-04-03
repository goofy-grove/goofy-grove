import { StrictMode } from 'react';

export const withStrictMode =
  (component: () => React.ReactNode) => (): React.ReactNode => (
    <StrictMode>{component()}</StrictMode>
  );
