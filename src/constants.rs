// SPDX-License-Identifier: EPL-2.0
//! TraCI protocol constants — direct translation of `libsumo/TraCIConstants.h`.
//!
//! Every `constexpr int` in the C++ header becomes a `pub const u8` (for
//! single-byte command/variable identifiers) or `pub const i32` (for signed
//! flags and special values).  The naming convention keeps the original
//! SCREAMING_SNAKE_CASE identifiers unchanged so that porting code from C++ to
//! Rust requires only mechanical renaming.

#![allow(non_upper_case_globals, dead_code)]

// ============================================================================
// VERSION
// ============================================================================
pub const TRACI_VERSION: i32 = 22;

// ============================================================================
// COMMANDS
// ============================================================================
pub const CMD_GETVERSION: u8              = 0x00;
pub const CMD_LOAD: u8                    = 0x01;
pub const CMD_EXECUTEMOVE: u8             = 0x7d;
pub const CMD_SIMSTEP: u8                 = 0x02;
pub const CMD_SETORDER: u8                = 0x03;
pub const CMD_STOP: u8                    = 0x12;
pub const CMD_REROUTE_TO_PARKING: u8      = 0xc2;
pub const CMD_RESUME: u8                  = 0x19;
pub const CMD_CHANGELANE: u8              = 0x13;
pub const CMD_SLOWDOWN: u8                = 0x14;
pub const CMD_CHANGESUBLANE: u8           = 0x15;
pub const CMD_OPENGAP: u8                 = 0x16;
pub const CMD_REPLACE_STOP: u8            = 0x17;
pub const CMD_INSERT_STOP: u8             = 0x18;
pub const VAR_TAXI_FLEET: u8              = 0x20;
pub const CMD_TAXI_DISPATCH: u8           = 0x21;
pub const CMD_CHANGETARGET: u8            = 0x31;
pub const CMD_CLOSE: u8                   = 0x7f;
pub const CMD_ADD_SUBSCRIPTION_FILTER: u8 = 0x7e;

// ============================================================================
// INDUCTION LOOP (E1)
// ============================================================================
pub const CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT: u8     = 0x80;
pub const RESPONSE_SUBSCRIBE_INDUCTIONLOOP_CONTEXT: u8 = 0x90;
pub const CMD_GET_INDUCTIONLOOP_VARIABLE: u8           = 0xa0;
pub const RESPONSE_GET_INDUCTIONLOOP_VARIABLE: u8      = 0xb0;
pub const CMD_SET_INDUCTIONLOOP_VARIABLE: u8           = 0xc0;
pub const CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE: u8     = 0xd0;
pub const RESPONSE_SUBSCRIBE_INDUCTIONLOOP_VARIABLE: u8 = 0xe0;

// ============================================================================
// MULTI-ENTRY/EXIT (E3)
// ============================================================================
pub const CMD_SUBSCRIBE_MULTIENTRYEXIT_CONTEXT: u8     = 0x81;
pub const RESPONSE_SUBSCRIBE_MULTIENTRYEXIT_CONTEXT: u8 = 0x91;
pub const CMD_GET_MULTIENTRYEXIT_VARIABLE: u8           = 0xa1;
pub const RESPONSE_GET_MULTIENTRYEXIT_VARIABLE: u8      = 0xb1;
pub const CMD_SET_MULTIENTRYEXIT_VARIABLE: u8           = 0xc1;
pub const CMD_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE: u8     = 0xd1;
pub const RESPONSE_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE: u8 = 0xe1;

// ============================================================================
// TRAFFIC LIGHTS
// ============================================================================
pub const CMD_SUBSCRIBE_TL_CONTEXT: u8     = 0x82;
pub const RESPONSE_SUBSCRIBE_TL_CONTEXT: u8 = 0x92;
pub const CMD_GET_TL_VARIABLE: u8           = 0xa2;
pub const RESPONSE_GET_TL_VARIABLE: u8      = 0xb2;
pub const CMD_SET_TL_VARIABLE: u8           = 0xc2;
pub const CMD_SUBSCRIBE_TL_VARIABLE: u8     = 0xd2;
pub const RESPONSE_SUBSCRIBE_TL_VARIABLE: u8 = 0xe2;

