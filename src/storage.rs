use {crate::Request, state, std::marker::PhantomData};

static mut STORAGE: Option<state::Container> = None;

pub fn init() {
    unsafe {
        STORAGE = Some(state::Container::new());
    }
}

pub fn get<T: Send + Sync + 'static>() -> &'static T {
    unsafe { STORAGE.as_ref().unwrap().get::<T>() }
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
