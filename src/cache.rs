use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

/// The TypeCache is heavily inspired by the `state` crate and the way
/// the Rocket framework handles global and local state. You could say
/// we've immutably borrowed some ideas. *Rim-shot!*
///
/// Basically, we've got a hash that can store different types but
/// only one of each type. The type id is the key, and you need to
/// know what type you're asking for when you call `get()` to be able
/// to do anything with it:
///
/// ```rust
/// # use vial::TypeCache;
/// let cache = TypeCache::new();
///
/// cache.set::<String>("Hi friends".to_string());
/// assert_eq!(Some(&"Hi friends".to_string()), cache.get::<String>());
///
/// cache.set::<usize>(12345);
/// assert_eq!(Some(&12345), cache.get::<usize>());
/// ```
///
/// We use this in Vial for global state as well as local request
/// state, however this design is flawed as the local cache shouldn't
/// be forced into `Send + Sync`. These two will be separated in a
/// future release.
#[derive(Debug, Default)]
pub struct TypeCache {
    map: RefCell<HashMap<TypeId, *mut dyn Any>>,
    mutex: AtomicUsize,
}

impl TypeCache {
    /// Create a new, empty TypeCache.
    #[must_use] pub fn new() -> Self {
        Self::default()
    }

    /// TypeCache works like a regular hash map, but with types as
    /// keys. Meaning it can only store one of each type.
    /// Choose wisely.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.lock();
        let item = unsafe {
            self.map
                .borrow()
                .get(&TypeId::of::<T>())
                .map(|ptr| &*(*ptr as *const dyn Any).cast::<T>())
        };
        self.unlock();
        item
    }

    /// As long as your object is `Send + Sync + 'static`, TypeCache
    /// can store it.
    pub fn set<T: Send + Sync + 'static>(&self, v: T) {
        self.lock();
        let boxed = Box::into_raw(Box::new(v) as Box<dyn Any>);
        self.map.borrow_mut().insert(TypeId::of::<T>(), boxed);
        self.unlock();
    }

    #[inline(always)]
    fn lock(&self) {
        while self.mutex.compare_and_swap(0, 1, Ordering::SeqCst) != 0 {}
    }

    #[inline(always)]
    fn unlock(&self) {
        assert!(self.mutex.compare_and_swap(1, 0, Ordering::SeqCst) == 1);
    }
}
