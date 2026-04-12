import importAliases from '@dword-design/eslint-plugin-import-alias';
import js from '@eslint/js';
import pluginRouter from '@tanstack/eslint-plugin-router';
import tsParser from '@typescript-eslint/parser';
import { createTypeScriptImportResolver } from 'eslint-import-resolver-typescript';
import { importX } from 'eslint-plugin-import-x';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import { defineConfig, globalIgnores } from 'eslint/config';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default defineConfig(
  globalIgnores(['dist']),
  js.configs.recommended,
  reactHooks.configs.flat['recommended-latest'],
  reactRefresh.configs.vite,
  eslintPluginPrettierRecommended,

  tseslint.configs.recommendedTypeChecked,
  importX.flatConfigs.recommended,
  importX.flatConfigs.typescript,
  importX.flatConfigs.react,

  pluginRouter.configs['flat/recommended'],

  // {
  //   settings: {
  //     'import-x/resolver-next': [
  //       createTypeScriptImportResolver({
  //         alwaysTryTypes: true,
  //         project: './',
  //         tsconfig: './tsconfig.json',
  //       }),
  //     ]
  //   },
  // },

  importAliases.configs.recommended,

  {
    files: ['**/*.{ts,tsx}'],
    plugins: {
      '@typescript-eslint': tseslint.plugin,
      '@dword-design/import-alias/prefer-alias': {
        shouldReadTsConfig: false,
      },
    },
    languageOptions: {
      ecmaVersion: 2023,
      globals: globals.browser,
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 2023,
        sourceType: 'module',
        ecmaFeatures: { jsx: true },
        projectService: true,
      },
    },
    settings: {
      'import-x/resolver-next': [
        createTypeScriptImportResolver({
          alwaysTryTypes: true,
          project: './',
          tsconfig: './tsconfig.eslint.json',
        }),
      ]
    },
    rules: {
      'prettier/prettier': 'error',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-floating-promises': 'error',
      '@dword-design/import-alias/prefer-alias': [
        'error',
        {
          alias: {
            "@shared/*": "./src/shared/*",
            "@entities/*": "./src/entities/*",
            "@features/*": "./src/features/*",
            "@widgets/*": "./src/widgets/*",
            "@pages/*": "./src/pages/*",
            "@app/*": "./src/app/*"
          },
        }
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
      '@typescript-eslint/no-unsafe-argument': 'error',
      '@typescript-eslint/explicit-module-boundary-types': 'error',
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
);
