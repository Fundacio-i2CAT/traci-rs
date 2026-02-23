// SPDX-License-Identifier: EPL-2.0
//! # traci-rs
//!
//! A pure-Rust client library for the [SUMO](https://sumo.dlr.de) TraCI
//! (Traffic Control Interface) protocol.
//!
//! ## Quick start
//!
//! ```no_run
//! use traci_rs::TraciClient;
//!
//! fn main() -> Result<(), traci_rs::TraciError> {
//!     let mut client = TraciClient::connect("127.0.0.1", 8813)?;
//!     client.set_order(1)?;
//!
//!     // Advance the simulation one step
//!     client.simulation_step(0.0)?;
//!
//!     // Query all vehicles
//!     let ids = client.vehicle.get_id_list(&mut client)?;
//!     for id in &ids {
//!         let pos = client.vehicle.get_position(&mut client, id)?;
//!         println!("{}: ({}, {})", id, pos.x, pos.y);
//!     }
//!
//!     client.close()?;
//!     Ok(())
//! }
//! ```

#[macro_use]
pub(crate) mod scopes;

pub mod constants;
pub mod error;
pub mod storage;
pub mod socket;
pub mod types;
pub mod client;

pub use client::TraciClient;
pub use error::TraciError;
pub use types::*;

// Re-export scope types so users can name them in generic helpers.
pub use scopes::vehicle::VehicleScope;
pub use scopes::person::PersonScope;
pub use scopes::simulation::SimulationScope;
pub use scopes::traffic_light::TrafficLightScope;
pub use scopes::vehicle_type::VehicleTypeScope;
pub use scopes::edge::EdgeScope;
pub use scopes::lane::LaneScope;
pub use scopes::junction::JunctionScope;
pub use scopes::route::RouteScope;
