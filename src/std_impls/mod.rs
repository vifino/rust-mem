// Basic mem impls on STD basis.

use interface::*;
use errors::*;

/// Simple `Vec`-based memory block.
/// Should suffice basic RAM needs.
pub struct MemVector {
    size: Addr,
    mem: Vec<Byte>,
}

impl MemoryCreator<MemVector> for MemVector {
    fn new(size: Addr) -> MemVector {
        // Allocate a vector, filling it with zeros.
        let mut vec: Vec<Byte> = Vec::new();
        vec.resize(size + 1, 0);

        MemVector {
            size: size,
            mem: vec,
        }
    }
}

impl MemoryBlock for MemVector {
    fn get_size(&self) -> Addr {
        self.size
    }

    fn get(&self, addr: Addr) -> Result<Byte, Error> {
        match self.mem.get(addr) {
            Some(b) => Ok(*b),
            None => bail!(ErrorKind::NoData(addr)),
        }
    }

    fn set(&mut self, addr: Addr, byte: Byte) -> Result<(), Error> {
        if addr > self.size {
            bail!(ErrorKind::TooBig(addr, self.size));
        }
        self.mem[addr] = byte;
        Ok(())
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memvec_works_basic() {
        let mut mem = Box::new(MemVector::new(0xFF));
        mem.set(0x00, 101).unwrap();
        mem.flush().unwrap();
        assert_eq!(101, mem.get(0x00).unwrap());
    }

    #[test]
    fn memvec_works_all() {
        let mut mem = Box::new(MemVector::new(0xFF));
        let sz = mem.get_size();
        assert_eq!(0xFF, sz);

        // Set stuff.
        for i in 0..sz + 1 {
            mem.set(i, (i & 0xFF) as u8).unwrap()
        }

        mem.flush().unwrap();

        // Get stuff.
        for i in 0..sz + 1 {
            assert_eq!((i & 0xFF) as u8, mem.get(i).unwrap());
        }

        // Delete stuff.
        mem.delete(0, sz).unwrap();

        mem.flush().unwrap();

        // Get stuff.
        for i in 0..sz + 1 {
            assert_eq!(0, mem.get(i).unwrap());
        }
    }
}
