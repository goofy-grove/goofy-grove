use domain::prelude::{User, UserId, UserPassword, Username};

fn main() {
    println!(
        "Hello, world, {:?}!",
        User::new(
            UserId::new("1".into()),
            Username::new("John".into()),
            UserPassword::new("password".into())
        )
    );
}
