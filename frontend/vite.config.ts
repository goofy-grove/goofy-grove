import path from 'path';

import { tanstackRouter } from '@tanstack/router-plugin/vite';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    tanstackRouter({
      target: 'react',
      autoCodeSplitting: true,
      generatedRouteTree: './src/app/lib/@tanstack/router/route-tree.gen.ts',
      routesDirectory: './src/app/lib/@tanstack/router/routes',
    }),
    react(),
  ],
  resolve: {
    alias: {
      '@shared': path.resolve(__dirname, './src/shared'),
      '@entities': path.resolve(__dirname, './src/entities'),
      '@features': path.resolve(__dirname, './src/features'),
      '@widgets': path.resolve(__dirname, './src/widgets'),
      '@pages': path.resolve(__dirname, './src/pages'),
      '@app': path.resolve(__dirname, './src/app'),
    },
  },
});