// ============================================================================
// LANE
// ============================================================================
pub const CMD_SUBSCRIBE_LANE_CONTEXT: u8     = 0x83;
pub const RESPONSE_SUBSCRIBE_LANE_CONTEXT: u8 = 0x93;
pub const CMD_GET_LANE_VARIABLE: u8           = 0xa3;
pub const RESPONSE_GET_LANE_VARIABLE: u8      = 0xb3;
pub const CMD_SET_LANE_VARIABLE: u8           = 0xc3;
pub const CMD_SUBSCRIBE_LANE_VARIABLE: u8     = 0xd3;
pub const RESPONSE_SUBSCRIBE_LANE_VARIABLE: u8 = 0xe3;

// ============================================================================
// VEHICLE
// ============================================================================
pub const CMD_SUBSCRIBE_VEHICLE_CONTEXT: u8     = 0x84;
pub const RESPONSE_SUBSCRIBE_VEHICLE_CONTEXT: u8 = 0x94;
pub const CMD_GET_VEHICLE_VARIABLE: u8           = 0xa4;
pub const RESPONSE_GET_VEHICLE_VARIABLE: u8      = 0xb4;
pub const CMD_SET_VEHICLE_VARIABLE: u8           = 0xc4;
pub const CMD_SUBSCRIBE_VEHICLE_VARIABLE: u8     = 0xd4;
pub const RESPONSE_SUBSCRIBE_VEHICLE_VARIABLE: u8 = 0xe4;

// ============================================================================
// VEHICLE TYPE
// ============================================================================
pub const CMD_SUBSCRIBE_VEHICLETYPE_CONTEXT: u8     = 0x85;
pub const RESPONSE_SUBSCRIBE_VEHICLETYPE_CONTEXT: u8 = 0x95;
pub const CMD_GET_VEHICLETYPE_VARIABLE: u8           = 0xa5;
pub const RESPONSE_GET_VEHICLETYPE_VARIABLE: u8      = 0xb5;
pub const CMD_SET_VEHICLETYPE_VARIABLE: u8           = 0xc5;
pub const CMD_SUBSCRIBE_VEHICLETYPE_VARIABLE: u8     = 0xd5;
pub const RESPONSE_SUBSCRIBE_VEHICLETYPE_VARIABLE: u8 = 0xe5;

// ============================================================================
// ROUTE
// ============================================================================
pub const CMD_SUBSCRIBE_ROUTE_CONTEXT: u8     = 0x86;
pub const RESPONSE_SUBSCRIBE_ROUTE_CONTEXT: u8 = 0x96;
pub const CMD_GET_ROUTE_VARIABLE: u8           = 0xa6;
pub const RESPONSE_GET_ROUTE_VARIABLE: u8      = 0xb6;
pub const CMD_SET_ROUTE_VARIABLE: u8           = 0xc6;
pub const CMD_SUBSCRIBE_ROUTE_VARIABLE: u8     = 0xd6;
pub const RESPONSE_SUBSCRIBE_ROUTE_VARIABLE: u8 = 0xe6;

// ============================================================================
// POI
// ============================================================================
pub const CMD_SUBSCRIBE_POI_CONTEXT: u8     = 0x87;
pub const RESPONSE_SUBSCRIBE_POI_CONTEXT: u8 = 0x97;
pub const CMD_GET_POI_VARIABLE: u8           = 0xa7;
pub const RESPONSE_GET_POI_VARIABLE: u8      = 0xb7;
pub const CMD_SET_POI_VARIABLE: u8           = 0xc7;
pub const CMD_SUBSCRIBE_POI_VARIABLE: u8     = 0xd7;
pub const RESPONSE_SUBSCRIBE_POI_VARIABLE: u8 = 0xe7;

// ============================================================================
// POLYGON
// ============================================================================
pub const CMD_SUBSCRIBE_POLYGON_CONTEXT: u8     = 0x88;
pub const RESPONSE_SUBSCRIBE_POLYGON_CONTEXT: u8 = 0x98;
pub const CMD_GET_POLYGON_VARIABLE: u8           = 0xa8;
pub const RESPONSE_GET_POLYGON_VARIABLE: u8      = 0xb8;
pub const CMD_SET_POLYGON_VARIABLE: u8           = 0xc8;
pub const CMD_SUBSCRIBE_POLYGON_VARIABLE: u8     = 0xd8;
pub const RESPONSE_SUBSCRIBE_POLYGON_VARIABLE: u8 = 0xe8;

