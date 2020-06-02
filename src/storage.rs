use {
    crate::{cache::TypeCache, Request},
    std::marker::PhantomData,
};

static mut STORAGE: Option<TypeCache> = None;

pub fn init() {
    unsafe {
        STORAGE = Some(TypeCache::new());
    }
}

pub fn get<T: Send + Sync + 'static>() -> &'static T {
    unsafe { STORAGE.as_ref().unwrap().get::<T>().as_ref().unwrap() }
}

pub fn set<T: Send + Sync + 'static>(o: T) {
    unsafe {
        STORAGE.as_ref().unwrap().set(o);
    }
}

pub struct State<T: Send + Sync + 'static> {
    request: Request,
    phantom: PhantomData<T>,
}

impl<T: Send + Sync + 'static> std::ops::Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        get::<T>()
    }
}

impl<T: Send + Sync + 'static> From<Request> for State<T> {
    fn from(request: Request) -> State<T> {
        State {
            request,
            phantom: PhantomData,
        }
    }
}
