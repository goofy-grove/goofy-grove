use crate::{domain::prelude::*, generate_entity};

generate_entity!(FileMeta {
    id: FileId,
    filename: Filename,
    owner_id: UserId,
    original_name: FileOriginalName,
    content_type: FileContentType,
    size: FileSize
});
