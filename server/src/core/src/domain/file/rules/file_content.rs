use crate::impl_new_type;

impl_new_type!(
    // NOTE: cannot be cloned because it's too heavy
    #[derive(Debug)]
    pub struct FileContent(Vec<u8>);
);
