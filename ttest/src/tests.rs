#![cfg(test)]

use crate::{when, Wrapper};

#[derive(Default)]
struct User {
    age:  u32,
    name: String,
}

#[test]
fn test_test() {
    let mut user = Wrapper::<User>::default();

    when("User is default", || {
        assert_eq!(user.age, 0);
        assert_eq!(user.name, "");

        user.age = 4324234;
        user.name = "Prostaf".to_string();
    });

    when("User is 10 years old", || {
        user.age = 10;

        assert_eq!(user.age, 10);
        assert_eq!(user.name, "");
    });
}
