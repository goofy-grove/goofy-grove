import { useEffect, useMemo } from 'react';

export const useObjectUrl = (file?: File) => {
  const url = useMemo(
    () => (file ? URL.createObjectURL(file) : undefined),
    [file],
  );

  useEffect(
    () => () => {
      if (url) {
        URL.revokeObjectURL(url);
      }
    },
    [url],
  );

  return url;
};
