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
    'max-nesting-depth': null,
    'selector-no-qualifying-type': null,
    'selector-max-id': 1,
  },
};
