export default {
  'src/**/*.*': (stagedFiles) => {
    const stagedFilesString = stagedFiles.join(' ');

    return [
      `npm run lint ${stagedFilesString}`,
      'npm run check:types',
    ];
  },
}
