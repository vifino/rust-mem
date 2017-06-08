/////////////
// HELPERS //
/////////////

use interface::*;
use errors::*;

use std::cmp;

/// Copy all of the `src` memory block to `dst`.
/// Returns `Ok(writtenbytes)` to signify `writtenbytes` bytes have been copied,
/// or `Err(error)` if some error happened during copying.
pub fn copy<T: MemoryBlock>(src: &T, dst: &mut T) -> Result<Addr, Error> {
    let src_sz = src.get_size();
    let dst_sz = dst.get_size();
    if src_sz > dst_sz {
        bail!(ErrorKind::TooBig(src_sz, dst_sz));
    }
    for i in 0..src_sz {
        let status = match src.get(i) {
            Ok(byte) => dst.set(i, byte),
            Err(e) => Err(e),
        };
        status.chain_err(|| "in copy helper")?;
    }
    Ok(src_sz)
}


/// Copy the contents of `src` at `from` to `to` to `dst`, starting at `pos`.
/// Returns `Ok(writtenbytes)` to signify `writtenbytes` bytes have been copied,
/// or `Err(error)` if some error happened during copying.
pub fn copy_at<T: MemoryBlock>(src: &T, dst: &mut T, from: Addr, to: Addr, pos: Addr) -> Result<Addr, Error> {
    let src_sz = src.get_size();
    let dst_sz = dst.get_size();

    let lowest = cmp::min(to, from);
    let numbytes = cmp::max(to, from) - lowest;

    // Handle invalid cases.
    if (lowest + numbytes) > src_sz {
        bail!(ErrorKind::TooBig(lowest + numbytes, src_sz));
    };
    if (pos + numbytes) > dst_sz {
        bail!(ErrorKind::TooBig(pos + numbytes, dst_sz));
    };

    // Actual copying
    for i in from..(to+1) {
        let status = match src.get(i) {
            Ok(byte) => {
                let dstpos = pos + (i - lowest);
                dst.set(dstpos, byte)
            },
            Err(e) => Err(e),
        };
        status.chain_err(|| "in copy_at helper")?;
    }
    Ok(numbytes)
}
