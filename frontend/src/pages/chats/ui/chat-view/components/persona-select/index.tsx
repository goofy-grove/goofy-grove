import { useTranslation } from 'react-i18next';

import { IdentityAvatar } from '@pages/chats/ui/chat-view/components/identity-avatar';

import { Select } from '@shared/ui/select';

import type { PersonaSelectProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const PersonaSelect: FC<PersonaSelectProps> = ({
  personas,
  selectedId,
  onChange,
}) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });

  const renderPersona = (id: string) => {
    const persona = personas.find((item) => item.id === id);

    return persona && (
      <span className="chat-persona-select__value">
        <IdentityAvatar identity={persona} small />

        <span>{persona.name}</span>
      </span>
    );
  };

  return (
    <Select
      className="chat-persona-select"
      items={personas.map((persona) => ({
        value: persona.id,
        label: persona.name,
      }))}
      selected={selectedId}
      placeholder={t('choose')}
      onChange={onChange}
      renderOption={(item) => renderPersona(item.value)}
      renderValue={(items) => renderPersona(items[0].value)}
    />
  );
};
