use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[derive(Debug)]
pub struct TypeCache {
    map: RefCell<HashMap<TypeId, *mut dyn Any>>,
    mutex: AtomicUsize,
}

impl TypeCache {
    pub fn new() -> TypeCache {
        TypeCache {
            map: RefCell::new(HashMap::new()),
            mutex: AtomicUsize::new(0),
        }
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.lock();
        let item = unsafe {
            self.map
                .borrow()
                .get(&TypeId::of::<T>())
                .map(|ptr| &*(*ptr as *const dyn Any as *const T))
        };
        self.unlock();
        item
    }

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
