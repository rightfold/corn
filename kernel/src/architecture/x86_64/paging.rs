//! x86-64 paging, using page map level four, also known as PML4.

use address::PhysicalAddress;

/// Page table entry.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    /// Get the physical address stored in a page table entry. This address may
    /// point to another page table (in case this page table entry is in the
    /// first, second, or third page table), or to a page (in case this page
    /// table entry is in the fourth page table).
    pub fn page_table_address(&self) -> PhysicalAddress {
        // The physical address is stored in bits [12 .. 52). It is to be
        // page-aligned, so after unsetting the remaining bits of the entry, a
        // 52-bit address has been formed.
        let mask = !0u64 >> 12 & !0u64 << (64 - 52);
        PhysicalAddress((self.0 & mask) as *mut u8)
    }
}
