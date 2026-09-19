import { fileURLToPath } from 'node:url';

import { defineConfig } from 'vite';

export default defineConfig({
  resolve: {
    alias: Object.fromEntries(
      ['shared', 'entities', 'features', 'widgets', 'pages', 'app'].map(
        (layer) => [
          `@${layer}`,
          fileURLToPath(new URL(`../src/${layer}`, import.meta.url)),
        ],
      ),
    ),
  },
});
