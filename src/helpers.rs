/////////////
// HELPERS //
/////////////

use interface::*;
use errors::MemError;

/// Copy all of the `src` memory block to `dst`.
/// Returns `Ok(writtenbytes)` to signify `writtenbytes` bytes have been copied,
/// or `Err(error)` if some error happened during copying.
pub fn copy<T: MemoryBlock>(src: T, dst: &mut T) -> Result<Addr, MemError> {
    let sz = src.get_size();
    if sz > dst.get_size() {
        panic!("dst memory block is bigger than src")
    }
    for i in 0..sz {
        let status = match src.get(i) {
            Ok(byte) => dst.set(i, byte),
            Err(e) => return Err(e),
        };
        try!(status);
    }
    Ok(sz)
}


/// Copy the contents of `src` at `from` to `to` to `dst`.
/// Returns `Ok(writtenbytes)` to signify `writtenbytes` bytes have been copied,
/// or `Err(error)` if some error happened during copying.
pub fn copy_at<T: MemoryBlock>(src: T,
                               dst: &mut T,
                               from: Addr,
                               to: Addr)
                               -> Result<Addr, MemError> {
    for i in from..to {
        let status = match src.get(i) {
            Ok(byte) => dst.set(i, byte),
            Err(e) => return Err(e),
        };
        try!(status);
    }
    Ok(to - from)
}
