#![cfg(test)]

use crate::{when, Wrapper};
use simple_logger::SimpleLogger;

#[derive(Default)]
struct User {
    age: u32,
    name: String,
}

#[test]
fn test_test() {
    SimpleLogger::new().init().unwrap();

    let mut user = Wrapper::<User>::default();

    when("User is default", move || {
        assert_eq!(user.age, 0);
        assert_eq!(user.name, "");

        user.age = 4324234;
        user.name = "Prostaf".to_string();
    });

    when("User is 10 years old", move || {
        user.age = 10;

        assert_eq!(user.age, 10);
        assert_eq!(user.name, "");
    });

    when("Test has failed", move || {
        user.age = 100;

        assert_eq!(user.age, 10);
        assert_eq!(user.name, "");
    });
}
