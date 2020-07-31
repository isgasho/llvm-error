use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

/// An owned handle to the task, tracked by ref count
pub(crate) struct Task<S: 'static> {
    _p: PhantomData<S>,
}

/// A task was notified
pub(crate) struct Notified<S: 'static>(Task<S>);

/// Task result sent back
pub(crate) type Result<T> = std::result::Result<T, JoinError>;

pub(crate) trait Schedule: Sync + Sized + 'static {
    /// Bind a task to the executor.
    ///
    /// Guaranteed to be called from the thread that called `poll` on the task.
    /// The returned `Schedule` instance is associated with the task and is used
    /// as `&self` in the other methods on this trait.
    fn bind(task: Task<Self>) -> Self;

    /// The task has completed work and is ready to be released. The scheduler
    /// is free to drop it whenever.
    ///
    /// If the scheduler can immediately release the task, it should return
    /// it as part of the function. This enables the task module to batch
    /// the ref-dec with other options.
    fn release(&self, task: &Task<Self>) -> Option<Task<Self>>;

    /// Schedule the task
    fn schedule(&self, task: Notified<Self>);

    /// Schedule the task to run in the near future, yielding the thread to
    /// other tasks.
    fn yield_now(&self, task: Notified<Self>) {
        self.schedule(task);
    }
}

/// Create a new task with an associated join handle
pub(crate) fn joinable<T, S>(task: T) -> (Notified<S>, JoinHandle<T::Output>)
where
    T: Future + Send + 'static,
    S: Schedule,
{
    let raw = RawTask::new::<_, S>(task);

    let task = Task { _p: PhantomData };

    let join = JoinHandle::new(raw);

    (Notified(task), join)
}
/// Task failed to execute to completion.
pub struct JoinError {}

pub struct JoinHandle<T> {
    _p: PhantomData<T>,
}

impl<T> JoinHandle<T> {
    fn new(_: RawTask) -> JoinHandle<T> {
        JoinHandle { _p: PhantomData }
    }
}

impl<T> Unpin for JoinHandle<T> {}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

/// Raw task handle
pub(super) struct RawTask {}

impl RawTask {
    pub(super) fn new<T, S>(_: T) -> RawTask
    where
        T: Future,
        S: Schedule,
    {
        RawTask {}
    }
}
