import type { ApiErrorData } from './validation';
import type { TFunction } from 'i18next';

export class ApiRequestError extends Error {
  readonly data: ApiErrorData;

  constructor(data: ApiErrorData) {
    super(data.code);
    this.name = 'ApiRequestError';
    this.data = data;
  }
}

const formatBytes = (bytes: number): string => {
  const mb = bytes / (1024 * 1024);

  if (mb >= 1) {
    const rounded = Number.isInteger(mb) ? mb.toString() : mb.toFixed(1);

    return `${rounded} MB`;
  }

  const kb = bytes / 1024;

  if (kb >= 1) {
    const rounded = Number.isInteger(kb) ? kb.toString() : kb.toFixed(1);

    return `${rounded} KB`;
  }

  return `${bytes} B`;
};

export const getApiErrorMessage = (error: unknown, t: TFunction): string => {
  if (!(error instanceof ApiRequestError)) {
    return t('common.errors.save_failed');
  }

  switch (error.data.code) {
    case 'file_invalid_size':
      return t('common.errors.file_too_large', {
        maxSize: formatBytes(error.data.params.max_size),
      });
    case 'file_invalid_content_type':
      return t('common.errors.file_invalid_type');
    default:
      return t('common.errors.save_failed');
  }
};
