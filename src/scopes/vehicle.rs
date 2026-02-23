// SPDX-License-Identifier: EPL-2.0
//! TraCI Vehicle domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{
        ContextSubscriptionResults, SubscriptionResults, TraciBestLanesData, TraciColor,
        TraciNextTLSData, TraciPosition,
    },
};

// ---------------------------------------------------------------------------
// Vehicle signal constants
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub const SIGNAL_BLINKER_RIGHT: i32    = 1;
#[allow(dead_code)]
pub const SIGNAL_BLINKER_LEFT: i32     = 2;
#[allow(dead_code)]
pub const SIGNAL_BLINKER_EMERGENCY: i32 = 4;
#[allow(dead_code)]
pub const SIGNAL_BRAKELIGHT: i32       = 8;
#[allow(dead_code)]
pub const SIGNAL_FRONTLIGHT: i32       = 16;
#[allow(dead_code)]
pub const SIGNAL_FOGLIGHT: i32         = 32;
#[allow(dead_code)]
pub const SIGNAL_HIGHBEAM: i32         = 64;
#[allow(dead_code)]
pub const SIGNAL_BACKDRIVE: i32        = 128;
#[allow(dead_code)]
pub const SIGNAL_WIPER: i32            = 256;
#[allow(dead_code)]
pub const SIGNAL_DOOR_OPEN_LEFT: i32   = 512;
#[allow(dead_code)]
pub const SIGNAL_DOOR_OPEN_RIGHT: i32  = 1024;
#[allow(dead_code)]
pub const SIGNAL_EMERGENCY_BLUE: i32   = 2048;
#[allow(dead_code)]
pub const SIGNAL_EMERGENCY_RED: i32    = 4096;
#[allow(dead_code)]
pub const SIGNAL_EMERGENCY_YELLOW: i32 = 8192;
#[allow(dead_code)]
pub const SIGNAL_RESET: i32            = -1;

// ---------------------------------------------------------------------------
// Scope struct
// ---------------------------------------------------------------------------

/// Scope for interacting with SUMO vehicle objects.
#[derive(Debug, Default)]
pub struct VehicleScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl VehicleScope {
    crate::impl_scope_accessors!();

