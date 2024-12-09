use crate::controller::Controller;
use log::info;
use std::panic::{catch_unwind, UnwindSafe};

pub struct TestCase<T: FnOnce() + UnwindSafe> {
    title: String,
    test: Option<T>,
}

impl<T: FnOnce() + UnwindSafe> TestCase<T> {
    pub(crate) fn new(title: &str, call: T) -> Self {
        Controller::set_test_name(title);

        Self {
            title: title.to_string(),
            test: Some(call),
        }
    }

    pub(crate) fn execute(&mut self) {
        let result = catch_unwind(self.test.take().unwrap());

        match result {
            Ok(_) => {
                info!("OK: {}", self.title);
            }
            Err(_) => {
                info!("FAILED: {}", self.title);
            }
        }
    }
}

impl<T: FnOnce() + UnwindSafe> Drop for TestCase<T> {
    fn drop(&mut self) {
        if self.test.is_some() {
            self.execute();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test_case::TestCase;

    fn some_method() -> TestCase<impl FnOnce()> {
        TestCase::new("some_test", || {
            dbg!("fuuu");
        })
    }

    #[test]
    fn test_call_on_drop() {
        let _call_on_drop = TestCase::new("some test", || {
            dbg!("0000");
        });

        dbg!("posle o");

        some_method();

        dbg!("posle fu1");

        some_method();

        dbg!("posle fu2");
    }
}
