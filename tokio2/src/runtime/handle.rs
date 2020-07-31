use crate::runtime::{context, Spawner};
use crate::task::JoinHandle;
use std::future::Future;

#[derive(Clone)]
pub struct Handle {
    pub(super) spawner: Spawner,
}

impl Handle {
    pub fn enter<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        context::enter(self.clone(), f)
    }
}

impl Handle {
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.spawner.spawn(future)
    }
}
