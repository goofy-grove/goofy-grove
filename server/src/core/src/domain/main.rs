use domain::prelude::{User, UserId, UserPassword, Username};

fn main() {
    println!(
        "Hello, world, {:?}!",
        User {
            uid: UserId::new("1".into()),
            name: Username::new("John".into()),
            password: UserPassword::new("password".into()),
        }
    );
}
