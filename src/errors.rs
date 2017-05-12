//! Error module
//! y u do this???
//!
//! This should make errors useable! Yay!
use std::fmt;
use std::error::Error as StdError;

use interface::*;

#[derive(Debug)]
pub enum MemError {
    TooBig { given: Addr, max: Addr },
    TooSmall { given: Addr, min: Addr },
    InvalidAddr { addr: Addr },
    ReadOnly { at: Addr, globally: bool },
    WriteOnly { at: Addr, globally: bool },
    UnalignedAccess { at: Addr, alignment: Addr },
    NoData { at: Addr },
    InvalidData { at: Addr },
    HardwareFault { at: Addr, reason: String },
    Uninitialized { at: Addr },
    NotApplicable { at: Addr },
    NotImplemented,
}

impl fmt::Display for MemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MemError::TooBig { given, max } => {
                write!(f, "MemoryBlock Error: {:#X} is too big, {:#X} is the maximum", given, max)
            }
            MemError::TooSmall { given, min } => {
                write!(f, "MemoryBlock Error: {:#X} is too small, {:#X} is the minimum", given, min)
            }
            MemError::InvalidAddr { addr } => {
                write!(f, "MemoryBlock Error: Invalid address: {:#X}", addr)
            }
            MemError::ReadOnly { at, globally: g } => {
                let errtext = if g {
                    "Globally read only"
                } else {
                    "Section read only"
                };
                write!(f, "MemoryBlock Error: {} @ {:#X} ", errtext, at)
            }
            MemError::WriteOnly { at, globally: g } => {
                let errtext = if g {
                    "Globally write only"
                } else {
                    "Section write only"
                };
                write!(f, "MemoryBlock Error: {} @ {:#X} ", errtext, at)
            }
            MemError::UnalignedAccess { at, alignment: a } => {
                write!(f, "MemoryBlock Error: {:#X}: Unaligned access, needs {} byte alignment", at, a)
            }
            MemError::NoData { at } => write!(f, "MemoryBlock Error: {:#X}: No data", at),
            MemError::InvalidData { at } => write!(f, "MemoryBlock Error: {:#X}: Invalid data", at),
            MemError::HardwareFault { at, ref reason } => {
                write!(f, "MemoryBlock Error: Hardware fault occured @ {:#X}: {}", at, reason)
            }
            MemError::Uninitialized { at } => {
                write!(f, "MemoryBlock Error: {:#X} is uninitialized", at)
            }
            MemError::NotImplemented => {
                write!(f, "MemoryBlock Error: not implemented")
            }
            MemError::NotApplicable { at } => {
                write!(f, "MemoryBlock Error: action not applicable @ {:#X}", at)
            }
        }
    }
}

impl StdError for MemError {
    fn description(&self) -> &str {
        match *self {
            MemError::TooBig { given: _, max: _ } => "too big",
            MemError::TooSmall { given: _, min: _ } => "too small",
            MemError::InvalidAddr { addr: _ } => "invalid address",
            MemError::ReadOnly { at: _, globally: _ } => "memory read only",
            MemError::WriteOnly { at: _, globally: _ } => "memory write only",
            MemError::UnalignedAccess { at: _, alignment: _ } => "unaligned access to memory",
            MemError::NoData { at: _ } => "no data",
            MemError::InvalidData { at: _ } => "invalid data",
            MemError::HardwareFault { at: _, reason: _ } => "hardware fault",
            MemError::Uninitialized { at: _ } => "uninitialized",
            MemError::NotImplemented => "not implemented",
            MemError::NotApplicable { at: _ } => "not applicable",
        }
    }
}
