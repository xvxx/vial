use crate::TypeCache;

static mut STORAGE: Option<TypeCache> = None;

pub fn init() {
    unsafe {
        STORAGE = Some(TypeCache::new());
    }
}

#[must_use] pub fn get<T: Send + Sync + 'static>() -> &'static T {
    unsafe { STORAGE.as_ref().unwrap().get::<T>().as_ref().unwrap() }
}

pub fn set<T: Send + Sync + 'static>(o: T) {
    unsafe {
        STORAGE.as_ref().unwrap().set(o);
    }
}
