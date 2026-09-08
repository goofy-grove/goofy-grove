import { Text } from '@shared/ui/text';

import type { PageHeaderProps } from './types';

import './styles.scss';

export const PageHeader = ({ title, children, className }: PageHeaderProps) => {
  return (
    <header className={`page-header ${className ?? ''}`}>
      <Text tag="h2">{title}</Text>

      <div className="page-header__actions">{children}</div>
    </header>
  );
};
