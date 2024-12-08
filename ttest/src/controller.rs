use std::sync::Mutex;

static TEST_NAME: Mutex<String> = Mutex::new(String::new());

pub(crate) struct Controller {}

impl Controller {
    pub(crate) fn _set_test_name(name: &str) {
        *TEST_NAME.lock().unwrap() = name.into();
    }

    pub(crate) fn test_name() -> String {
        TEST_NAME.lock().unwrap().clone()
    }
}
