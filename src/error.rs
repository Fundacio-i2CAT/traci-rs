// SPDX-License-Identifier: EPL-2.0
//! Error types for the `sumo-traci` crate.

use std::{fmt, io};

/// All errors that can be returned by the TraCI client.
#[derive(Debug)]
pub enum TraciError {
    /// A TCP/IP I/O error occurred (connect, send, receive, …).
    Connection(io::Error),

    /// The server responded with a protocol-level error
    /// (wrong command id, bad message length, unexpected type tag, …).
    Protocol(String),

    /// The SUMO server returned `RTYPE_ERR` for a command we sent.
    SimulationError(String),

    /// The SUMO server replied that the requested command is not implemented.
    NotImplemented(String),

    /// SUMO has reached the configured end time and closed the simulation.
    /// Returned by [`TraciClient::simulation_step`] when `CMD_CLOSE` is received.
    SimulationEnd,
}

impl fmt::Display for TraciError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraciError::Connection(e) => write!(f, "TraCI connection error: {e}"),
            TraciError::Protocol(msg) => write!(f, "TraCI protocol error: {msg}"),
            TraciError::SimulationError(msg) => write!(f, "TraCI simulation error: {msg}"),
            TraciError::NotImplemented(msg) => write!(f, "TraCI command not implemented: {msg}"),
            TraciError::SimulationEnd => write!(f, "SUMO simulation ended"),
        }
    }
}

impl std::error::Error for TraciError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TraciError::Connection(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for TraciError {
    fn from(e: io::Error) -> Self {
        TraciError::Connection(e)
    }
}
