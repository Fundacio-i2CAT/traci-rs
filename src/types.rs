// SPDX-License-Identifier: EPL-2.0
//! Data structures and the [`TraciValue`] enum — translation of `libsumo/TraCIDefs.h`.
//!
//! In the C++ library subscription results are stored as
//! `map<int, shared_ptr<TraCIResult>>` using runtime polymorphism.  Here we use an
//! **enum** variant approach: every concrete result type is a variant of
//! [`TraciValue`].  This is zero-cost, exhaustively matchable, and idiomatic Rust.

use std::collections::HashMap;
use crate::constants::INVALID_DOUBLE_VALUE;

// ============================================================================
// TraciValue — the central enum replacing C++ shared_ptr<TraCIResult>
// ============================================================================

/// Every value that can be received from or sent to a SUMO TraCI server.
///
/// Variants map 1-to-1 to the TraCI `TYPE_*` / `POSITION_*` tags.  Where the
/// C++ library had specialised wrapper structs (e.g. `TraCILogicVectorWrapped`)
/// they are folded directly into the corresponding variant here.
#[derive(Debug, Clone, PartialEq)]
pub enum TraciValue {
    /// `TYPE_INTEGER` (0x09) — 32-bit signed integer.
    Int(i32),
    /// `TYPE_DOUBLE` (0x0b) — 64-bit IEEE 754 float.
    Double(f64),
    /// `TYPE_STRING` (0x0c).
    String(String),
    /// `TYPE_STRINGLIST` (0x0e).
    StringList(Vec<String>),
    /// `TYPE_DOUBLELIST` (0x10).
    DoubleList(Vec<f64>),
    /// `POSITION_2D` (0x01) — 2-D Cartesian position.
    Pos2D { x: f64, y: f64 },
    /// `POSITION_3D` (0x03) — 3-D Cartesian position.
    Pos3D { x: f64, y: f64, z: f64 },
    /// `TYPE_COLOR` (0x11) — RGBA colour.
    Color(TraciColor),
    /// `TYPE_POLYGON` (0x06) — list of 2-D positions.
    Polygon(Vec<TraciPosition>),
    /// A complete traffic-light program logic (`TL_COMPLETE_DEFINITION_RYG`).
    LogicList(Vec<TraciLogic>),
    /// A list of lane connections returned by `LANE_LINKS`.
    ConnectionList(Vec<Vec<TraciConnection>>),
    /// A `TraCIStage` structure (person stages, `FIND_ROUTE` result, …).
    Stage(TraciStage),
    /// Induction-loop per-vehicle data (`LAST_STEP_VEHICLE_DATA`).
    VehicleDataList(Vec<TraciVehicleData>),
    /// Upcoming traffic light data for a vehicle (`VAR_NEXT_TLS`).
    NextTLSList(Vec<TraciNextTLSData>),
    /// Best-lane information for a vehicle (`VAR_BEST_LANES`).
    BestLanesList(Vec<TraciBestLanesData>),
    /// A raw byte tag + bytes for forward-compatibility with unknown future types.
    Unknown { type_id: u8, raw: Vec<u8> },
}

// ============================================================================
// Subscription result type aliases
// ============================================================================

/// Per-object subscription results: `{ variable_id → value }`.
pub type TraciResults = HashMap<u8, TraciValue>;

/// All variable-subscription results: `{ object_id → TraciResults }`.
pub type SubscriptionResults = HashMap<String, TraciResults>;

/// All context-subscription results: `{ ego_object_id → SubscriptionResults }`.
pub type ContextSubscriptionResults = HashMap<String, SubscriptionResults>;

// ============================================================================
// Position
// ============================================================================

/// A 2-D or 3-D position. For 2-D positions `z` is [`INVALID_DOUBLE_VALUE`].
#[derive(Debug, Clone, PartialEq)]
pub struct TraciPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl TraciPosition {
    pub fn new_2d(x: f64, y: f64) -> Self {
        Self { x, y, z: INVALID_DOUBLE_VALUE }
    }

    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn is_3d(&self) -> bool {
        self.z != INVALID_DOUBLE_VALUE
    }
}

