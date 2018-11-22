//! x86-64 paging, using page map level four, also known as PML4.

use address::PhysicalAddress;
use architecture::x86_64::Processor;

/// Page table.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PageTable(pub [PageTableEntry; 512]);

/// Page table entry.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTableEntry(u64);

/// Get the physical address of the first page table used by a processor.
/// Multiple processors may use the same page table, but this is not
/// necessarily the case.
pub fn first_page_table(processor: &Processor) -> PhysicalAddress {
    // The physical address of the first page table is stored in bits
    // [12 .. 64).
    let mask = !0u64 >> 12 & !0u64 << (64 - 64);
    PhysicalAddress((processor.cr3() & mask) as *mut u8)
}

impl PageTableEntry {
    /// Get the physical address stored in a page table entry. This address may
    /// point to another page table (in case this page table entry is in the
    /// first, second, or third page table), or to a page (in case this page
    /// table entry is in the fourth page table).
    pub fn physical_address(&self) -> PhysicalAddress {
        // The physical address is stored in bits [12 .. 52). It is to be
        // page-aligned, so after unsetting the remaining bits of the entry, a
        // 52-bit address has been formed.
        let mask = !0u64 >> 12 & !0u64 << (64 - 52);
        PhysicalAddress((self.0 & mask) as *mut u8)
    }
}
