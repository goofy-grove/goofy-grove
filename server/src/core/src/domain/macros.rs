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

#[macro_export]
macro_rules! generate_entity {
    ($name:ident { $( $field:ident: $field_type:ty ),* }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $( $field: $field_type ),*
        }

        impl $name {
            pub fn new($( $field: $field_type ),*) -> Self {
                Self {
                    $( $field ),*
                }
            }

            $( pub fn $field(&self) -> &$field_type {
                &self.$field
            } )*
        }
    };
}

#[macro_export]
macro_rules! generate_event {
    ($($name:ident<$data_type:ty>),*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                id: EventId,
                data: EventData<$data_type>,
            }

            impl Event<$data_type> for $name {
                fn id(&self) -> &EventId {
                    &self.id
                }

                fn data(&self) -> &EventData<$data_type> {
                    &self.data
                }

                fn into_inner(self) -> $data_type {
                    self.data.0
                }
            }
        )*
    };
}
