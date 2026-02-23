// SPDX-License-Identifier: EPL-2.0
//! Binary serialisation buffer — equivalent to `tcpip::Storage` in the C++ library.
//!
//! All multi-byte integers and floating-point values are encoded in **big-endian**
//! byte order, matching the SUMO TraCI wire format.

use crate::error::TraciError;

/// A byte buffer that supports sequential writing and reading of TraCI wire-format
/// primitive types.
///
/// The buffer internally holds a `Vec<u8>` and keeps a separate read cursor so that
/// write and read operations can be freely interleaved (just like the C++ `Storage`).
#[derive(Debug, Default)]
pub struct Storage {
    buf: Vec<u8>,
    pos: usize,
}

impl Storage {
    // -----------------------------------------------------------------------
    // Construction
    // -----------------------------------------------------------------------

    /// Create an empty storage buffer.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a storage buffer pre-loaded with the given bytes.
    /// The read cursor starts at position 0.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { buf: data, pos: 0 }
    }

    // -----------------------------------------------------------------------
    // Buffer state
    // -----------------------------------------------------------------------

    /// Number of bytes currently stored.
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// `true` when the buffer contains no bytes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Current read-cursor position.
    #[inline]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// `true` when the read cursor has not reached the end of the buffer.
    #[inline]
    pub fn valid_pos(&self) -> bool {
        self.pos < self.buf.len()
    }

    /// Return the raw byte slice (entire buffer, regardless of cursor).
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    /// Clear all content and reset the read cursor.
    #[inline]
    pub fn reset(&mut self) {
        self.buf.clear();
        self.pos = 0;
    }

    /// Reset the read cursor to position 0 without discarding written data.
    #[inline]
    pub fn reset_pos(&mut self) {
        self.pos = 0;
    }

    /// Append all bytes from `other` to this buffer.
    pub fn append_storage(&mut self, other: &Storage) {
        self.buf.extend_from_slice(&other.buf);
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn check_read(&self, n: usize) -> Result<(), TraciError> {
        if self.pos + n > self.buf.len() {
            Err(TraciError::Protocol(format!(
                "Storage: attempt to read {} bytes at position {} but buffer length is {}",
                n,
                self.pos,
                self.buf.len()
            )))
        } else {
            Ok(())
        }
    }

    #[inline]
    fn read_raw_bytes<const N: usize>(&mut self) -> Result<[u8; N], TraciError> {
        self.check_read(N)?;
        let mut arr = [0u8; N];
        arr.copy_from_slice(&self.buf[self.pos..self.pos + N]);
        self.pos += N;
        Ok(arr)
    }

    // -----------------------------------------------------------------------
    // Unsigned byte  (u8)
    // -----------------------------------------------------------------------

    /// Read one unsigned byte (0–255).
    pub fn read_u8(&mut self) -> Result<u8, TraciError> {
        self.check_read(1)?;
        let v = self.buf[self.pos];
        self.pos += 1;
        Ok(v)
    }

    /// Write one unsigned byte.
    ///
    /// # Panics
    /// Panics in debug builds if `value > 255` (which cannot happen for a `u8`).
    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        self.buf.push(value);
    }

    // -----------------------------------------------------------------------
    // Signed byte  (i8 wrapped as i32 for API compatibility)
    // -----------------------------------------------------------------------

    /// Read one signed byte (−128..127), returned as `i32`.
    pub fn read_byte(&mut self) -> Result<i32, TraciError> {
        let raw = self.read_u8()? as i32;
        Ok(if raw < 128 { raw } else { raw - 256 })
    }

    /// Write one signed byte value in −128..127.
    pub fn write_byte(&mut self, value: i32) -> Result<(), TraciError> {
        if !(-128..=127).contains(&value) {
            return Err(TraciError::Protocol(format!(
                "Storage::write_byte: value {value} out of range [-128, 127]"
            )));
        }
        self.write_u8(((value + 256) % 256) as u8);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Short  (i16)
    // -----------------------------------------------------------------------

    /// Read a big-endian signed 16-bit integer.
    pub fn read_i16(&mut self) -> Result<i16, TraciError> {
        Ok(i16::from_be_bytes(self.read_raw_bytes::<2>()?))
    }

    /// Write a big-endian signed 16-bit integer.
    pub fn write_i16(&mut self, value: i16) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    // -----------------------------------------------------------------------
    // Integer  (i32)
    // -----------------------------------------------------------------------

    /// Read a big-endian signed 32-bit integer.
    pub fn read_i32(&mut self) -> Result<i32, TraciError> {
        Ok(i32::from_be_bytes(self.read_raw_bytes::<4>()?))
    }

    /// Write a big-endian signed 32-bit integer.
    pub fn write_i32(&mut self, value: i32) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    // -----------------------------------------------------------------------
    // Float  (f32)
    // -----------------------------------------------------------------------

    /// Read a big-endian 32-bit float.
    pub fn read_f32(&mut self) -> Result<f32, TraciError> {
        Ok(f32::from_be_bytes(self.read_raw_bytes::<4>()?))
    }

    /// Write a big-endian 32-bit float.
    pub fn write_f32(&mut self, value: f32) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    // -----------------------------------------------------------------------
    // Double  (f64)
    // -----------------------------------------------------------------------

    /// Read a big-endian 64-bit double.
    pub fn read_f64(&mut self) -> Result<f64, TraciError> {
        Ok(f64::from_be_bytes(self.read_raw_bytes::<8>()?))
    }

    /// Write a big-endian 64-bit double.
    pub fn write_f64(&mut self, value: f64) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    // -----------------------------------------------------------------------
    // String  (i32 length prefix + UTF-8 bytes)
    // -----------------------------------------------------------------------

    /// Read a length-prefixed string (4-byte big-endian length, then UTF-8 bytes).
    pub fn read_string(&mut self) -> Result<String, TraciError> {
        let len = self.read_i32()? as usize;
        self.check_read(len)?;
        let bytes = self.buf[self.pos..self.pos + len].to_vec();
        self.pos += len;
        String::from_utf8(bytes).map_err(|e| TraciError::Protocol(format!("Invalid UTF-8 in string: {e}")))
    }

    /// Write a length-prefixed string.
    pub fn write_string(&mut self, s: &str) {
        self.write_i32(s.len() as i32);
        self.buf.extend_from_slice(s.as_bytes());
    }

    // -----------------------------------------------------------------------
    // String list  (i32 count + n strings)
    // -----------------------------------------------------------------------

    /// Read a list of length-prefixed strings.
    pub fn read_string_list(&mut self) -> Result<Vec<String>, TraciError> {
        let count = self.read_i32()?;
        let mut v = Vec::with_capacity(count as usize);
        for _ in 0..count {
            v.push(self.read_string()?);
        }
        Ok(v)
    }

    /// Write a list of length-prefixed strings.
    pub fn write_string_list(&mut self, list: &[String]) {
        self.write_i32(list.len() as i32);
        for s in list {
            self.write_string(s);
        }
    }

    // -----------------------------------------------------------------------
    // Double list  (i32 count + n f64s)
    // -----------------------------------------------------------------------

    /// Read a list of big-endian f64 values.
    pub fn read_f64_list(&mut self) -> Result<Vec<f64>, TraciError> {
        let count = self.read_i32()?;
        let mut v = Vec::with_capacity(count as usize);
        for _ in 0..count {
            v.push(self.read_f64()?);
        }
        Ok(v)
    }

    /// Write a list of big-endian f64 values.
    pub fn write_f64_list(&mut self, list: &[f64]) {
        self.write_i32(list.len() as i32);
        for &d in list {
            self.write_f64(d);
        }
    }

    // -----------------------------------------------------------------------
    // Raw packet append
    // -----------------------------------------------------------------------

    /// Append raw bytes directly (used when forwarding storage sub-payloads).
    pub fn write_packet(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }
}

// ---------------------------------------------------------------------------
// Convenience: hex dump (mirrors C++ hexDump for debugging)
// ---------------------------------------------------------------------------

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.buf {
            write!(f, "{b:02X} ")?;
        }
        Ok(())
    }
}
