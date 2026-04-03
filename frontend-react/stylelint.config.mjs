/** @type {import('stylelint').Config} */
export default {
  plugins: ['stylelint-order', 'stylelint-prettier'],
  extends: [
    'stylelint-config-sass-guidelines',
    'stylelint-prettier/recommended',
  ],
  customSyntax: 'postcss-scss',
  rules: {
    'property-no-unknown': true,
    'selector-max-id': 2,
  },
};
