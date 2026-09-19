import { createInstance } from 'i18next';
import { I18nextProvider, initReactI18next } from 'react-i18next';

import en from '../public/locales/en.json';
import ru from '../public/locales/ru.json';

import type { Preview } from '@storybook/react-vite';

import '@app/assets/general.scss';
import './preview.scss';

const i18n = createInstance();
await i18n.use(initReactI18next).init({
  lng: 'ru',
  fallbackLng: 'en',
  resources: { en: { translation: en }, ru: { translation: ru } },
  interpolation: { escapeValue: false },
});

const preview: Preview = {
  tags: ['autodocs'],
  parameters: {
    layout: 'padded',
    backgrounds: { disable: true },
    a11y: { test: 'todo' },
  },
  globalTypes: {
    locale: {
      description: 'Language',
      toolbar: {
        icon: 'globe',
        items: [
          { value: 'ru', title: 'Русский' },
          { value: 'en', title: 'English' },
        ],
      },
    },
  },
  initialGlobals: { locale: 'ru' },
  loaders: [
    async ({ globals }) => {
      await i18n.changeLanguage(String(globals.locale));

      return {};
    },
  ],
  decorators: [
    (Story) => (
      <I18nextProvider i18n={i18n}>
        <Story />
      </I18nextProvider>
    ),
  ],
};

export default preview;
