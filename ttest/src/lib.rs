mod controller;
mod test_case;
mod tests;
mod wrapper;

use std::panic::UnwindSafe;
pub use wrapper::Wrapper;

use crate::test_case::TestCase;

pub fn when(title: &str, test: impl FnOnce() + UnwindSafe) -> TestCase<impl FnOnce()> {
    TestCase::new(title, test)
}
