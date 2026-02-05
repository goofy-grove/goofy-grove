#[macro_export]
macro_rules! impl_as_domain_newtype {
    ($($name:ident -> $value_type:ty),*) => {
        $(
            impl $name {
                pub fn new(value: $value_type) -> Self {
                    Self(value)
                }

                pub fn value(&self) -> &$value_type {
                    &self.0
                }
            }

            impl From<$value_type> for $name {
                fn from(value: $value_type) -> Self {
                    Self::new(value)
                }
            }
        )*
    };
}
