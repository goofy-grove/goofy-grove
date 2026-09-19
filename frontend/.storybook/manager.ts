import { addons } from 'storybook/manager-api';
import { create } from 'storybook/theming';

addons.setConfig({
  theme: create({
    base: 'dark',
    brandTitle: 'Goofy Grove · UI',
    brandImage: '/grove.svg',
    fontBase: 'Nunito, sans-serif',
    colorPrimary: '#b5a0cd',
    colorSecondary: '#bbce9b',
    appBg: '#101d17',
    appContentBg: '#1c2c24',
    appPreviewBg: '#101d17',
    appBorderColor: '#34483b',
    appBorderRadius: 14,
    textColor: '#e6eddf',
    barBg: '#1c2c24',
    barTextColor: '#aabcae',
    barSelectedColor: '#bbce9b',
    inputBg: '#23362c',
    inputBorder: '#34483b',
    inputTextColor: '#e6eddf',
    inputBorderRadius: 10,
  }),
});
