import { StrictMode } from 'react';

export const withStrictMode = (component: () => React.ReactNode) => () => (
  <StrictMode>{component()}</StrictMode>
);
