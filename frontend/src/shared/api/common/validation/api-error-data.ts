import * as z from 'zod';

function apiError<const T extends string>(
  code: T,
): z.ZodObject<{ code: z.ZodLiteral<T> }>;
function apiError<const T extends string, P extends z.ZodType>(
  code: T,
  params: P,
): z.ZodObject<{ code: z.ZodLiteral<T>; params: P }>;
function apiError<const T extends string, P extends z.ZodType>(
  code: T,
  params?: P,
) {
  const base = { code: z.literal(code) };

  return params ? z.object({ ...base, params }) : z.object(base);
}

export const ApiErrorDataSchema = z.discriminatedUnion('code', [
  apiError('auth_invalid_username'),
  apiError('auth_invalid_password'),
  apiError('auth_invalid_credentials'),
  apiError('auth_token_not_found'),
  apiError('auth_token_invalid'),
  apiError('auth_refresh_token_not_found'),
  apiError('auth_user_not_found'),
  apiError('auth_authentication_failed'),
  apiError('auth_token_generation_failed'),

  apiError('user_not_found'),
  apiError('user_avatar_not_found'),
  apiError('user_no_fields_provided'),
  apiError('user_update_failed'),

  apiError('persona_not_found'),
  apiError('character_access_denied'),
  apiError('persona_invalid_name'),
  apiError('persona_invalid_uid'),
  apiError('persona_avatar_not_found'),
  apiError('persona_no_fields_provided'),
  apiError('persona_access_denied'),
  apiError('persona_list_failed'),
  apiError('persona_create_failed'),
  apiError('persona_update_failed'),
  apiError('persona_delete_failed'),

  apiError('character_not_found'),
  apiError('character_avatar_not_found'),
  apiError('character_invalid_name'),
  apiError('character_invalid_uid'),
  apiError('character_no_fields_provided'),
  apiError('character_list_failed'),
  apiError('character_create_failed'),
  apiError('character_update_failed'),
  apiError('character_delete_failed'),

  apiError('file_not_found'),
  apiError('file_access_denied'),
  apiError(
    'file_invalid_size',
    z.object({
      max_size: z.number(),
      size: z.number(),
    }),
  ),
  apiError(
    'file_invalid_content_type',
    z.object({
      allowed: z.array(z.string()),
      content_type: z.string(),
    }),
  ),
  apiError('file_field_required'),
  apiError('file_invalid_original_name'),
  apiError('file_upload_policy_not_configured'),
  apiError('file_get_failed'),
  apiError('file_upload_failed'),
  apiError('file_response_build_failed'),
  apiError('file_invalid_status'),
  apiError('file_invalid_scope'),

  apiError('common_invalid_socket_id_header'),
  apiError('common_invalid_request_body'),
]);
