export type AlertProps = {
  type?: 'success' | 'error' | 'warning' | 'info';
  message: string;
  closable?: boolean;
};
