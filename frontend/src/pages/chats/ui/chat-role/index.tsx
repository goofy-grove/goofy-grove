import { useTranslation } from 'react-i18next';

import type { ChatRoleProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const ChatRole: FC<ChatRoleProps> = ({ kind }) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });

  return (
    <span className={`chat-role chat-role--${kind ?? 'unknown'}`}>
      {t(kind ?? 'unknownAuthor')}
    </span>
  );
};
