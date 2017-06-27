////////////////
// INTERFACES //
////////////////

extern crate byteorder;
use self::byteorder::{ByteOrder, BigEndian, LittleEndian};

use errors::*;

/// Address type/size.
/// Just a simple alias to usize for easier-to-read code. (In my opinion, obviously.)
/// Originally, I wanted to make it use generics,
/// but that got messy really quick.
pub type Addr = usize;

/// Byte.
/// Convenience alias.
pub type Byte = u8;

/// Simple trait for a finite memory block.
pub trait MemoryBlock {
    /// Get the block's accessible size.
    /// Simply the highest address,
    /// NOT the number of addresses.
    fn get_size(&self) -> Addr;

    /// Set a byte at address.
    fn set(&mut self, Addr, Byte) -> Result<(), Error>;

    /// Get a byte at address.
    /// Returns `Ok(X)` on success, where X will be the byte.
    fn get(&self, Addr) -> Result<Byte, Error>;

    /// Delete data at address.
    /// May allow the block to efficiently delete it, marking it as unused.
    /// This could allow the block to do wear leveling, for example.
    fn delete(&mut self, from: Addr, to: Addr) -> Result<(), Error> {
        if from == to {
            self.set(from, 0).chain_err(|| format!("failure to delete {:#X}", from))?;
        }
        for i in from..to {
            self.set(i, 0).chain_err(|| format!("failure to delete byte {} in {:#X}-{:#X}", i-from, from, to))?;
        }
        self.set(to, 0).chain_err(|| format!("failure to delete {:#X}", to))
    }

    /// Flush writes out.
    /// In case it does any form of caching, calling this method
    /// ensures it has written all the data it needs to write.
    /// Or, well, it fails otherwise.
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

/// Subtrait for 32 bit big-endian interface
pub trait MemoryBlock32be: MemoryBlock {
    /// Set a 32 bit value (4 bytes) in Big Endian format.
    fn set32be(&mut self, addr: Addr, data: u32) -> Result<(), Error> {
        let mut bytes = [0; 4];
        BigEndian::write_u32(&mut bytes, data);
        for i in 0..4 {
            self.set(addr + i, bytes[i]).chain_err(|| ErrorKind::EndianessHelperFail("set32be", addr, i))?;
        }
        Ok(())
    }
    /// Get a 32 bit value (4 bytes) stored in Big Endian format.
    fn get32be(&self, addr: Addr) -> Result<u32, Error> {
        let mut bytes = [0; 4];
        for i in 0..4 {
            bytes[i] = self.get(addr + i).chain_err(|| ErrorKind::EndianessHelperFail("get32be", addr, i))?;
        }
        Ok(BigEndian::read_u32(&bytes))
    }
}
/// Subtrait for 32 bit little-endian interface
pub trait MemoryBlock32le: MemoryBlock {
    /// Set a 32 bit value (4 bytes) in Little Endian format.
    fn set32le(&mut self, addr: Addr, data: u32) -> Result<(), Error> {
        let mut bytes = [0; 4];
        LittleEndian::write_u32(&mut bytes, data);
        for i in 0..4 {
            self.set(addr + i, bytes[i]).chain_err(|| ErrorKind::EndianessHelperFail("set32le", addr, i))?;
        }
        Ok(())
    }
    /// Get a 32 bit value (4 bytes) stored in Little Endian format.
    fn get32le(&self, addr: Addr) -> Result<u32, Error> {
        let mut bytes = [0; 4];
        for i in 0..4 {
            bytes[i] = self.get(addr + i).chain_err(|| ErrorKind::EndianessHelperFail("get32le", addr, i))?;
        }
        Ok(LittleEndian::read_u32(&bytes))
    }
}

/// Simple `new(size)` trait for backends.
// For convenience, mostly.
pub trait MemoryCreator<T> {
    /// Returns a `MemoryBlock` of size `size`.
    /// `size` being the highest address,
    /// NOT number of addresses.
    fn new(size: Addr) -> T;
}

/// Trait for a new function that wraps
/// another memory block, returning one itself.
///
/// Useful for invalid access checkers, debuggers, etc....
pub trait MemoryMiddlewareCreator<T> {
    fn new<M: MemoryBlock>(Box<M>) -> T;
}