// ============================================================================
// JUNCTION
// ============================================================================
pub const CMD_SUBSCRIBE_JUNCTION_CONTEXT: u8     = 0x89;
pub const RESPONSE_SUBSCRIBE_JUNCTION_CONTEXT: u8 = 0x99;
pub const CMD_GET_JUNCTION_VARIABLE: u8           = 0xa9;
pub const RESPONSE_GET_JUNCTION_VARIABLE: u8      = 0xb9;
pub const CMD_SET_JUNCTION_VARIABLE: u8           = 0xc9;
pub const CMD_SUBSCRIBE_JUNCTION_VARIABLE: u8     = 0xd9;
pub const RESPONSE_SUBSCRIBE_JUNCTION_VARIABLE: u8 = 0xe9;

// ============================================================================
// EDGE
// ============================================================================
pub const CMD_SUBSCRIBE_EDGE_CONTEXT: u8     = 0x8a;
pub const RESPONSE_SUBSCRIBE_EDGE_CONTEXT: u8 = 0x9a;
pub const CMD_GET_EDGE_VARIABLE: u8           = 0xaa;
pub const RESPONSE_GET_EDGE_VARIABLE: u8      = 0xba;
pub const CMD_SET_EDGE_VARIABLE: u8           = 0xca;
pub const CMD_SUBSCRIBE_EDGE_VARIABLE: u8     = 0xda;
pub const RESPONSE_SUBSCRIBE_EDGE_VARIABLE: u8 = 0xea;

// ============================================================================
// SIMULATION
// ============================================================================
pub const CMD_SUBSCRIBE_SIM_CONTEXT: u8     = 0x8b;
pub const RESPONSE_SUBSCRIBE_SIM_CONTEXT: u8 = 0x9b;
pub const CMD_GET_SIM_VARIABLE: u8           = 0xab;
pub const RESPONSE_GET_SIM_VARIABLE: u8      = 0xbb;
pub const CMD_SET_SIM_VARIABLE: u8           = 0xcb;
pub const CMD_SUBSCRIBE_SIM_VARIABLE: u8     = 0xdb;
pub const RESPONSE_SUBSCRIBE_SIM_VARIABLE: u8 = 0xeb;

// ============================================================================
// GUI
// ============================================================================
pub const CMD_SUBSCRIBE_GUI_CONTEXT: u8     = 0x8c;
pub const RESPONSE_SUBSCRIBE_GUI_CONTEXT: u8 = 0x9c;
pub const CMD_GET_GUI_VARIABLE: u8           = 0xac;
pub const RESPONSE_GET_GUI_VARIABLE: u8      = 0xbc;
pub const CMD_SET_GUI_VARIABLE: u8           = 0xcc;
pub const CMD_SUBSCRIBE_GUI_VARIABLE: u8     = 0xdc;
pub const RESPONSE_SUBSCRIBE_GUI_VARIABLE: u8 = 0xec;

// ============================================================================
// LANE AREA (E2)
// ============================================================================
pub const CMD_SUBSCRIBE_LANEAREA_CONTEXT: u8     = 0x8d;
pub const RESPONSE_SUBSCRIBE_LANEAREA_CONTEXT: u8 = 0x9d;
pub const CMD_GET_LANEAREA_VARIABLE: u8           = 0xad;
pub const RESPONSE_GET_LANEAREA_VARIABLE: u8      = 0xbd;
pub const CMD_SET_LANEAREA_VARIABLE: u8           = 0xcd;
pub const CMD_SUBSCRIBE_LANEAREA_VARIABLE: u8     = 0xdd;
pub const RESPONSE_SUBSCRIBE_LANEAREA_VARIABLE: u8 = 0xed;

// ============================================================================
// PERSON
// ============================================================================
pub const CMD_SUBSCRIBE_PERSON_CONTEXT: u8     = 0x8e;
pub const RESPONSE_SUBSCRIBE_PERSON_CONTEXT: u8 = 0x9e;
pub const CMD_GET_PERSON_VARIABLE: u8           = 0xae;
pub const RESPONSE_GET_PERSON_VARIABLE: u8      = 0xbe;
pub const CMD_SET_PERSON_VARIABLE: u8           = 0xce;
pub const CMD_SUBSCRIBE_PERSON_VARIABLE: u8     = 0xde;
pub const RESPONSE_SUBSCRIBE_PERSON_VARIABLE: u8 = 0xee;

