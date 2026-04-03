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
});
