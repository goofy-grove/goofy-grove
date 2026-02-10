use domain::prelude::{User, UserId, UserName, UserPassword};

fn main() {
    println!(
        "Hello, world, {:?}!",
        User::new(
            UserId::new("1".into()),
            UserName::new("John".into()),
            UserPassword::new("password".into())
        )
    );
}
