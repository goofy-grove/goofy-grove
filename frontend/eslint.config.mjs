import importAliases from '@dword-design/eslint-plugin-import-alias';
import js from '@eslint/js';
import stylistic from '@stylistic/eslint-plugin';
import pluginRouter from '@tanstack/eslint-plugin-router';
import { createTypeScriptImportResolver } from 'eslint-import-resolver-typescript';
import { importX } from 'eslint-plugin-import-x';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import { defineConfig, globalIgnores } from 'eslint/config';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default defineConfig(
  globalIgnores([
    'dist',
    'storybook-static',
    'src/app/lib/@tanstack/router/route-tree.gen.ts',
  ]),
  {
    name: 'frontend/typescript',
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommendedTypeChecked,
      reactHooks.configs.flat['recommended-latest'],
      reactRefresh.configs.vite,
      importX.flatConfigs.recommended,
      importX.flatConfigs.typescript,
      importX.flatConfigs.react,
      pluginRouter.configs['flat/recommended'],
      importAliases.configs.recommended,
    ],
    plugins: {
      '@stylistic': stylistic,
    },
    languageOptions: {
      ecmaVersion: 2023,
      globals: globals.browser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    settings: {
      'import-x/resolver-next': [
        createTypeScriptImportResolver({
          alwaysTryTypes: true,
          project: `${import.meta.dirname}/tsconfig.json`,
        }),
      ],
    },
    rules: {
      '@stylistic/padding-line-between-statements': [
        'error',
        { blankLine: 'always', prev: 'if', next: '*' },
        {
          blankLine: 'always',
          prev: '*',
          next: ['if', 'for', 'while', 'switch', 'try'],
        },
        { blankLine: 'always', prev: '*', next: 'return' },
        { blankLine: 'always', prev: ['const', 'let'], next: '*' },
        {
          blankLine: 'any',
          prev: ['const', 'let'],
          next: ['const', 'let'],
        },
        {
          blankLine: 'always',
          prev: ['multiline-const', 'multiline-let'],
          next: '*',
        },
        {
          blankLine: 'always',
          prev: '*',
          next: ['multiline-const', 'multiline-let'],
        },
      ],
      '@dword-design/import-alias/prefer-alias': [
        'error',
        {
          shouldReadTsConfig: true,
          shouldReadBabelConfig: false,
        },
      ],
      '@typescript-eslint/no-misused-promises': [
        'error',
        {
          checksVoidReturn: {
            attributes: false,
          },
        },
      ],
      '@typescript-eslint/only-throw-error': [
        'error',
        {
          allow: [
            {
              from: 'package',
              package: '@tanstack/router-core',
              name: 'Redirect',
            },
            {
              from: 'package',
              package: '@tanstack/router-core',
              name: 'NotFoundError',
            },
          ],
        },
      ],
      'import-x/no-absolute-path': 'error',
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_' },
      ],
      'import-x/no-named-as-default': 'off',
      'class-methods-use-this': 'error',
      '@typescript-eslint/explicit-module-boundary-types': 'off',
      'import-x/no-relative-parent-imports': 'off',
      'import-x/no-relative-packages': 'off',
      'import-x/order': [
        'error',
        {
          pathGroups: [
            { pattern: '@app/**', group: 'internal', position: 'after' },
            { pattern: '@pages/**', group: 'internal', position: 'after' },
            { pattern: '@widgets/**', group: 'internal', position: 'after' },
            { pattern: '@features/**', group: 'internal', position: 'after' },
            { pattern: '@entities/**', group: 'internal', position: 'after' },
            { pattern: '@shared/**', group: 'internal', position: 'after' },
          ],
          groups: [
            'builtin',
            'external',
            'index',
            'internal',
            'parent',
            'sibling',
            'object',
            'type',
          ],
          'newlines-between': 'always',
          pathGroupsExcludedImportTypes: ['builtin'],
          alphabetize: { order: 'asc', caseInsensitive: true },
        },
      ],
    },
  },
  {
    files: ['stories/**/*.{ts,tsx}', '.storybook/**/*.{ts,tsx}'],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
  {
    files: ['.storybook/**/*.{ts,tsx}'],
    languageOptions: { globals: { ...globals.browser, ...globals.node } },
  },
  {
    files: ['vite.config.ts'],
    languageOptions: { globals: globals.node },
  },
  {
    files: ['**/*.{ts,tsx}'],
    extends: [eslintPluginPrettierRecommended],
    rules: { curly: ['error', 'all'] },
  },
);
