import js from '@eslint/js';
import eslintPluginImport from 'eslint-plugin-import';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import { defineConfig, globalIgnores } from 'eslint/config';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import pluginRouter from '@tanstack/eslint-plugin-router'

export default defineConfig(
  globalIgnores(['dist']),
  js.configs.recommended,
  reactHooks.configs.flat['recommended-latest'],
  reactRefresh.configs.vite,
  eslintPluginPrettierRecommended,

  tseslint.configs.recommendedTypeChecked,
  eslintPluginImport.flatConfigs.recommended,
  eslintPluginImport.flatConfigs.typescript,
  eslintPluginImport.flatConfigs.react,

  pluginRouter.configs['flat/recommended'],

  {
    files: ['**/*.{ts,tsx}'],
    plugins: {
      '@typescript-eslint': tseslint.plugin,
    },
    languageOptions: {
      ecmaVersion: 2025,
      globals: globals.browser,
      parserOptions: {
        ecmaVersion: 2025,
        sourceType: 'module',
        ecmaFeatures: { jsx: true },
        projectService: true,
      },
    },
    rules: {
      'prettier/prettier': 'error',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-floating-promises': 'error',
      '@typescript-eslint/no-misused-promises': [
        'error',
        {
          checksVoidReturn: {
            attributes: false,
          },
        },
      ],
      "@typescript-eslint/only-throw-error": [
        "error",
        {
          "allow": [
            {
              "from": "package",
              "package": "@tanstack/router-core",
              "name": "Redirect"
            },
            {
              "from": "package",
              "package": "@tanstack/router-core",
              "name": "NotFoundError"
            }
          ]
        }
      ],
      '@typescript-eslint/no-unsafe-argument': 'error',
      '@typescript-eslint/explicit-module-boundary-types': 'error',
      'import/no-absolute-path': 'error',
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_' },
      ],
      'import/no-named-as-default': 'off',
      'class-methods-use-this': 'error',
      '@typescript-eslint/explicit-module-boundary-types': 'off',
      'import/order': [
        'error',
        {
          pathGroups: [
            { pattern: '@/**', group: 'internal', position: 'after' },
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
        },
      ],
    },
  },
);