impl Default for TraciPosition {
    fn default() -> Self {
        Self { x: INVALID_DOUBLE_VALUE, y: INVALID_DOUBLE_VALUE, z: INVALID_DOUBLE_VALUE }
    }
}

// ============================================================================
// Road position
// ============================================================================

/// A position on the road network (edge + lane + offset along edge).
#[derive(Debug, Clone, PartialEq)]
pub struct TraciRoadPosition {
    pub edge_id: String,
    pub pos: f64,
    pub lane_index: i32,
}

// ============================================================================
// Colour
// ============================================================================

/// An RGBA colour (each channel 0–255).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraciColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl TraciColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for TraciColor {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

// ============================================================================
// Traffic light logic (phase programme)
// ============================================================================

/// One phase within a traffic light programme.
#[derive(Debug, Clone, PartialEq)]
pub struct TraciPhase {
    pub duration: f64,
    pub state: String,
    pub min_dur: f64,
    pub max_dur: f64,
    pub next: Vec<i32>,
    pub name: String,
}

impl Default for TraciPhase {
    fn default() -> Self {
        Self {
            duration: INVALID_DOUBLE_VALUE,
            state: String::new(),
            min_dur: INVALID_DOUBLE_VALUE,
            max_dur: INVALID_DOUBLE_VALUE,
            next: Vec::new(),
            name: String::new(),
        }
    }
}

/// A complete traffic-light programme.
#[derive(Debug, Clone, PartialEq)]
pub struct TraciLogic {
    pub program_id: String,
    /// Programme type (0 = static, 3 = actuated, …).
    pub type_: i32,
    pub current_phase_index: i32,
    pub phases: Vec<TraciPhase>,
    pub sub_parameter: HashMap<String, String>,
}

// ============================================================================
// Lane connections
// ============================================================================

/// One lane-to-lane connection (output of `LANE_LINKS`).
#[derive(Debug, Clone, PartialEq)]
pub struct TraciConnection {
    pub approached_lane: String,
    pub has_prio: bool,
    pub is_open: bool,
    pub has_foe: bool,
    pub approached_internal: String,
    pub state: String,
    pub direction: String,
    pub length: f64,
}

/// One lane-to-lane link (output of traffic-light controlled link queries).
#[derive(Debug, Clone, PartialEq)]
pub struct TraciLink {
    pub from_lane: String,
    pub via_lane: String,
    pub to_lane: String,
}

// ============================================================================
// Induction-loop vehicle data
// ============================================================================

/// Per-vehicle data from an induction-loop detector (`LAST_STEP_VEHICLE_DATA`).
#[derive(Debug, Clone, PartialEq)]
pub struct TraciVehicleData {
    pub id: String,
    pub length: f64,
    pub entry_time: f64,
    pub leave_time: f64,
    pub type_id: String,
}

// ============================================================================
// Upcoming traffic light data
// ============================================================================

/// One upcoming traffic light for a vehicle (`VAR_NEXT_TLS`).
#[derive(Debug, Clone, PartialEq)]
pub struct TraciNextTLSData {
    pub id: String,
    pub tl_index: i32,
    pub dist: f64,
    /// The current phase state character (e.g. `'r'`, `'g'`, `'y'`).
    pub state: char,
}

// ============================================================================
// Best-lanes data
// ============================================================================

/// Best-lane information for a vehicle (`VAR_BEST_LANES`).
#[derive(Debug, Clone, PartialEq)]
pub struct TraciBestLanesData {
    pub lane_id: String,
    pub length: f64,
    pub occupation: f64,
    pub best_lane_offset: i32,
    pub allows_continuation: bool,
    pub continuation_lanes: Vec<String>,
}

// ============================================================================
// Stage (person journey stages / route finding)
// ============================================================================