// ============================================================================
// REROUTER
// ============================================================================
pub const CMD_SUBSCRIBE_REROUTER_CONTEXT: u8     = 0x08;
pub const RESPONSE_SUBSCRIBE_REROUTER_CONTEXT: u8 = 0x18;
pub const CMD_GET_REROUTER_VARIABLE: u8           = 0x28;
pub const RESPONSE_GET_REROUTER_VARIABLE: u8      = 0x38;
pub const CMD_SET_REROUTER_VARIABLE: u8           = 0x48;
pub const CMD_SUBSCRIBE_REROUTER_VARIABLE: u8     = 0x58;
pub const RESPONSE_SUBSCRIBE_REROUTER_VARIABLE: u8 = 0x68;

// ============================================================================
// ROUTE PROBE
// ============================================================================
pub const CMD_SUBSCRIBE_ROUTEPROBE_CONTEXT: u8     = 0x06;
pub const RESPONSE_SUBSCRIBE_ROUTEPROBE_CONTEXT: u8 = 0x16;
pub const CMD_GET_ROUTEPROBE_VARIABLE: u8           = 0x26;
pub const RESPONSE_GET_ROUTEPROBE_VARIABLE: u8      = 0x36;
pub const CMD_SET_ROUTEPROBE_VARIABLE: u8           = 0x46;
pub const CMD_SUBSCRIBE_ROUTEPROBE_VARIABLE: u8     = 0x56;
pub const RESPONSE_SUBSCRIBE_ROUTEPROBE_VARIABLE: u8 = 0x66;

// ============================================================================
// POSITION REPRESENTATIONS
// ============================================================================
pub const POSITION_LON_LAT: u8     = 0x00;
pub const POSITION_2D: u8          = 0x01;
pub const POSITION_LON_LAT_ALT: u8 = 0x02;
pub const POSITION_3D: u8          = 0x03;
pub const POSITION_ROADMAP: u8     = 0x04;

// ============================================================================
// DATA TYPES
// ============================================================================
pub const TYPE_POLYGON: u8    = 0x06;
pub const TYPE_UBYTE: u8      = 0x07;
pub const TYPE_BYTE: u8       = 0x08;
pub const TYPE_INTEGER: u8    = 0x09;
pub const TYPE_DOUBLE: u8     = 0x0b;
pub const TYPE_STRING: u8     = 0x0c;
pub const TYPE_STRINGLIST: u8 = 0x0e;
pub const TYPE_COMPOUND: u8   = 0x0f;
pub const TYPE_DOUBLELIST: u8 = 0x10;
pub const TYPE_COLOR: u8      = 0x11;

// ============================================================================
// RESULT TYPES
// ============================================================================
pub const RTYPE_OK: u8             = 0x00;
pub const RTYPE_NOTIMPLEMENTED: u8 = 0x01;
pub const RTYPE_ERR: u8            = 0xff;

// ============================================================================
// SPECIAL / INVALID VALUES
// ============================================================================
pub const INVALID_DOUBLE_VALUE: f64 = -1_073_741_824.0_f64;
pub const INVALID_INT_VALUE: i32    = -1_073_741_824_i32;
pub const MAX_ORDER: i32            = 1_073_741_824_i32;
pub const DEFAULT_NUM_RETRIES: i32  = 60;

// ============================================================================
// DISTANCE REQUESTS
// ============================================================================
pub const REQUEST_AIRDIST: u8    = 0x00;
pub const REQUEST_DRIVINGDIST: u8 = 0x01;

// ============================================================================
// VEHICLE REMOVAL REASONS
// ============================================================================
pub const REMOVE_TELEPORT: u8          = 0x00;
pub const REMOVE_PARKING: u8           = 0x01;
pub const REMOVE_ARRIVED: u8           = 0x02;
pub const REMOVE_VAPORIZED: u8         = 0x03;
pub const REMOVE_TELEPORT_ARRIVED: u8  = 0x04;

// ============================================================================
// VEHICLE MOVE REASONS
// ============================================================================
pub const MOVE_AUTOMATIC: u8 = 0x00;
pub const MOVE_TELEPORT: u8  = 0x01;
pub const MOVE_NORMAL: u8    = 0x02;

// ============================================================================
// PERSON / CONTAINER STAGES
// ============================================================================
pub const STAGE_WAITING_FOR_DEPART: i32 = 0x00;
pub const STAGE_WAITING: i32            = 0x01;
pub const STAGE_WALKING: i32            = 0x02;
pub const STAGE_DRIVING: i32            = 0x03;
pub const STAGE_ACCESS: i32             = 0x04;
pub const STAGE_TRIP: i32               = 0x05;
pub const STAGE_TRANSHIP: i32           = 0x06;

