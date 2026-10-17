import { useTranslation } from 'react-i18next';

import { LocaleSwitcher, PageHeader, Text } from '@shared/ui';

import type { FC } from 'react';

import './styles.scss';

export const SettingsPage: FC = () => {
  const { t } = useTranslation();

  return (
    <div className="settings-page">
      <PageHeader title={t('menu.settings')} />

      <Text>{t('grove.settings_intro')}</Text>

      <section className="settings-page__section">
        <Text tag="h3">{t('ui.select_locale')}</Text>

        <LocaleSwitcher />
      </section>
    </div>
  );
};
