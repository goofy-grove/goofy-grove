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
macro_rules! impl_new_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner_ty:ty);
        error: $error_type:ty;
        $(sanitize: $sanitize_fn:expr;)?
        validate: $validate_fn:expr;
    ) => {
        $(#[$meta])*
        $vis struct $name($inner_ty);

        impl $name {
            pub fn try_new(value: $inner_ty) -> Result<Self, $error_type> {
                $(
                    let value = ($sanitize_fn)(value);
                )?

                ($validate_fn)(&value)?;

                Ok(Self(value))
            }

            pub fn into_inner(self) -> $inner_ty {
                self.0
            }

            pub fn inner(&self) -> &$inner_ty {
                &self.0
            }
        }

        impl TryFrom<$inner_ty> for $name {
            type Error = $error_type;
            fn try_from(value: $inner_ty) -> Result<Self, Self::Error> {
                Self::try_new(value)
            }
        }
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner_ty:ty);
        sanitize: $sanitize_fn:expr;
    ) => {
        $(#[$meta])*
        $vis struct $name($inner_ty);

        impl $name {
            pub fn new(value: $inner_ty) -> Self {
                let value = ($sanitize_fn)(value);
                Self(value)
            }

            pub fn into_inner(self) -> $inner_ty {
                self.0
            }

            pub fn inner(&self) -> &$inner_ty {
                &self.0
            }
        }

        impl From<$inner_ty> for $name {
            fn from(value: $inner_ty) -> Self {
                Self::new(value)
            }
        }
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner_ty:ty);
    ) => {
        $(#[$meta])*
        $vis struct $name($inner_ty);

        impl $name {
            pub fn new(value: $inner_ty) -> Self {
                Self(value)
            }

            pub fn into_inner(self) -> $inner_ty {
                self.0
            }

            pub fn inner(&self) -> &$inner_ty {
                &self.0
            }
        }

        impl From<$inner_ty> for $name {
            fn from(value: $inner_ty) -> Self {
                Self::new(value)
            }
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
