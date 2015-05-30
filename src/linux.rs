use std::io::{Read, Write, Error, Result};
use std::os::unix::io::{RawFd, AsRawFd};
use std::mem;


// TODO: support offset and large file offsets
// #[cfg(not(target_arch = "x86"))]
fn sendfile_internal(out_fd: RawFd, in_fd: RawFd, count: usize) -> isize {
    unsafe {
        let ret: usize = syscall!(SENDFILE, out_fd, in_fd, 0, count);
        mem::transmute(ret)
    }
}


/*
#[cfg(target_arch = "x86")]
fn sendfile_internal(out_fd: RawFd, in_fd: RawFd, count: u64) -> usize {
    syscall!(SENDFILE64, out_fd, in_fd, 0, count)
}
*/


pub fn sendfile<R, W>(source: &R, sink: &W, count: usize) -> Result<usize>
        where R: Read + AsRawFd,
              W: Write + AsRawFd {
    let ret = sendfile_internal(
            sink.as_raw_fd(),
            source.as_raw_fd(),
            count
            );

    if ret < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret as usize)
    }
}


pub fn sendfile_all<R, W>(source: &R, sink: &W, chunk_size: usize) -> Result<usize>
        where R: Read + AsRawFd,
              W: Write + AsRawFd {
    let mut written = 0usize;

    loop {
        let ret = try!(sendfile(source, sink, chunk_size));

        if ret == 0usize {
            // we're done
            return Ok(written);
        } else {
            written += ret;
        }
    }
}
