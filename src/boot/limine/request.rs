use core::cell::UnsafeCell;
use core::ops::Deref;

#[repr(C)]
pub struct LimineResponse<T> {
    pub revision: u64,
    pub data: T,
}

impl<T> LimineResponse<T> {
    pub fn new(revision: u64, data: T) -> Self {
        Self { revision, data }
    }
}

#[repr(C)]
pub struct LimineRequest<T> {
    pub id: [u64; 4],
    pub revision: u64,
    pub response: UnsafeCell<*mut LimineResponse<T>>,
}

impl<T> LimineRequest<T> {
    pub const fn new(id: [u64; 4], revision: u64) -> Self {
        Self {
            id,
            revision,
            response: UnsafeCell::new(core::ptr::null_mut()),
        }
    }

    pub fn get(&self) -> Option<&'static LimineResponse<T>> {
        unsafe { self.response.get().read_volatile().as_ref() }
    }
}

unsafe impl<T> Sync for LimineRequest<T> {}

impl<T> Deref for LimineResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
