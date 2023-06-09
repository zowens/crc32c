//! Provides wrappers for Read and Write types which checksum the bytes being read/written.
use std::io::{Read, Write};

use crate::crc32c_append;

/// Reader wrapper which tracks the checksum of all bytes read.
pub struct Crc32cReader<R: Read> {
    checksum: u32,
    inner: R,
}

impl<R: Read> Crc32cReader<R> {
    /// Wrap an instance of a Reader.
    pub fn new(rdr: R) -> Self {
        Self {
            checksum: 0,
            inner: rdr,
        }
    }

    /// Unwrap the inner reader.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Get the checksum of all bytes read.
    pub fn crc32c(&self) -> u32 {
        self.checksum
    }
}

impl<R: Read> Read for Crc32cReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let out = self.inner.read(buf)?;
        self.checksum = crc32c_append(self.checksum, &buf[..out]);
        Ok(out)
    }
}

/// Writer wrapper which tracks the checksum of all bytes written.
pub struct Crc32Writer<W: Write> {
    checksum: u32,
    inner: W,
}

impl<W: Write> Crc32Writer<W> {
    /// Wrap an instance of a Writer.
    pub fn new(w: W) -> Self {
        Self {
            checksum: 0,
            inner: w,
        }
    }

    /// Unwrap the inner writer.
    pub fn into_inner(self) -> W {
        self.inner
    }

    /// Get the checksum of all bytes written.
    pub fn crc32c(&self) -> u32 {
        self.checksum
    }
}

impl<W: Write> Write for Crc32Writer<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let out = self.inner.write(buf)?;
        self.checksum = crc32c_append(self.checksum, &buf[..out]);
        Ok(out)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
