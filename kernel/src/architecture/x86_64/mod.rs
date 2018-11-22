//! x86-64-specific code.

pub mod paging;

/// Processor state.
pub struct Processor;

impl Processor {
    /// The value inside the CR3 register.
    pub fn cr3(&self) -> u64 {
        unimplemented!();
    }
}
