import { useEffect, useState } from 'react';

import { api } from '@shared/api';

const fileUrlCache = new Map<string, Promise<string>>();

const getFileUrl = (fileUid: string) => {
  const cachedUrl = fileUrlCache.get(fileUid);

  if (cachedUrl) return cachedUrl;

  const request = api.files
    .getBlob(fileUid)
    .then((blob) => URL.createObjectURL(blob))
    .catch((error: unknown) => {
      fileUrlCache.delete(fileUid);
      throw error;
    });

  fileUrlCache.set(fileUid, request);

  return request;
};

export const useFileUrl = (fileUid: string | null | undefined) => {
  const [cache, setCache] = useState<{ fileUid: string; url: string } | null>(
    null,
  );

  useEffect(() => {
    if (!fileUid) {
      return;
    }

    let revoked = false;

    void getFileUrl(fileUid)
      .then((url) => {
        if (revoked) {
          return;
        }

        setCache({ fileUid, url });
      })
      .catch(() => {
        if (!revoked) {
          setCache(null);
        }
      });

    return () => {
      revoked = true;
    };
  }, [fileUid]);

  if (!fileUid) {
    return null;
  }

  return cache?.fileUid === fileUid ? cache.url : null;
};
