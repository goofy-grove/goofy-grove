import { useEffect, useState } from 'react';

import { api } from '@shared/api';

export const useFileUrl = (fileUid: string | null | undefined) => {
  const [cache, setCache] = useState<{ fileUid: string; url: string } | null>(
    null,
  );

  useEffect(() => {
    if (!fileUid) {
      return;
    }

    let revoked = false;
    let objectUrl: string | null = null;

    void api.files
      .getBlob(fileUid)
      .then((blob) => {
        if (revoked) {
          return;
        }

        objectUrl = URL.createObjectURL(blob);
        setCache({ fileUid, url: objectUrl });
      })
      .catch(() => {
        if (!revoked) {
          setCache(null);
        }
      });

    return () => {
      revoked = true;

      if (objectUrl) {
        URL.revokeObjectURL(objectUrl);
      }
    };
  }, [fileUid]);

  if (!fileUid) {
    return null;
  }

  return cache?.fileUid === fileUid ? cache.url : null;
};
