use domain::prelude::User;

#[cfg(test)]
mod tests;

fn main() {
    println!(
        "Hello, world, {:?}!",
        User::new("1".to_string(), "John".to_string(), "password".to_string())
    );
}
