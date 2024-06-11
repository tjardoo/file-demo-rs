use std::{any::Any, collections::HashMap, sync::Mutex};

static mut CONTAINER: Option<Mutex<Container>> = None;

#[allow(dead_code)]
pub fn init_container() {
    unsafe {
        CONTAINER = Some(Mutex::new(Container::new()));
    }
}

#[allow(dead_code)]
pub fn container() -> &'static Mutex<Container> {
    unsafe { CONTAINER.as_ref().expect("Container not initialized") }
}

pub struct Container {
    #[allow(dead_code)]
    items: HashMap<String, Box<dyn Any>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn bind<T: 'static>(&mut self, key: &str, value: T) {
        self.items.insert(key.to_string(), Box::new(value));
    }

    #[allow(dead_code)]
    pub fn resolve<T: 'static>(&self, key: &str) -> Option<&T> {
        self.items
            .get(key)
            .and_then(|item| item.downcast_ref::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_without_init_container() {
        let _container = container().lock().unwrap();
    }

    #[test]
    fn test_bind() {
        init_container();

        let mut container = container().lock().unwrap();

        #[derive(Debug, PartialEq)]
        enum TestEnum {
            Test,
        }

        container.bind("random_number", 2024);
        container.bind("random_string", "Hello, World!");
        container.bind("random_enum", TestEnum::Test);

        assert_eq!(container.resolve("random_number"), Some(&2024));
        assert_eq!(container.resolve("random_string"), Some(&"Hello, World!"));
        assert_eq!(container.resolve("random_enum"), Some(&TestEnum::Test));
    }

    #[test]
    fn test_resolve_non_existing_key() {
        init_container();

        let container = container().lock().unwrap();

        assert_eq!(container.resolve::<String>("something_else"), None);
    }
}