// ============================================================================
// STOP FLAGS
// ============================================================================
pub const STOP_DEFAULT: u8              = 0x00;
pub const STOP_PARKING: u8              = 0x01;
pub const STOP_TRIGGERED: u8            = 0x02;
pub const STOP_CONTAINER_TRIGGERED: u8  = 0x04;
pub const STOP_BUS_STOP: u8             = 0x08;
pub const STOP_CONTAINER_STOP: u8       = 0x10;
pub const STOP_CHARGING_STATION: u8     = 0x20;
pub const STOP_PARKING_AREA: u8         = 0x40;
pub const STOP_OVERHEAD_WIRE: u8        = 0x80;

// ============================================================================
// DEPARTURE FLAGS
// ============================================================================
pub const DEPARTFLAG_TRIGGERED: i32           = -0x01;
pub const DEPARTFLAG_CONTAINER_TRIGGERED: i32 = -0x02;
pub const DEPARTFLAG_NOW: i32                 = -0x03;
pub const DEPARTFLAG_SPLIT: i32               = -0x04;
pub const DEPARTFLAG_BEGIN: i32               = -0x05;
pub const DEPARTFLAG_SPEED_RANDOM: i32        = -0x02;
pub const DEPARTFLAG_SPEED_MAX: i32           = -0x03;
pub const DEPARTFLAG_LANE_RANDOM: i32         = -0x02;
pub const DEPARTFLAG_LANE_FREE: i32           = -0x03;
pub const DEPARTFLAG_LANE_ALLOWED_FREE: i32   = -0x04;
pub const DEPARTFLAG_LANE_BEST_FREE: i32      = -0x05;
pub const DEPARTFLAG_LANE_FIRST_ALLOWED: i32  = -0x06;
pub const DEPARTFLAG_POS_RANDOM: i32          = -0x02;
pub const DEPARTFLAG_POS_FREE: i32            = -0x03;
pub const DEPARTFLAG_POS_BASE: i32            = -0x04;
pub const DEPARTFLAG_POS_LAST: i32            = -0x05;
pub const DEPARTFLAG_POS_RANDOM_FREE: i32     = -0x06;
pub const ARRIVALFLAG_LANE_CURRENT: i32       = -0x02;
pub const ARRIVALFLAG_SPEED_CURRENT: i32      = -0x02;
pub const ARRIVALFLAG_POS_RANDOM: i32         = -0x02;
pub const ARRIVALFLAG_POS_MAX: i32            = -0x03;

// ============================================================================
// ROUTING MODES
// ============================================================================
pub const ROUTING_MODE_DEFAULT: i32                       = 0x00;
pub const ROUTING_MODE_AGGREGATED: i32                    = 0x01;
pub const ROUTING_MODE_EFFORT: i32                        = 0x02;
pub const ROUTING_MODE_COMBINED: i32                      = 0x03;
pub const ROUTING_MODE_AGGREGATED_CUSTOM: i32             = 0x04;
pub const ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS: i32  = 0x08;

// ============================================================================
// TRAFFIC LIGHT TYPES
// ============================================================================
pub const TRAFFICLIGHT_TYPE_STATIC: i32      = 0x00;
pub const TRAFFICLIGHT_TYPE_ACTUATED: i32    = 0x03;
pub const TRAFFICLIGHT_TYPE_NEMA: i32        = 0x04;
pub const TRAFFICLIGHT_TYPE_DELAYBASED: i32  = 0x05;

// ============================================================================
// LANE CHANGE DIRECTIONS
// ============================================================================
pub const LANECHANGE_LEFT: i32  =  0x01;
pub const LANECHANGE_RIGHT: i32 = -0x01;

// ============================================================================
// SUBSCRIPTION FILTER TYPES
// ============================================================================
pub const FILTER_TYPE_NONE: u8             = 0x00;
pub const FILTER_TYPE_LANES: u8            = 0x01;
pub const FILTER_TYPE_NOOPPOSITE: u8       = 0x02;
pub const FILTER_TYPE_DOWNSTREAM_DIST: u8  = 0x03;
pub const FILTER_TYPE_UPSTREAM_DIST: u8    = 0x04;
pub const FILTER_TYPE_LEAD_FOLLOW: u8      = 0x05;
pub const FILTER_TYPE_TURN: u8             = 0x07;
pub const FILTER_TYPE_VCLASS: u8           = 0x08;
pub const FILTER_TYPE_VTYPE: u8            = 0x09;
pub const FILTER_TYPE_FIELD_OF_VISION: u8  = 0x0a;
pub const FILTER_TYPE_LATERAL_DIST: u8     = 0x0b;