    // -----------------------------------------------------------------------
    // Generic getters
    // -----------------------------------------------------------------------

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, vehicle_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_PARAMETER, vehicle_id, Some(&add));
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, vehicle_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_PARAMETER, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // State getters (simple)
    // -----------------------------------------------------------------------

    pub fn get_speed(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SPEED, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_lateral_speed(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SPEED_LAT, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_acceleration(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ACCELERATION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_position(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<TraciPosition, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_POSITION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(POSITION_2D))?;
        client.read_pos_2d_from_input()
    }

    pub fn get_position3d(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<TraciPosition, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_POSITION3D, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(POSITION_3D))?;
        client.read_pos_3d_from_input()
    }

    pub fn get_angle(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ANGLE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_road_id(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ROAD_ID, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_lane_id(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LANE_ID, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_lane_index(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LANE_INDEX, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_type_id(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_TYPE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_route_id(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ROUTE_ID, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_route_index(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ROUTE_INDEX, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_route(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_EDGES, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_color(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<TraciColor, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_COLOR, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_COLOR))?;
        client.read_color_from_input()
    }

    pub fn get_lane_position(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LANEPOSITION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_distance(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_DISTANCE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_signals(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SIGNALS, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_lateral_lane_position(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LANEPOSITION_LAT, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_co2_emission(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_CO2EMISSION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_co_emission(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_COEMISSION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_hc_emission(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_HCEMISSION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_pmx_emission(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_PMXEMISSION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_nox_emission(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_NOXEMISSION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_fuel_consumption(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_FUELCONSUMPTION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_noise_emission(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_NOISEEMISSION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_electricity_consumption(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ELECTRICITYCONSUMPTION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_waiting_time(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_WAITING_TIME, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_accumulated_waiting_time(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ACCUMULATED_WAITING_TIME, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_lane_change_mode(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LANECHANGE_MODE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_speed_mode(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SPEEDSETMODE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_slope(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SLOPE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_line(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LINE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_via(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_VIA, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_stop_state(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_STOPSTATE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_routing_mode(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ROUTING_MODE, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_stop_delay(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_STOP_DELAY, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_stop_arrival_delay(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_STOP_ARRIVALDELAY, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_speed_without_traci(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SPEED_WITHOUT_TRACI, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn is_route_valid(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<bool, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ROUTE_VALID, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        Ok(client.read_int_from_input()? != 0)
    }

    pub fn get_allowed_speed(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ALLOWED_SPEED, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_person_number(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_PERSON_NUMBER, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_person_capacity(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_PERSON_CAPACITY, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_person_id_list(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, LAST_STEP_PERSON_ID_LIST, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    // VehicleType attribute shortcuts
    pub fn get_max_speed(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_MAXSPEED, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_accel(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_ACCEL, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_decel(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_DECEL, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_tau(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_TAU, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_imperfection(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_IMPERFECTION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_speed_factor(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SPEED_FACTOR, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_speed_deviation(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SPEED_DEVIATION, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_vehicle_class(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_VEHICLECLASS, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_min_gap(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_MINGAP, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_width(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_WIDTH, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_length(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LENGTH, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_height(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_HEIGHT, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_max_speed_lat(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_MAXSPEED_LAT, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_min_gap_lat(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_MINGAP_LAT, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_lateral_alignment(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LATALIGNMENT, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_emission_class(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_EMISSIONCLASS, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_shape_class(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SHAPECLASS, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    // -----------------------------------------------------------------------
    // Compound getters
    // -----------------------------------------------------------------------

    /// Get upcoming traffic-light data for a vehicle.
    pub fn get_next_tls(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<Vec<TraciNextTLSData>, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_NEXT_TLS, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_COMPOUND))?;
        client.read_int_from_input()?; // skip components count
        client.read_ubyte_from_input()?; // skip inner type tag
        let n = client.read_int_from_input()?;
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            client.read_ubyte_from_input()?;
            let id = client.read_string_from_input()?;
            client.read_ubyte_from_input()?;
            let tl_index = client.read_int_from_input()?;
            client.read_ubyte_from_input()?;
            let dist = client.read_double_from_input()?;
            client.read_ubyte_from_input()?;
            let state_byte = client.read_ubyte_from_input()? as i8 as u8;
            let state = state_byte as char;
            result.push(TraciNextTLSData { id, tl_index, dist, state });
        }
        Ok(result)
    }

    /// Get best-lane information for a vehicle.
    pub fn get_best_lanes(&self, client: &mut TraciClient, vehicle_id: &str) -> Result<Vec<TraciBestLanesData>, TraciError> {
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_BEST_LANES, vehicle_id, None);
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_COMPOUND))?;
        client.read_int_from_input()?; // skip total byte count
        client.read_ubyte_from_input()?; // skip inner type tag
        let n = client.read_int_from_input()?;
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            client.read_ubyte_from_input()?;
            let lane_id = client.read_string_from_input()?;
            client.read_ubyte_from_input()?;
            let length = client.read_double_from_input()?;
            client.read_ubyte_from_input()?;
            let occupation = client.read_double_from_input()?;
            client.read_ubyte_from_input()?;
            // bestLaneOffset is a signed byte
            let best_lane_offset = client.read_ubyte_from_input()? as i8 as i32;
            client.read_ubyte_from_input()?;
            let allows_continuation = client.read_ubyte_from_input()? == 1;
            client.read_ubyte_from_input()?;
            let m = client.read_int_from_input()?;
            let mut continuation_lanes = Vec::with_capacity(m as usize);
            for _ in 0..m {
                continuation_lanes.push(client.read_string_from_input()?);
            }
            result.push(TraciBestLanesData {
                lane_id,
                length,
                occupation,
                best_lane_offset,
                allows_continuation,
                continuation_lanes,
            });
        }
        Ok(result)
    }

    /// Get the leading vehicle and gap ahead of a vehicle.
    pub fn get_leader(&self, client: &mut TraciClient, vehicle_id: &str, dist: f64) -> Result<(String, f64), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(dist);
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_LEADER, vehicle_id, Some(&add));
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_COMPOUND))?;
        client.read_int_from_input()?; // components
        client.read_ubyte_from_input()?;
        let leader_id = client.read_string_from_input()?;
        client.read_ubyte_from_input()?;
        let gap = client.read_double_from_input()?;
        Ok((leader_id, gap))
    }

    /// Get the following vehicle and gap behind a vehicle.
    pub fn get_follower(&self, client: &mut TraciClient, vehicle_id: &str, dist: f64) -> Result<(String, f64), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(dist);
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_FOLLOWER, vehicle_id, Some(&add));
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_COMPOUND))?;
        client.read_int_from_input()?; // components
        client.read_ubyte_from_input()?;
        let follower_id = client.read_string_from_input()?;
        client.read_ubyte_from_input()?;
        let gap = client.read_double_from_input()?;
        Ok((follower_id, gap))
    }

    /// Get lane change state for a given direction.
    /// Returns `(state_without_traci, state)`.
    pub fn get_lane_change_state(&self, client: &mut TraciClient, vehicle_id: &str, direction: i32) -> Result<(i32, i32), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(direction);
        client.create_command(CMD_GET_VEHICLE_VARIABLE, CMD_CHANGELANE, vehicle_id, Some(&add));
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_COMPOUND))?;
        client.read_int_from_input()?; // components
        client.read_ubyte_from_input()?;
        let state_without_traci = client.read_int_from_input()?;
        client.read_ubyte_from_input()?;
        let state = client.read_int_from_input()?;
        Ok((state_without_traci, state))
    }

    /// Get the safe follow speed.
    pub fn get_follow_speed(&self, client: &mut TraciClient, vehicle_id: &str, speed: f64, gap: f64, leader_speed: f64, leader_max_decel: f64, leader_id: &str) -> Result<f64, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(5);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(gap);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(leader_speed);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(leader_max_decel);
        add.write_u8(TYPE_STRING);
        add.write_string(leader_id);
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_FOLLOW_SPEED, vehicle_id, Some(&add));
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    /// Get the safe gap to keep to a leader.
    pub fn get_secure_gap(&self, client: &mut TraciClient, vehicle_id: &str, speed: f64, leader_speed: f64, leader_max_decel: f64, leader_id: &str) -> Result<f64, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(4);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(leader_speed);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(leader_max_decel);
        add.write_u8(TYPE_STRING);
        add.write_string(leader_id);
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_SECURE_GAP, vehicle_id, Some(&add));
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    /// Get the maximum safe speed to reach a stop with given gap.
    pub fn get_stop_speed(&self, client: &mut TraciClient, vehicle_id: &str, speed: f64, gap: f64) -> Result<f64, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(gap);
        client.create_command(CMD_GET_VEHICLE_VARIABLE, VAR_STOP_SPEED, vehicle_id, Some(&add));
        client.process_get(CMD_GET_VEHICLE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    // -----------------------------------------------------------------------
    // Vehicle setters / commands
    // -----------------------------------------------------------------------

    /// Add a new vehicle to the simulation.
    ///
    /// If `depart` is `"-1"` the current simulation time is used.
    pub fn add(
        &self,
        client: &mut TraciClient,
        vehicle_id: &str,
        route_id: &str,
        type_id: &str,
        mut depart: String,
        depart_lane: &str,
        depart_pos: &str,
        depart_speed: &str,
        arrival_lane: &str,
        arrival_pos: &str,
        arrival_speed: &str,
        from_taz: &str,
        to_taz: &str,
        line: &str,
        person_capacity: i32,
        person_number: i32,
    ) -> Result<(), TraciError> {
        if depart == "-1" {
            // Get current time inline to avoid borrow issues
            client.create_command(CMD_GET_SIM_VARIABLE, VAR_TIME_STEP, "", None);
            client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
            let time_ms = client.read_int_from_input()?;
            depart = format!("{:.2}", time_ms as f64 / 1000.0);
        }
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(14);
        add.write_u8(TYPE_STRING); add.write_string(route_id);
        add.write_u8(TYPE_STRING); add.write_string(type_id);
        add.write_u8(TYPE_STRING); add.write_string(&depart);
        add.write_u8(TYPE_STRING); add.write_string(depart_lane);
        add.write_u8(TYPE_STRING); add.write_string(depart_pos);
        add.write_u8(TYPE_STRING); add.write_string(depart_speed);
        add.write_u8(TYPE_STRING); add.write_string(arrival_lane);
        add.write_u8(TYPE_STRING); add.write_string(arrival_pos);
        add.write_u8(TYPE_STRING); add.write_string(arrival_speed);
        add.write_u8(TYPE_STRING); add.write_string(from_taz);
        add.write_u8(TYPE_STRING); add.write_string(to_taz);
        add.write_u8(TYPE_STRING); add.write_string(line);
        add.write_u8(TYPE_INTEGER); add.write_i32(person_capacity);
        add.write_u8(TYPE_INTEGER); add.write_i32(person_number);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, ADD_FULL, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn remove(&self, client: &mut TraciClient, vehicle_id: &str, reason: u8) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_BYTE);
        add.write_u8(reason);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, REMOVE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn change_target(&self, client: &mut TraciClient, vehicle_id: &str, edge_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(edge_id);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_CHANGETARGET, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn change_lane(&self, client: &mut TraciClient, vehicle_id: &str, lane_index: i32, duration: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_BYTE);
        add.write_u8(lane_index as u8);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(duration);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_CHANGELANE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn change_lane_relative(&self, client: &mut TraciClient, vehicle_id: &str, lane_change: i32, duration: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(3);
        add.write_u8(TYPE_BYTE);
        add.write_u8(lane_change as u8);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(duration);
        add.write_u8(TYPE_BYTE);
        add.write_u8(1);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_CHANGELANE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn change_sublane(&self, client: &mut TraciClient, vehicle_id: &str, lat_dist: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(lat_dist);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_CHANGESUBLANE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_route_id(&self, client: &mut TraciClient, vehicle_id: &str, route_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(route_id);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_ROUTE_ID, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_route(&self, client: &mut TraciClient, vehicle_id: &str, edges: &[String]) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRINGLIST);
        add.write_i32(edges.len() as i32);
        for e in edges {
            add.write_string(e);
        }
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_ROUTE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    /// Reroute vehicle using travel-times.
    ///
    /// If `current_travel_times` is true, the current edge travel times are
    /// first fetched and adapted before issuing the reroute command.
    pub fn reroute_traveltime(&self, client: &mut TraciClient, vehicle_id: &str, current_travel_times: bool) -> Result<(), TraciError> {
        if current_travel_times {
            // Inline edge operations to avoid borrow conflicts
            client.create_command(CMD_GET_EDGE_VARIABLE, TRACI_ID_LIST, "", None);
            client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_STRINGLIST))?;
            let edges = client.read_string_list_from_input()?;
            for edge in &edges {
                // get current traveltime
                client.create_command(CMD_GET_EDGE_VARIABLE, VAR_CURRENT_TRAVELTIME, edge, None);
                client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
                let tt = client.read_double_from_input()?;
                // adapt traveltime
                let mut content = Storage::new();
                content.write_u8(TYPE_COMPOUND);
                content.write_i32(3);
                content.write_u8(TYPE_DOUBLE);
                content.write_f64(0.0);
                content.write_u8(TYPE_DOUBLE);
                content.write_f64(f64::MAX);
                content.write_u8(TYPE_DOUBLE);
                content.write_f64(tt);
                client.create_command(CMD_SET_EDGE_VARIABLE, VAR_EDGE_TRAVELTIME, edge, Some(&content));
                client.process_set(CMD_SET_EDGE_VARIABLE)?;
            }
        }
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(0);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_REROUTE_TRAVELTIME, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn move_to(&self, client: &mut TraciClient, vehicle_id: &str, lane_id: &str, position: f64, reason: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(3);
        add.write_u8(TYPE_STRING);
        add.write_string(lane_id);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(position);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(reason);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_MOVE_TO, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn move_to_xy(&self, client: &mut TraciClient, vehicle_id: &str, edge_id: &str, lane: i32, x: f64, y: f64, angle: f64, keep_route: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(6);
        add.write_u8(TYPE_STRING);
        add.write_string(edge_id);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(lane);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(x);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(y);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(angle);
        add.write_u8(TYPE_BYTE);
        add.write_u8(keep_route as u8);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, MOVE_TO_XY, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn slow_down(&self, client: &mut TraciClient, vehicle_id: &str, speed: f64, duration: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(duration);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_SLOWDOWN, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    /// Open a gap in front of the vehicle.
    ///
    /// Pass `max_decel <= 0.0` to omit the optional 4th parameter.
    pub fn open_gap(&self, client: &mut TraciClient, vehicle_id: &str, new_tau: f64, duration: f64, change_rate: f64, max_decel: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        if max_decel > 0.0 {
            add.write_i32(4);
        } else {
            add.write_i32(3);
        }
        add.write_u8(TYPE_DOUBLE); add.write_f64(new_tau);
        add.write_u8(TYPE_DOUBLE); add.write_f64(duration);
        add.write_u8(TYPE_DOUBLE); add.write_f64(change_rate);
        if max_decel > 0.0 {
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(max_decel);
        }
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_OPENGAP, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_speed(&self, client: &mut TraciClient, vehicle_id: &str, speed: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_SPEED, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_acceleration(&self, client: &mut TraciClient, vehicle_id: &str, accel: f64, duration: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_DOUBLE); add.write_f64(accel);
        add.write_u8(TYPE_DOUBLE); add.write_f64(duration);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_ACCELERATION, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_previous_speed(&self, client: &mut TraciClient, vehicle_id: &str, prev_speed: f64, prev_acceleration: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_DOUBLE); add.write_f64(prev_speed);
        add.write_u8(TYPE_DOUBLE); add.write_f64(prev_acceleration);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_PREV_SPEED, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    /// Note: uses raw byte write matching C++ `writeByte(TYPE_INTEGER)`.
    pub fn set_lane_change_mode(&self, client: &mut TraciClient, vehicle_id: &str, mode: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(mode);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_LANECHANGE_MODE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    /// Note: uses raw byte write matching C++ `writeByte(TYPE_INTEGER)`.
    pub fn set_speed_mode(&self, client: &mut TraciClient, vehicle_id: &str, mode: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(mode);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_SPEEDSETMODE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    /// Set a stop for a vehicle.
    pub fn set_stop(
        &self,
        client: &mut TraciClient,
        vehicle_id: &str,
        edge_id: &str,
        end_pos: f64,
        lane_index: i32,
        duration: f64,
        flags: i32,
        start_pos: f64,
        until: f64,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(7);
        add.write_u8(TYPE_STRING); add.write_string(edge_id);
        add.write_u8(TYPE_DOUBLE); add.write_f64(end_pos);
        add.write_u8(TYPE_BYTE);   add.write_u8(lane_index as u8);
        add.write_u8(TYPE_DOUBLE); add.write_f64(duration);
        add.write_u8(TYPE_BYTE);   add.write_u8(flags as u8);
        add.write_u8(TYPE_DOUBLE); add.write_f64(start_pos);
        add.write_u8(TYPE_DOUBLE); add.write_f64(until);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, CMD_STOP, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_type(&self, client: &mut TraciClient, vehicle_id: &str, type_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(type_id);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_TYPE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_color(&self, client: &mut TraciClient, vehicle_id: &str, c: &TraciColor) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COLOR);
        add.write_u8(c.r);
        add.write_u8(c.g);
        add.write_u8(c.b);
        add.write_u8(c.a);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_COLOR, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_line(&self, client: &mut TraciClient, vehicle_id: &str, line: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(line);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_LINE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_via(&self, client: &mut TraciClient, vehicle_id: &str, via: &[String]) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRINGLIST);
        add.write_i32(via.len() as i32);
        for v in via {
            add.write_string(v);
        }
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_VIA, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_signals(&self, client: &mut TraciClient, vehicle_id: &str, signals: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(signals);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_SIGNALS, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_routing_mode(&self, client: &mut TraciClient, vehicle_id: &str, routing_mode: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(routing_mode);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_ROUTING_MODE, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    // VehicleType attribute shortcuts
    pub fn set_shape_class(&self, client: &mut TraciClient, vehicle_id: &str, clazz: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(clazz);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_SHAPECLASS, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_emission_class(&self, client: &mut TraciClient, vehicle_id: &str, clazz: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(clazz);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_EMISSIONCLASS, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_speed_factor(&self, client: &mut TraciClient, vehicle_id: &str, factor: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(factor);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_SPEED_FACTOR, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_min_gap(&self, client: &mut TraciClient, vehicle_id: &str, min_gap: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(min_gap);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_MINGAP, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    pub fn set_max_speed(&self, client: &mut TraciClient, vehicle_id: &str, speed: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        client.create_command(CMD_SET_VEHICLE_VARIABLE, VAR_MAXSPEED, vehicle_id, Some(&add));
        client.process_set(CMD_SET_VEHICLE_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Subscriptions
    // -----------------------------------------------------------------------

    pub fn subscribe(&self, client: &mut TraciClient, vehicle_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_VEHICLE_VARIABLE, vehicle_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, vehicle_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_VEHICLE_CONTEXT, vehicle_id, begin, end, domain, range, vars)
    }

    // -----------------------------------------------------------------------
    // Subscription filters (private helpers)
    // -----------------------------------------------------------------------

    fn add_subscription_filter_empty(&self, client: &mut TraciClient, filter_type: u8) -> Result<(), TraciError> {
        client.create_filter_command(CMD_ADD_SUBSCRIPTION_FILTER, filter_type, None);
        client.process_set(CMD_ADD_SUBSCRIPTION_FILTER)?;
        Ok(())
    }

    fn add_subscription_filter_float(&self, client: &mut TraciClient, filter_type: u8, val: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(val);
        client.create_filter_command(CMD_ADD_SUBSCRIPTION_FILTER, filter_type, Some(&add));
        client.process_set(CMD_ADD_SUBSCRIPTION_FILTER)?;
        Ok(())
    }

    fn add_subscription_filter_string_list(&self, client: &mut TraciClient, filter_type: u8, vals: &[String]) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRINGLIST);
        add.write_i32(vals.len() as i32);
        for v in vals {
            add.write_string(v);
        }
        client.create_filter_command(CMD_ADD_SUBSCRIPTION_FILTER, filter_type, Some(&add));
        client.process_set(CMD_ADD_SUBSCRIPTION_FILTER)?;
        Ok(())
    }

    fn add_subscription_filter_byte_list(&self, client: &mut TraciClient, filter_type: u8, vals: &[i32]) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(vals.len() as u8);
        for &v in vals {
            add.write_u8(v as u8);
        }
        client.create_filter_command(CMD_ADD_SUBSCRIPTION_FILTER, filter_type, Some(&add));
        client.process_set(CMD_ADD_SUBSCRIPTION_FILTER)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Subscription filters (public API)
    // -----------------------------------------------------------------------

    /// Add a lane-filter to the last vehicle context subscription.
    pub fn add_subscription_filter_lanes(
        &self, client: &mut TraciClient,
        lanes: &[i32],
        no_opposite: bool,
        downstream_dist: f64,
        upstream_dist: f64,
    ) -> Result<(), TraciError> {
        self.add_subscription_filter_byte_list(client, FILTER_TYPE_LANES, lanes)?;
        if no_opposite {
            self.add_subscription_filter_empty(client, FILTER_TYPE_NOOPPOSITE)?;
        }
        if downstream_dist >= 0.0 {
            self.add_subscription_filter_float(client, FILTER_TYPE_DOWNSTREAM_DIST, downstream_dist)?;
        }
        if upstream_dist >= 0.0 {
            self.add_subscription_filter_float(client, FILTER_TYPE_UPSTREAM_DIST, upstream_dist)?;
        }
        Ok(())
    }

    /// Omit vehicles on opposite-direction lanes.
    pub fn add_subscription_filter_no_opposite(&self, client: &mut TraciClient) -> Result<(), TraciError> {
        self.add_subscription_filter_empty(client, FILTER_TYPE_NOOPPOSITE)
    }

    /// Limit the downstream distance for resulting vehicles.
    pub fn add_subscription_filter_downstream_distance(&self, client: &mut TraciClient, dist: f64) -> Result<(), TraciError> {
        self.add_subscription_filter_float(client, FILTER_TYPE_DOWNSTREAM_DIST, dist)
    }

    /// Limit the upstream distance for resulting vehicles.
    pub fn add_subscription_filter_upstream_distance(&self, client: &mut TraciClient, dist: f64) -> Result<(), TraciError> {
        self.add_subscription_filter_float(client, FILTER_TYPE_UPSTREAM_DIST, dist)
    }

    /// Restrict to leader and follower of the ego (car-following maneuver).
    pub fn add_subscription_filter_cf_maneuver(
        &self, client: &mut TraciClient,
        downstream_dist: f64,
        upstream_dist: f64,
    ) -> Result<(), TraciError> {
        self.add_subscription_filter_lead_follow(client, &[0])?;
        if downstream_dist >= 0.0 {
            self.add_subscription_filter_downstream_distance(client, downstream_dist)?;
        }
        if upstream_dist >= 0.0 {
            self.add_subscription_filter_upstream_distance(client, upstream_dist)?;
        }
        Ok(())
    }

    /// Restrict to neighbors/leader/follower for a lane-change maneuver.
    pub fn add_subscription_filter_lc_maneuver(
        &self, client: &mut TraciClient,
        direction: i32,
        no_opposite: bool,
        downstream_dist: f64,
        upstream_dist: f64,
    ) -> Result<(), TraciError> {
        if direction.abs() != 1 {
            eprintln!("Ignoring lane change subscription filter with non-neighboring lane offset direction {}", direction);
            return Ok(());
        }
        self.add_subscription_filter_lead_follow(client, &[0, direction])?;
        if no_opposite {
            self.add_subscription_filter_no_opposite(client)?;
        }
        if downstream_dist >= 0.0 {
            self.add_subscription_filter_downstream_distance(client, downstream_dist)?;
        }
        if upstream_dist >= 0.0 {
            self.add_subscription_filter_upstream_distance(client, upstream_dist)?;
        }
        Ok(())
    }

    /// Restrict to lead and follow vehicles in given lanes.
    pub fn add_subscription_filter_lead_follow(&self, client: &mut TraciClient, lanes: &[i32]) -> Result<(), TraciError> {
        self.add_subscription_filter_empty(client, FILTER_TYPE_LEAD_FOLLOW)?;
        self.add_subscription_filter_byte_list(client, FILTER_TYPE_LANES, lanes)
    }

    /// Restrict to foes on an upcoming junction.
    pub fn add_subscription_filter_turn(
        &self, client: &mut TraciClient,
        downstream_dist: f64,
        foe_dist_to_junction: f64,
    ) -> Result<(), TraciError> {
        self.add_subscription_filter_float(client, FILTER_TYPE_TURN, foe_dist_to_junction)?;
        if downstream_dist >= 0.0 {
            self.add_subscription_filter_downstream_distance(client, downstream_dist)?;
        }
        Ok(())
    }

    /// Restrict returned vehicles to given vehicle classes.
    pub fn add_subscription_filter_vclass(&self, client: &mut TraciClient, v_classes: &[String]) -> Result<(), TraciError> {
        self.add_subscription_filter_string_list(client, FILTER_TYPE_VCLASS, v_classes)
    }

    /// Restrict returned vehicles to given vehicle types.
    pub fn add_subscription_filter_vtype(&self, client: &mut TraciClient, v_types: &[String]) -> Result<(), TraciError> {
        self.add_subscription_filter_string_list(client, FILTER_TYPE_VTYPE, v_types)
    }

    /// Restrict returned vehicles to given field-of-vision angle.
    pub fn add_subscription_filter_field_of_vision(&self, client: &mut TraciClient, angle: f64) -> Result<(), TraciError> {
        self.add_subscription_filter_float(client, FILTER_TYPE_FIELD_OF_VISION, angle)
    }

    /// Restrict returned vehicles to given lateral distance.
    pub fn add_subscription_filter_lateral_distance(
        &self, client: &mut TraciClient,
        lateral_dist: f64,
        downstream_dist: f64,
        upstream_dist: f64,
    ) -> Result<(), TraciError> {
        self.add_subscription_filter_float(client, FILTER_TYPE_LATERAL_DIST, lateral_dist)?;
        if downstream_dist >= 0.0 {
            self.add_subscription_filter_downstream_distance(client, downstream_dist)?;
        }
        if upstream_dist >= 0.0 {
            self.add_subscription_filter_upstream_distance(client, upstream_dist)?;
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Kinematic subscriptions
    // -----------------------------------------------------------------------

    /// Subscribe `vehicle_id` to receive position, speed, acceleration, and
    /// heading on every simulation step, for the duration [`begin`, `end`].
    ///
    /// After each `client.simulation_step()` call the results are available
    /// via [`Self::get_subscribed_kinematics`].
    ///
    /// # Example
    /// ```no_run
    /// # use traci_rs::TraciClient;
    /// # let mut client = TraciClient::connect("localhost", 8813).unwrap();
    /// client.vehicle.subscribe_kinematics(&mut client, "veh_0", 0.0, 3600.0).unwrap();
    /// loop {
    ///     client.simulation_step(0.0).unwrap();
    ///     if let Some(k) = client.vehicle.get_subscribed_kinematics("veh_0") {
    ///         println!("pos={:?} speed={}", k.position, k.speed);
    ///     }
    /// }
    /// ```
    pub fn subscribe_kinematics(
        &self,
        client: &mut TraciClient,
        vehicle_id: &str,
        begin: f64,
        end: f64,
    ) -> Result<(), TraciError> {
        // VAR_ACCELERATION (0x72) is the signed acceleration consistent with
        // get_acceleration(); use it here for API consistency.
        let vars = [VAR_POSITION, VAR_SPEED, VAR_ACCELERATION, VAR_ANGLE];
        client.subscribe_object_variable(
            CMD_SUBSCRIBE_VEHICLE_VARIABLE,
            vehicle_id,
            begin,
            end,
            &vars,
        )
    }

    /// Read the kinematic state for `vehicle_id` from the subscription cache.
    ///
    /// Returns `None` if no subscription result is available for this vehicle
    /// (e.g. the vehicle is not yet in the simulation, or the subscription was
    /// not set up).
    pub fn get_subscribed_kinematics(
        &self,
        vehicle_id: &str,
    ) -> Option<crate::types::SubscribedKinematics> {
        let results = self.subscription_results.get(vehicle_id)?;
        let pos = match results.get(&VAR_POSITION)? {
            crate::types::TraciValue::Pos2D { x, y } => TraciPosition::new_2d(*x, *y),
            _ => return None,
        };
        let speed = match results.get(&VAR_SPEED)? {
            crate::types::TraciValue::Double(v) => *v,
            _ => return None,
        };
        let acceleration = match results.get(&VAR_ACCELERATION)? {
            crate::types::TraciValue::Double(v) => *v,
            _ => return None,
        };
        let angle = match results.get(&VAR_ANGLE)? {
            crate::types::TraciValue::Double(v) => *v,
            _ => return None,
        };
        Some(crate::types::SubscribedKinematics { position: pos, speed, acceleration, angle })
    }
}