/// A person journey stage, or a found route (`VAR_STAGE`, `FIND_ROUTE`).
#[derive(Debug, Clone, PartialEq)]
pub struct TraciStage {
    pub type_: i32,
    pub v_type: String,
    pub line: String,
    pub dest_stop: String,
    pub edges: Vec<String>,
    pub travel_time: f64,
    pub cost: f64,
    pub length: f64,
    pub intended: String,
    pub depart: f64,
    pub depart_pos: f64,
    pub arrival_pos: f64,
    pub description: String,
}

impl Default for TraciStage {
    fn default() -> Self {
        Self {
            type_: crate::constants::INVALID_INT_VALUE,
            v_type: String::new(),
            line: String::new(),
            dest_stop: String::new(),
            edges: Vec::new(),
            travel_time: INVALID_DOUBLE_VALUE,
            cost: INVALID_DOUBLE_VALUE,
            length: INVALID_DOUBLE_VALUE,
            intended: String::new(),
            depart: INVALID_DOUBLE_VALUE,
            depart_pos: INVALID_DOUBLE_VALUE,
            arrival_pos: INVALID_DOUBLE_VALUE,
            description: String::new(),
        }
    }
}

// ============================================================================
// Next stop data
// ============================================================================

/// Detailed data for an upcoming or past vehicle stop.
#[derive(Debug, Clone, PartialEq)]
pub struct TraciNextStopData {
    pub lane: String,
    pub start_pos: f64,
    pub end_pos: f64,
    pub stopping_place_id: String,
    pub stop_flags: i32,
    pub duration: f64,
    pub until: f64,
    pub intended_arrival: f64,
    pub arrival: f64,
    pub depart: f64,
    pub split: String,
    pub join: String,
    pub act_type: String,
    pub trip_id: String,
    pub line: String,
    pub speed: f64,
}

// ============================================================================
// Taxi reservation
// ============================================================================

/// A taxi reservation as returned by `VAR_TAXI_RESERVATIONS`.
#[derive(Debug, Clone, PartialEq)]
pub struct TraciReservation {
    pub id: String,
    pub persons: Vec<String>,
    pub group: String,
    pub from_edge: String,
    pub to_edge: String,
    pub depart_pos: f64,
    pub arrival_pos: f64,
    pub depart: f64,
    pub reservation_time: f64,
    pub state: i32,
}

// ============================================================================
// Collision data
// ============================================================================

/// Data about a single collision event.
#[derive(Debug, Clone, PartialEq)]
pub struct TraciCollision {
    pub collider: String,
    pub victim: String,
    pub collider_type: String,
    pub victim_type: String,
    pub collider_speed: f64,
    pub victim_speed: f64,
    pub type_: String,
    pub lane: String,
    pub pos: f64,
}

// ============================================================================
// Signal constraint
// ============================================================================

/// A rail signal constraint.
#[derive(Debug, Clone, PartialEq)]
pub struct TraciSignalConstraint {
    pub signal_id: String,
    pub trip_id: String,
    pub foe_id: String,
    pub foe_signal: String,
    pub limit: i32,
    pub type_: i32,
    pub must_wait: bool,
    pub active: bool,
    pub param: HashMap<String, String>,
}

// ============================================================================
// SubscribedKinematics — populated by VehicleScope::subscribe_kinematics
// ============================================================================

/// Kinematic state for a vehicle, populated by `VehicleScope::subscribe_kinematics`.
///
/// All fields are updated automatically on every `simulation_step()` call once
/// a subscription has been set up.
#[derive(Debug, Clone, PartialEq)]
pub struct SubscribedKinematics {
    /// 2-D Cartesian position in the SUMO network coordinate system (metres).
    pub position: TraciPosition,
    /// Longitudinal speed (m/s).
    pub speed: f64,
    /// Longitudinal acceleration (m/s²). Positive = accelerating, negative = braking.
    pub acceleration: f64,
    /// Heading angle (degrees, 0 = North, clockwise).
    pub angle: f64,
}
