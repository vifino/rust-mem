/////////////
// HELPERS //
/////////////

use interface::*;
use errors::MemError;

use std::cmp;

/// Copy all of the `src` memory block to `dst`.
/// Returns `Ok(writtenbytes)` to signify `writtenbytes` bytes have been copied,
/// or `Err(error)` if some error happened during copying.
pub fn copy<T: MemoryBlock>(src: &T, dst: &mut T) -> Result<Addr, MemError> {
    let src_sz = src.get_size();
    let dst_sz = dst.get_size();
    if src_sz > dst_sz {
        return Err(MemError::TooBig { given: src_sz, max: dst_sz});
    }
    for i in 0..src_sz {
        let status = match src.get(i) {
            Ok(byte) => dst.set(i, byte),
            Err(e) => return Err(e),
        };
        try!(status);
    }
    Ok(src_sz)
}


/// Copy the contents of `src` at `from` to `to` to `dst`, starting at `pos`.
/// Returns `Ok(writtenbytes)` to signify `writtenbytes` bytes have been copied,
/// or `Err(error)` if some error happened during copying.
pub fn copy_at<T: MemoryBlock>(src: &T, dst: &mut T, from: Addr, to: Addr, pos: Addr) -> Result<Addr, MemError> {
    let src_sz = src.get_size();
    let dst_sz = dst.get_size();

    let lowest = cmp::min(to, from);
    let numbytes = cmp::max(to, from) - lowest;

    // Handle invalid cases.
    if (lowest + numbytes) > src_sz {
        return Err(MemError::TooBig { given: (lowest + numbytes), max: src_sz});
    };
    if (pos + numbytes) > dst_sz {
        return Err(MemError::TooBig { given: (pos + numbytes), max: dst_sz});
    };

    // Actual copying
    for i in from..(to+1) {
        let status = match src.get(i) {
            Ok(byte) => {
                let dstpos = pos + (i - lowest);
                dst.set(dstpos, byte)
            },
            Err(e) => return Err(e),
        };
        try!(status);
    }
    Ok(numbytes)
}
