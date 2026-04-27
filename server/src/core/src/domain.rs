mod auth;
mod character;
mod event;
mod macros;
mod persona;
mod ports;
mod tokens;
mod user;

pub mod prelude {
    pub use crate::domain::user::entities::*;
    pub use crate::domain::user::ports::*;
    pub use crate::domain::user::rules::*;

    pub use crate::domain::auth::ports::*;
    pub use crate::domain::auth::rules::*;

    pub use crate::domain::persona::entities::*;
    pub use crate::domain::persona::ports::*;
    pub use crate::domain::persona::rules::*;

    pub use crate::domain::character::entities::*;
    pub use crate::domain::character::ports::*;
    pub use crate::domain::character::rules::*;

    pub use crate::domain::tokens::entities::*;
    pub use crate::domain::tokens::ports::*;
    pub use crate::domain::tokens::rules::*;

    pub use crate::domain::event::entities::*;
    pub use crate::domain::event::ports::*;
    pub use crate::domain::event::rules::*;

    pub use crate::domain::ports::*;
}
