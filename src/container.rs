use std::{
    any::Any,
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

static CONTAINER: OnceLock<Mutex<Container>> = OnceLock::new();

pub fn init_container() {
    CONTAINER.get_or_init(|| Mutex::new(Container::new()));
}

#[allow(dead_code)]
pub fn container() -> &'static Mutex<Container> {
    CONTAINER.get().expect("Container not initialized")
}

pub struct Container {
    items: HashMap<String, Box<dyn Any + Send>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn bind<T: 'static + Send>(&mut self, key: &str, value: T) {
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

    #[test]
    fn test_multiple_inits() {
        init_container();

        let mut container = container().lock().unwrap();

        container.bind("number_ten", 10);

        init_container();

        assert_eq!(container.resolve("number_ten"), Some(&10));
    }
}
