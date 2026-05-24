use crate::{domain::prelude::*, generate_entity};

generate_entity!(
    #[derive(Debug)]
    CreateFileCommand {
        content_type: FileContentType,
        original_name: FileOriginalName,
        owner_id: UserId,
        pub content: FileContent
    }
);

generate_entity!(DeleteFileCommand { id: FileId });

generate_entity!(
    #[derive(Debug)]
    ReplaceFileCommand {
        id: FileId,
        content_type: FileContentType,
        content: FileContent
    }
);
