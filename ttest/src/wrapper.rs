use std::{
    collections::BTreeMap,
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use fake::{Fake, Faker};

use crate::controller::Controller;

const MAX_WRAPPED_SIZE: usize = 64;

static BUFFERS: Mutex<BTreeMap<String, [u8; MAX_WRAPPED_SIZE]>> = Mutex::new(BTreeMap::new());

pub struct Wrapper<T: Default> {
    wrapped_id: u64,
    _p:         PhantomData<T>,
}

impl<T: Default> Default for Wrapper<T> {
    fn default() -> Self {
        Self::assert_size();
        Self {
            wrapped_id: Faker.fake(),
            _p:         PhantomData,
        }
    }
}

impl<T: Default> Wrapper<T> {
    pub fn new(_val: T) -> Self {
        Self::assert_size();
        Self {
            wrapped_id: Faker.fake(),
            _p:         PhantomData,
        }
    }
}

impl<T: Default> Wrapper<T> {
    fn assert_size() {
        let size = size_of::<T>();
        assert!(
            size <= MAX_WRAPPED_SIZE,
            "This type is too big for Wrapper! Max: {MAX_WRAPPED_SIZE}, T: {size}"
        );
    }

    fn local_id(&self) -> String {
        format!("{}_{}", self.wrapped_id, Controller::test_name())
    }

    fn buff_with_id<'a>(id: String) -> &'a mut Option<T> {
        let mut binding = BUFFERS.lock().unwrap();
        let buff = binding.entry(id).or_insert_with(|| {
            let mut buffer = [0; MAX_WRAPPED_SIZE];

            // Suggested fix is ugly.
            #[allow(clippy::transmute_ptr_to_ptr)]
            let rf: &mut Option<T> = unsafe { transmute(&mut buffer) };

            *rf = None;

            buffer
        });

        #[allow(clippy::transmute_ptr_to_ptr)]
        unsafe {
            transmute(buff)
        }
    }

    fn local_buff(&self) -> &mut Option<T> {
        Self::buff_with_id(self.local_id())
    }
}

impl<T: Default> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let buff = self.local_buff();

        if buff.is_none() {
            *buff = T::default().into();
        }

        buff.as_ref().unwrap()
    }
}

impl<T: Default> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let buff = self.local_buff();

        if buff.is_none() {
            *buff = T::default().into();
        }

        buff.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::wrapper::Wrapper;

    #[derive(Debug)]
    struct User {
        id:   u64,
        age:  u32,
        name: String,
    }

    impl Default for User {
        fn default() -> Self {
            User {
                id:   15,
                age:  32,
                name: "Sokol".to_string(),
            }
        }
    }

    #[test]
    fn default_wrapper() {
        let user = Wrapper::<User>::default();

        assert_eq!(user.id, 15);
        assert_eq!(user.age, 32);
        assert_eq!(user.name, "Sokol");
    }
}
