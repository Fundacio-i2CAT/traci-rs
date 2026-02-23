// SPDX-License-Identifier: EPL-2.0
//! TCP socket layer — equivalent to `tcpip::Socket` in the C++ library.
//!
//! The TraCI wire format frames every message as:
//!
//! ```text
//! [ 4-byte big-endian total length (including these 4 bytes) ] [ payload bytes … ]
//! ```
//!
//! `send_exact` writes that frame.  `receive_exact` reads the 4-byte header first,
//! then blocks until all payload bytes have arrived.

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::error::TraciError;
use crate::storage::Storage;

/// Length of the message-length prefix in bytes (identical to `Socket::lengthLen` in C++).
const LENGTH_LEN: usize = 4;

/// A connected TraCI TCP socket.
pub struct TraciSocket {
    stream: TcpStream,
}

impl TraciSocket {
    /// Connect to a SUMO server at `host:port` and return the connected socket.
    ///
    /// Equivalent to `tcpip::Socket::connect()`.
    pub fn connect(host: &str, port: u16) -> Result<Self, TraciError> {
        let addr = format!("{host}:{port}");
        let stream = TcpStream::connect(&addr)
            .map_err(TraciError::Connection)?;

        // Disable Nagle's algorithm. Every TraCI call is a small independent
        // message; the caller blocks waiting for the reply, so batching
        // small writes only adds latency (~200 ms per round-trip without this).
        stream.set_nodelay(true)
            .map_err(TraciError::Connection)?;

        Ok(Self { stream })
    }

    /// Send the entire contents of `storage` as a length-framed TraCI message.
    ///
    /// The 4-byte big-endian `total_length` (header + payload) is prepended
    /// automatically, matching `tcpip::Socket::sendExact`.
    pub fn send_exact(&mut self, storage: &Storage) -> Result<(), TraciError> {
        let payload = storage.as_bytes();
        let total_len = (LENGTH_LEN + payload.len()) as u32;
        let header = total_len.to_be_bytes();
        self.stream.write_all(&header).map_err(TraciError::Connection)?;
        self.stream.write_all(payload).map_err(TraciError::Connection)?;
        Ok(())
    }

    /// Receive exactly one length-framed TraCI message.
    ///
    /// Reads the 4-byte length prefix, then blocks until all payload bytes are
    /// available.  Returns the payload wrapped in a [`Storage`] with the read
    /// cursor at position 0 — matching `tcpip::Socket::receiveExact`.
    pub fn receive_exact(&mut self) -> Result<Storage, TraciError> {
        // Read the 4-byte length header.
        let mut header = [0u8; LENGTH_LEN];
        self.stream.read_exact(&mut header).map_err(TraciError::Connection)?;
        let total_len = u32::from_be_bytes(header) as usize;
        if total_len < LENGTH_LEN {
            return Err(TraciError::Protocol(format!(
                "Received message length {total_len} is smaller than header size {LENGTH_LEN}"
            )));
        }
        let payload_len = total_len - LENGTH_LEN;
        let mut payload = vec![0u8; payload_len];
        self.stream.read_exact(&mut payload).map_err(TraciError::Connection)?;
        Ok(Storage::from_bytes(payload))
    }

    /// Close the underlying TCP connection.
    pub fn close(&mut self) -> Result<(), TraciError> {
        self.stream.shutdown(std::net::Shutdown::Both).map_err(TraciError::Connection)
    }
}
