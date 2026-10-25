export default {
  'frontend/{src,stories,.storybook}/**/*.{ts,tsx}':
    'npm --prefix frontend run check:lint:staged:fix --',
  'frontend/{src,stories,.storybook}/**/*.scss': [
    'npm --prefix frontend run check:styles:staged:fix --',
    'npm --prefix frontend run check:format:staged:fix --',
  ],
  'frontend/**/*': () => [
    'npm --prefix frontend run check:types',
    'npm --prefix frontend run check:architecture',
    'npm --prefix frontend run check:storybook',
  ],
  'server/**/*.{rs,toml,lock}': () => 'npm run check:rust:fix',
};
