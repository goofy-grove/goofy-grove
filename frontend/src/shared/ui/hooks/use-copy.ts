import { useState } from 'react';

export const useCopy = (value: string) => {
  const [isCopied, setIsCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(value);

      setIsCopied(true);
    } catch {
      // TODO: show notification
    }
  };

  return { isCopied, handleCopy };
};