// ============================================================================
// VARIABLE IDENTIFIERS (CMD_GET_*/CMD_SET_*)
// ============================================================================
pub const TRACI_ID_LIST: u8                       = 0x00;
pub const ID_COUNT: u8                            = 0x01;
pub const AUTOMATIC_VARIABLES_SUBSCRIPTION: u8   = 0x02;
pub const AUTOMATIC_CONTEXT_SUBSCRIPTION: u8     = 0x03;
pub const GENERIC_ATTRIBUTE: u8                  = 0x03;
pub const LAST_STEP_VEHICLE_NUMBER: u8            = 0x10;
pub const LAST_STEP_MEAN_SPEED: u8                = 0x11;
pub const LAST_STEP_VEHICLE_ID_LIST: u8           = 0x12;
pub const LAST_STEP_OCCUPANCY: u8                 = 0x13;
pub const LAST_STEP_VEHICLE_HALTING_NUMBER: u8    = 0x14;
pub const LAST_STEP_LENGTH: u8                    = 0x15;
pub const LAST_STEP_TIME_SINCE_DETECTION: u8      = 0x16;
pub const LAST_STEP_VEHICLE_DATA: u8              = 0x17;
pub const LAST_STEP_PERSON_ID_LIST: u8            = 0x1a;
pub const VAR_NAME: u8                            = 0x1b;
pub const VAR_FOLLOW_SPEED: u8                    = 0x1c;
pub const VAR_STOP_SPEED: u8                      = 0x1d;
pub const VAR_SECURE_GAP: u8                      = 0x1e;
pub const VAR_STOP_DELAY: u8                      = 0x1f;
pub const JAM_LENGTH_VEHICLE: u8                  = 0x18;
pub const JAM_LENGTH_METERS: u8                   = 0x19;
pub const TL_RED_YELLOW_GREEN_STATE: u8           = 0x20;
pub const TL_PHASE_INDEX: u8                      = 0x22;
pub const TL_PROGRAM: u8                          = 0x23;
pub const TL_PHASE_DURATION: u8                   = 0x24;
pub const TL_CONTROLLED_LANES: u8                 = 0x26;
pub const TL_CONTROLLED_LINKS: u8                 = 0x27;
pub const TL_CURRENT_PHASE: u8                    = 0x28;
pub const TL_CURRENT_PROGRAM: u8                  = 0x29;
pub const TL_CONTROLLED_JUNCTIONS: u8             = 0x2a;
pub const TL_COMPLETE_DEFINITION_RYG: u8          = 0x2b;
pub const TL_COMPLETE_PROGRAM_RYG: u8             = 0x2c;
pub const TL_NEXT_SWITCH: u8                      = 0x2d;
pub const TL_SPENT_DURATION: u8                   = 0x38;
pub const LANE_LINK_NUMBER: u8                    = 0x30;
pub const LANE_EDGE_ID: u8                        = 0x31;
pub const LANE_LINKS: u8                          = 0x33;
pub const LANE_ALLOWED: u8                        = 0x34;
pub const LANE_DISALLOWED: u8                     = 0x35;
pub const VAR_SLOPE: u8                           = 0x36;
pub const VAR_FOES: u8                            = 0x37;
pub const VAR_OPTION: u8                          = 0x32;
pub const VAR_SPEED: u8                           = 0x40;
pub const VAR_MAXSPEED: u8                        = 0x41;
pub const VAR_POSITION: u8                        = 0x42;
pub const VAR_ANGLE: u8                           = 0x43;
pub const VAR_LENGTH: u8                          = 0x44;
pub const VAR_COLOR: u8                           = 0x45;
/// Instantaneous acceleration from the last simulation step (m/s²).
/// Use this constant with vehicle-type subscription commands.
pub const VAR_ACCEL: u8                           = 0x46;
pub const VAR_DECEL: u8                           = 0x47;
pub const VAR_TAU: u8                             = 0x48;
pub const VAR_VEHICLECLASS: u8                    = 0x49;
pub const VAR_EMISSIONCLASS: u8                   = 0x4a;
pub const VAR_SHAPECLASS: u8                      = 0x4b;
pub const VAR_MINGAP: u8                          = 0x4c;
pub const VAR_WIDTH: u8                           = 0x4d;
pub const VAR_SHAPE: u8                           = 0x4e;
pub const VAR_TYPE: u8                            = 0x4f;
pub const VAR_ROAD_ID: u8                         = 0x50;
pub const VAR_LANE_ID: u8                         = 0x51;
pub const VAR_LANE_INDEX: u8                      = 0x52;
pub const VAR_ROUTE_ID: u8                        = 0x53;
pub const VAR_EDGES: u8                           = 0x54;
pub const VAR_FILL: u8                            = 0x55;
pub const VAR_LANEPOSITION: u8                    = 0x56;
pub const VAR_ROUTE: u8                           = 0x57;
pub const VAR_EDGE_TRAVELTIME: u8                 = 0x58;
pub const VAR_EDGE_EFFORT: u8                     = 0x59;
pub const VAR_CURRENT_TRAVELTIME: u8              = 0x5a;
pub const VAR_SIGNALS: u8                         = 0x5b;
pub const VAR_MOVE_TO: u8                         = 0x5c;
pub const VAR_IMPERFECTION: u8                    = 0x5d;
pub const VAR_SPEED_FACTOR: u8                    = 0x5e;
pub const VAR_SPEED_DEVIATION: u8                 = 0x5f;
pub const VAR_CO2EMISSION: u8                     = 0x60;
pub const VAR_COEMISSION: u8                      = 0x61;
pub const VAR_HCEMISSION: u8                      = 0x62;
pub const VAR_PMXEMISSION: u8                     = 0x63;
pub const VAR_NOXEMISSION: u8                     = 0x64;
pub const VAR_FUELCONSUMPTION: u8                 = 0x65;
pub const VAR_NOISEEMISSION: u8                   = 0x66;
pub const VAR_PERSON_NUMBER: u8                   = 0x67;
pub const VAR_LEADER: u8                          = 0x68;
pub const VAR_ROUTE_INDEX: u8                     = 0x69;
pub const VAR_WAITING_TIME: u8                    = 0x7a;
pub const VAR_ACCUMULATED_WAITING_TIME: u8        = 0x87;
pub const VAR_NEXT_TLS: u8                        = 0x70;
pub const VAR_TIME: u8                            = 0x66;
pub const VAR_TIME_STEP: u8                       = 0x70;
pub const VAR_ELECTRICITYCONSUMPTION: u8          = 0x71;
pub const VAR_LOADED_VEHICLES_NUMBER: u8          = 0x71;
pub const VAR_LOADED_VEHICLES_IDS: u8             = 0x72;
pub const VAR_DEPARTED_VEHICLES_NUMBER: u8        = 0x73;
pub const VAR_DEPARTED_VEHICLES_IDS: u8           = 0x74;
pub const VAR_TELEPORT_STARTING_VEHICLES_NUMBER: u8  = 0x75;
pub const VAR_TELEPORT_STARTING_VEHICLES_IDS: u8     = 0x76;
pub const VAR_TELEPORT_ENDING_VEHICLES_NUMBER: u8    = 0x77;
pub const VAR_TELEPORT_ENDING_VEHICLES_IDS: u8       = 0x78;
pub const VAR_ARRIVED_VEHICLES_NUMBER: u8         = 0x79;
pub const VAR_ARRIVED_VEHICLES_IDS: u8            = 0x7a;
pub const VAR_DELTA_T: u8                         = 0x7b;
pub const VAR_NET_BOUNDING_BOX: u8                = 0x7c;
pub const VAR_MIN_EXPECTED_VEHICLES: u8           = 0x7d;
pub const VAR_EMERGENCY_DECEL: u8                 = 0x7b;
pub const VAR_APPARENT_DECEL: u8                  = 0x7c;
pub const VAR_PARAMETER: u8                       = 0x7e;
pub const VAR_PARAMETER_WITH_KEY: u8              = 0x3e;
pub const VAR_SPEED_WITHOUT_TRACI: u8             = 0xb1;
pub const VAR_BEST_LANES: u8                      = 0xb2;
pub const VAR_SPEEDSETMODE: u8                    = 0xb3;
pub const MOVE_TO_XY: u8                          = 0xb4;
pub const VAR_STOPSTATE: u8                       = 0xb5;
pub const VAR_LANECHANGE_MODE: u8                 = 0xb6;
pub const VAR_ALLOWED_SPEED: u8                   = 0xb7;
pub const VAR_LANEPOSITION_LAT: u8                = 0xb8;
pub const VAR_LATALIGNMENT: u8                    = 0xb9;
pub const VAR_MAXSPEED_LAT: u8                    = 0xba;
pub const VAR_MINGAP_LAT: u8                      = 0xbb;
pub const VAR_HEIGHT: u8                          = 0xbc;
pub const VAR_LINE: u8                            = 0xbd;
pub const VAR_VIA: u8                             = 0xbe;
pub const VAR_ROUTING_MODE: u8                    = 0x89;
pub const VAR_POSITION3D: u8                      = 0x39;
pub const VAR_FOLLOWER: u8                        = 0x78;
pub const VAR_SPEED_LAT: u8                       = 0x32;
/// Signed acceleration as reported by `get_acceleration` (m/s²).
/// Positive = accelerating, negative = braking. Use this constant
/// with `subscribe_kinematics`.
pub const VAR_ACCELERATION: u8                    = 0x72;
pub const VAR_DISTANCE: u8                        = 0x84;
pub const VAR_STAGE: u8                           = 0xc0;
pub const VAR_NEXT_EDGE: u8                       = 0xc1;
pub const VAR_STAGES_REMAINING: u8                = 0xc2;
pub const VAR_VEHICLE: u8                         = 0xc3;
pub const APPEND_STAGE: u8                        = 0xc4;
pub const REMOVE_STAGE: u8                        = 0xc5;
pub const VAR_TAXI_RESERVATIONS: u8               = 0xc6;
pub const VAR_SAMPLE_LAST: u8                     = 0x20;
pub const VAR_SAMPLE_CURRENT: u8                  = 0x21;
pub const VAR_VIEW_ZOOM: u8                       = 0xa0;
pub const VAR_VIEW_OFFSET: u8                     = 0xa1;
pub const VAR_VIEW_SCHEMA: u8                     = 0xa2;
pub const VAR_VIEW_BOUNDARY: u8                   = 0xa3;
pub const VAR_SCREENSHOT: u8                      = 0xa5;
pub const VAR_TRACK_VEHICLE: u8                   = 0xa6;
pub const ADD: u8                                 = 0x80;
pub const REMOVE: u8                              = 0x81;
pub const COPY: u8                                = 0x88;
pub const POSITION_CONVERSION: u8                 = 0x82;
pub const DISTANCE_REQUEST: u8                    = 0x83;
pub const ADD_FULL: u8                            = 0x85;
pub const FIND_ROUTE: u8                          = 0x86;
pub const CMD_REROUTE_TRAVELTIME: u8              = 0x90;
pub const CMD_REROUTE_EFFORT: u8                  = 0x91;
pub const VAR_ROUTE_VALID: u8                     = 0x92;
pub const VAR_IMAGEFILE: u8                       = 0x93;
pub const VAR_BUS_STOP_WAITING: u8                = 0x67;
pub const VAR_BUS_STOP_WAITING_IDS: u8            = 0xef;
pub const VAR_DEPARTED_PERSONS_NUMBER: u8         = 0x24;
pub const VAR_DEPARTED_PERSONS_IDS: u8            = 0x25;
pub const VAR_ARRIVED_PERSONS_NUMBER: u8          = 0x26;
pub const VAR_ARRIVED_PERSONS_IDS: u8             = 0x27;
pub const CMD_MESSAGE: u8                         = 0x65;
pub const VAR_PERSON_CAPACITY: u8                 = 0x38;
pub const FROM_JUNCTION: u8                       = 0x7b;
pub const TO_JUNCTION: u8                         = 0x7c;
pub const INCOMING_EDGES: u8                      = 0x7b;
pub const OUTGOING_EDGES: u8                      = 0x7c;
pub const VAR_BIDI: u8                            = 0x7f;
pub const VAR_LAST_INTERVAL_NUMBER: u8            = 0x29;
pub const VAR_LANES: u8                           = 0x30;
pub const VAR_EXIT_LANES: u8                      = 0x31;
pub const VAR_EXIT_POSITIONS: u8                  = 0x43;
pub const CMD_SAVE_SIMSTATE: u8                   = 0x95;
pub const CMD_LOAD_SIMSTATE: u8                   = 0x96;
pub const CMD_CLEAR_PENDING_VEHICLES: u8          = 0x94;
pub const VAR_PENDING_VEHICLES: u8                = 0x94;
pub const FIND_INTERMODAL_ROUTE: u8               = 0x87;
pub const VAR_TIMELOSS: u8                        = 0x8c;
pub const VAR_STOP_ARRIVALDELAY: u8               = 0x22;
pub const VAR_PREV_SPEED: u8                      = 0x3c;
