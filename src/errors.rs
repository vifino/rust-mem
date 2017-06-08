//! Error module
//! y u do this???
//!
//! This should make errors useable! Yay!
use interface::*;

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    errors {
        TooBig(given: Addr, max: Addr) {
            description("given address too big")
            display("MemoryBlock Error: {:#X} is too big, {:#X} is the maximum", given, max)
        }
        TooSmall(given: Addr, min: Addr) {
            description("given address too small")
            display("MemoryBlock Error: {:#X} is too small, {:#X} is the minimum", given, min)
        }
        InvalidAddr(addr: Addr) {
            description("invalid address")
            display("MemoryBlock Error: Invalid address: {:#X}", addr)
        }
        ReadOnly(at: Addr, _globally: bool) {
            description("read only")
            display("MemoryBlock Error: Read only at {:#X}", at)
        }
        WriteOnly(at: Addr, _globally: bool) {
            description("write only")
            display("MemoryBlock Error: Write only at {:#X}", at)
        }
        UnalignedAccess(given: Addr, alignment: Addr) {
            description("unaligned access")
            display("MemoryBlock Error: Unaligned access at {:#X}, need {} byte alignment", given, alignment)
        }
        NoData(at: Addr) {
            description("no data")
            display("MemoryBlock Error: No data available at {:#X}", at)
        }
        InvalidData(at: Addr) {
            description("invalid data")
            display("MemoryBlock Error: {:#X}: Invalid data", at)
        }
        HardwareFault(at: Addr, reason: &'static str) {
            description("hardware fault")
            display("MemoryBlock Error: Hardware Fault at {:#X}: {}", at, reason)
        }
        Uninitialized(at: Addr) {
            description("uninitialized")
            display("MemoryBlock Error: {:#X} is uninitialized", at)
        }
        NotImplemented {
            description("not implemented")
            display("MemoryBlock Error: Not implemented")
        }
        NotApplicable(at: Addr) {
            description("not applicable")
            display("MemoryBlock Error: Action not applicable at {:#X}", at)
        }
    }
}
