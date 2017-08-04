// Readonly middleware
use interface::*;
use errors::*;

/// Read-only wrapper for MemoryBlocks.
/// Raises an error if you try to write to it.
/// Reading simply gets passed through.
/// Useful in cases where you dynamically dispatch between different blocks.
pub struct ReadOnly<M: MemoryBlock> {
    mem: Box<M>,
}

impl<M: MemoryBlock> MemoryMiddlewareCreator<M> for ReadOnly<M> {
    fn new(mem: Box<M>) -> Self {
        ReadOnly {
            mem: mem
        }
    }
}

impl<M: MemoryBlock> MemoryBlock for ReadOnly<M> {
    fn get_size(&self) -> Addr {
        self.mem.get_size()
    }

    fn get(&self, addr: Addr) -> Result<Byte, Error> {
        self.mem.get(addr)
    }

    fn set(&mut self, addr: Addr, _: Byte) -> Result<(), Error> {
        bail!(ErrorKind::ReadOnly(addr, true))
    }
}
