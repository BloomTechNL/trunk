use std::cell::RefCell;
use std::sync::OnceLock;

pub struct Slot<T> {
    value: OnceLock<T>,
    factory: RefCell<Option<Box<dyn FnOnce() -> T>>>,
}

impl<T> Slot<T> {
    pub fn register<F: FnOnce() -> T + 'static>(factory: F) -> Self {
        Self {
            value: OnceLock::new(),
            factory: RefCell::new(Some(Box::new(factory))),
        }
    }

    /// Returns a reference to the dependency value.
    ///
    /// On first call, runs the registered factory closure and caches the result.
    /// Subsequent calls return the cached value.
    ///
    /// # Panics
    ///
    /// Panics if the factory was somehow already consumed before the first call.
    pub fn resolve(&self) -> &T {
        self.value.get_or_init(|| {
            self.factory
                .borrow_mut()
                .take()
                .expect("factory already consumed before initialization")()
        })
    }
}
