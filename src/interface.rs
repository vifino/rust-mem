////////////////
// INTERFACES //
////////////////

use errors::MemError;

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
    fn set(&mut self, Addr, Byte) -> Result<(), MemError>;

    /// Get a byte at address.
    /// Returns `Ok(X)` on success, where X will be the byte.
    fn get(&self, Addr) -> Result<Byte, MemError>;

    /// Delete data at address.
    /// May allow the block to efficiently delete it, marking it as unused.
    /// This could allow the block to do wear leveling, for example.
    fn delete(&mut self, from: Addr, to: Addr) -> Result<(), MemError> {
        if from == to {
            return self.set(from, 0);
        }
        for i in from..to {
            self.set(i, 0)?;
        }
        self.set(to, 0)
    }

    /// Flush writes out.
    /// In case it does any form of caching, calling this method
    /// ensures it has written all the data it needs to write.
    /// Or, well, it fails otherwise.
    fn flush(&mut self) -> Result<(), MemError> {
        Ok(())
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
