use crate::loom::{
    cell::UnsafeCell,
    sync::atomic::{AtomicPtr, AtomicUsize},
};

use std::ptr;

/// A block in a linked list.
///
/// Each block in the list can hold up to `BLOCK_CAP` messages.
#[allow(dead_code)]
pub(crate) struct Block<T> {
    /// The start index of this block.
    ///
    /// Slots in this block have indices in `start_index .. start_index + BLOCK_CAP`.
    start_index: usize,

    /// The next block in the linked list.
    next: AtomicPtr<Block<T>>,

    /// Bitfield tracking slots that are ready to have their values consumed.
    ready_slots: AtomicUsize,

    /// The observed `tail_position` value *after* the block has been passed by
    /// `block_tail`.
    observed_tail_position: UnsafeCell<usize>,
}

#[allow(dead_code)]
pub(crate) enum Read<T> {
    Value(T),
    Closed,
}

impl<T> Block<T> {
    pub(crate) fn new(start_index: usize) -> Block<T> {
        Block {
            // The absolute index in the channel of the first slot in the block.
            start_index,

            // Pointer to the next block in the linked list.
            next: AtomicPtr::new(ptr::null_mut()),

            ready_slots: AtomicUsize::new(0),

            observed_tail_position: UnsafeCell::new(0),
        }
    }
}
