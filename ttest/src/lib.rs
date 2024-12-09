mod controller;
mod tests;
mod wrapper;

pub use wrapper::Wrapper;

use crate::controller::Controller;

pub fn when(title: &str, test: impl FnOnce()) {
    Controller::set_test_name(title);
    test();
}
