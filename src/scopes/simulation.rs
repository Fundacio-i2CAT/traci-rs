// SPDX-License-Identifier: EPL-2.0
//! TraCI Simulation domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{
        ContextSubscriptionResults, SubscriptionResults, TraciPosition, TraciRoadPosition,
        TraciStage,
    },
};

/// Scope for querying and controlling the SUMO simulation.
#[derive(Debug, Default)]
pub struct SimulationScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl SimulationScope {
    crate::impl_scope_accessors!();

    // -----------------------------------------------------------------------
    // Simple getters
    // -----------------------------------------------------------------------

    pub fn get_current_time(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_TIME_STEP, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_time(&self, client: &mut TraciClient) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_TIME, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_loaded_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_LOADED_VEHICLES_NUMBER, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_loaded_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_LOADED_VEHICLES_IDS, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_departed_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_DEPARTED_VEHICLES_NUMBER, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_departed_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_DEPARTED_VEHICLES_IDS, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_arrived_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_ARRIVED_VEHICLES_NUMBER, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_arrived_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_ARRIVED_VEHICLES_IDS, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_starting_teleport_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_TELEPORT_STARTING_VEHICLES_NUMBER, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_starting_teleport_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_TELEPORT_STARTING_VEHICLES_IDS, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_ending_teleport_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_TELEPORT_ENDING_VEHICLES_NUMBER, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_ending_teleport_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_TELEPORT_ENDING_VEHICLES_IDS, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_departed_person_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_DEPARTED_PERSONS_NUMBER, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_departed_person_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_DEPARTED_PERSONS_IDS, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_arrived_person_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_ARRIVED_PERSONS_NUMBER, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_arrived_person_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_ARRIVED_PERSONS_IDS, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_delta_t(&self, client: &mut TraciClient) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_DELTA_T, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_net_boundary(&self, client: &mut TraciClient) -> Result<Vec<crate::types::TraciPosition>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_NET_BOUNDING_BOX, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_POLYGON))?;
        client.read_polygon_from_input()
    }

    pub fn get_min_expected_number(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_MIN_EXPECTED_VEHICLES, "", None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_option(&self, client: &mut TraciClient, option: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_OPTION, option, None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_bus_stop_waiting(&self, client: &mut TraciClient, stop_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_BUS_STOP_WAITING, stop_id, None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_bus_stop_waiting_id_list(&self, client: &mut TraciClient, stop_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_SIM_VARIABLE, VAR_BUS_STOP_WAITING_IDS, stop_id, None);
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    // -----------------------------------------------------------------------
    // Position conversion
    // -----------------------------------------------------------------------

    /// Convert a road position (edge + offset + lane) to a 2-D or geographic position.
    pub fn convert2d(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        pos: f64,
        lane_index: i32,
        to_geo: bool,
    ) -> Result<TraciPosition, TraciError> {
        let pos_type = if to_geo { POSITION_LON_LAT } else { POSITION_2D };
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(POSITION_ROADMAP);
        add.write_string(edge_id);
        add.write_f64(pos);
        add.write_u8(lane_index as u8);
        add.write_u8(TYPE_UBYTE);
        add.write_u8(pos_type);
        client.create_command(CMD_GET_SIM_VARIABLE, POSITION_CONVERSION, "", Some(&add));
        client.process_get(CMD_GET_SIM_VARIABLE, Some(pos_type))?;
        let x = client.read_double_from_input()?;
        let y = client.read_double_from_input()?;
        Ok(TraciPosition::new_2d(x, y))
    }

    /// Convert a road position to a 3-D or geographic+alt position.
    pub fn convert3d(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        pos: f64,
        lane_index: i32,
        to_geo: bool,
    ) -> Result<TraciPosition, TraciError> {
        let pos_type = if to_geo { POSITION_LON_LAT_ALT } else { POSITION_3D };
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(POSITION_ROADMAP);
        add.write_string(edge_id);
        add.write_f64(pos);
        add.write_u8(lane_index as u8);
        add.write_u8(TYPE_UBYTE);
        add.write_u8(pos_type);
        client.create_command(CMD_GET_SIM_VARIABLE, POSITION_CONVERSION, "", Some(&add));
        client.process_get(CMD_GET_SIM_VARIABLE, Some(pos_type))?;
        let x = client.read_double_from_input()?;
        let y = client.read_double_from_input()?;
        let z = client.read_double_from_input()?;
        Ok(TraciPosition::new_3d(x, y, z))
    }

    /// Convert a 2-D (or geo) position to a road position.
    pub fn convert_road(
        &self,
        client: &mut TraciClient,
        x: f64,
        y: f64,
        is_geo: bool,
        v_class: &str,
    ) -> Result<TraciRoadPosition, TraciError> {
        let src_pos_type = if is_geo { POSITION_LON_LAT } else { POSITION_2D };
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(3);
        add.write_u8(src_pos_type);
        add.write_f64(x);
        add.write_f64(y);
        add.write_u8(TYPE_UBYTE);
        add.write_u8(POSITION_ROADMAP);
        add.write_u8(TYPE_STRING);
        add.write_string(v_class);
        client.create_command(CMD_GET_SIM_VARIABLE, POSITION_CONVERSION, "", Some(&add));
        client.process_get(CMD_GET_SIM_VARIABLE, Some(POSITION_ROADMAP))?;
        let edge_id = client.read_string_from_input()?;
        let pos = client.read_double_from_input()?;
        let lane_index = client.read_ubyte_from_input()? as i32;
        Ok(TraciRoadPosition { edge_id, pos, lane_index })
    }

    /// Convert between geographic and Cartesian positions.
    pub fn convert_geo(
        &self,
        client: &mut TraciClient,
        x: f64,
        y: f64,
        from_geo: bool,
    ) -> Result<TraciPosition, TraciError> {
        let pos_type = if from_geo { POSITION_2D } else { POSITION_LON_LAT };
        let src_pos_type = if from_geo { POSITION_LON_LAT } else { POSITION_2D };
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(src_pos_type);
        add.write_f64(x);
        add.write_f64(y);
        add.write_u8(TYPE_UBYTE);
        add.write_u8(pos_type);
        client.create_command(CMD_GET_SIM_VARIABLE, POSITION_CONVERSION, "", Some(&add));
        client.process_get(CMD_GET_SIM_VARIABLE, Some(pos_type))?;
        let rx = client.read_double_from_input()?;
        let ry = client.read_double_from_input()?;
        Ok(TraciPosition::new_2d(rx, ry))
    }

    // -----------------------------------------------------------------------
    // Distance queries
    // -----------------------------------------------------------------------

    /// Get the distance between two 2-D (or geo) positions.
    pub fn get_distance_2d(
        &self,
        client: &mut TraciClient,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        is_geo: bool,
        is_driving: bool,
    ) -> Result<f64, TraciError> {
        let pos_type = if is_geo { POSITION_LON_LAT } else { POSITION_2D };
        let dist_type: u8 = if is_driving { REQUEST_DRIVINGDIST } else { REQUEST_AIRDIST };
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(3);
        add.write_u8(pos_type);
        add.write_f64(x1);
        add.write_f64(y1);
        add.write_u8(pos_type);
        add.write_f64(x2);
        add.write_f64(y2);
        add.write_u8(dist_type);
        client.create_command(CMD_GET_SIM_VARIABLE, DISTANCE_REQUEST, "", Some(&add));
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    /// Get the distance between two road positions.
    pub fn get_distance_road(
        &self,
        client: &mut TraciClient,
        edge_id1: &str,
        pos1: f64,
        edge_id2: &str,
        pos2: f64,
        is_driving: bool,
    ) -> Result<f64, TraciError> {
        let dist_type: u8 = if is_driving { REQUEST_DRIVINGDIST } else { REQUEST_AIRDIST };
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(3);
        add.write_u8(POSITION_ROADMAP);
        add.write_string(edge_id1);
        add.write_f64(pos1);
        add.write_u8(0); // lane
        add.write_u8(POSITION_ROADMAP);
        add.write_string(edge_id2);
        add.write_f64(pos2);
        add.write_u8(0); // lane
        add.write_u8(dist_type);
        client.create_command(CMD_GET_SIM_VARIABLE, DISTANCE_REQUEST, "", Some(&add));
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    // -----------------------------------------------------------------------
    // Route finding
    // -----------------------------------------------------------------------

    /// Find a route between two edges, returning a [`TraciStage`].
    pub fn find_route(
        &self,
        client: &mut TraciClient,
        from_edge: &str,
        to_edge: &str,
        v_type: &str,
        pos: f64,
        routing_mode: i32,
    ) -> Result<TraciStage, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(5);
        add.write_u8(TYPE_STRING);
        add.write_string(from_edge);
        add.write_u8(TYPE_STRING);
        add.write_string(to_edge);
        add.write_u8(TYPE_STRING);
        add.write_string(v_type);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(pos);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(routing_mode);
        client.create_command(CMD_GET_SIM_VARIABLE, FIND_ROUTE, "", Some(&add));
        client.process_get(CMD_GET_SIM_VARIABLE, Some(TYPE_COMPOUND))?;
        read_traci_stage(client)
    }

    // -----------------------------------------------------------------------
    // State management
    // -----------------------------------------------------------------------

    pub fn load_state(&self, client: &mut TraciClient, path: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(path);
        client.create_command(CMD_SET_SIM_VARIABLE, CMD_LOAD_SIMSTATE, "", Some(&add));
        client.process_set(CMD_SET_SIM_VARIABLE)?;
        Ok(())
    }

    pub fn save_state(&self, client: &mut TraciClient, destination: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(destination);
        client.create_command(CMD_SET_SIM_VARIABLE, CMD_SAVE_SIMSTATE, "", Some(&add));
        client.process_set(CMD_SET_SIM_VARIABLE)?;
        Ok(())
    }

    pub fn write_message(&self, client: &mut TraciClient, msg: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(msg);
        client.create_command(CMD_SET_SIM_VARIABLE, CMD_MESSAGE, "", Some(&add));
        client.process_set(CMD_SET_SIM_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_SIM_VARIABLE, "", begin, end, vars)
    }
}

// ============================================================================
// Shared helper: read a TraciStage from client.input (TYPE_COMPOUND already consumed)
// ============================================================================

/// Read a `TraciStage` from the input buffer. The TYPE_COMPOUND tag and expected-type
/// check have already been performed by `process_get`; the next bytes are the payload.
pub(crate) fn read_traci_stage(client: &mut TraciClient) -> Result<TraciStage, TraciError> {
    client.read_int_from_input()?; // components count
    client.read_ubyte_from_input()?; // TYPE_INTEGER tag
    let type_ = client.read_int_from_input()?;

    client.read_ubyte_from_input()?; // TYPE_STRING tag
    let v_type = client.read_string_from_input()?;

    client.read_ubyte_from_input()?;
    let line = client.read_string_from_input()?;

    client.read_ubyte_from_input()?;
    let dest_stop = client.read_string_from_input()?;

    client.read_ubyte_from_input()?;
    let edges = client.read_string_list_from_input()?;

    client.read_ubyte_from_input()?;
    let travel_time = client.read_double_from_input()?;

    client.read_ubyte_from_input()?;
    let cost = client.read_double_from_input()?;

    client.read_ubyte_from_input()?;
    let length = client.read_double_from_input()?;

    client.read_ubyte_from_input()?;
    let intended = client.read_string_from_input()?;

    client.read_ubyte_from_input()?;
    let depart = client.read_double_from_input()?;

    client.read_ubyte_from_input()?;
    let depart_pos = client.read_double_from_input()?;

    client.read_ubyte_from_input()?;
    let arrival_pos = client.read_double_from_input()?;

    client.read_ubyte_from_input()?;
    let description = client.read_string_from_input()?;

    Ok(TraciStage {
        type_,
        v_type,
        line,
        dest_stop,
        edges,
        travel_time,
        cost,
        length,
        intended,
        depart,
        depart_pos,
        arrival_pos,
        description,
    })
}
