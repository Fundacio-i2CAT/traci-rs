// SPDX-License-Identifier: EPL-2.0
//! TraCI Person domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{
        ContextSubscriptionResults, SubscriptionResults, TraciColor, TraciPosition, TraciStage,
    },
    scopes::simulation::read_traci_stage,
};

/// Scope for interacting with SUMO person objects.
#[derive(Debug, Default)]
pub struct PersonScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl PersonScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, person_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_PARAMETER, person_id, Some(&add));
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, person_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_PARAMETER, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    pub fn get_speed(&self, client: &mut TraciClient, person_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_SPEED, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_position(&self, client: &mut TraciClient, person_id: &str) -> Result<TraciPosition, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_POSITION, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(POSITION_2D))?;
        client.read_pos_2d_from_input()
    }

    pub fn get_position3d(&self, client: &mut TraciClient, person_id: &str) -> Result<TraciPosition, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_POSITION3D, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(POSITION_3D))?;
        client.read_pos_3d_from_input()
    }

    pub fn get_angle(&self, client: &mut TraciClient, person_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_ANGLE, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_slope(&self, client: &mut TraciClient, person_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_SLOPE, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_lane_position(&self, client: &mut TraciClient, person_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_LANEPOSITION, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_color(&self, client: &mut TraciClient, person_id: &str) -> Result<TraciColor, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_COLOR, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_COLOR))?;
        client.read_color_from_input()
    }

    pub fn get_length(&self, client: &mut TraciClient, person_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_LENGTH, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_road_id(&self, client: &mut TraciClient, person_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_ROAD_ID, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_lane_id(&self, client: &mut TraciClient, person_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_LANE_ID, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_type_id(&self, client: &mut TraciClient, person_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_TYPE, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_speed_factor(&self, client: &mut TraciClient, person_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_SPEED_FACTOR, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_waiting_time(&self, client: &mut TraciClient, person_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_WAITING_TIME, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_next_edge(&self, client: &mut TraciClient, person_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_NEXT_EDGE, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_vehicle(&self, client: &mut TraciClient, person_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_VEHICLE, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_remaining_stages(&self, client: &mut TraciClient, person_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_STAGES_REMAINING, person_id, None);
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    /// Get a specific stage of the person's journey.
    pub fn get_stage(&self, client: &mut TraciClient, person_id: &str, next_stage_index: i32) -> Result<TraciStage, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(next_stage_index);
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_STAGE, person_id, Some(&add));
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_COMPOUND))?;
        read_traci_stage(client)
    }

    /// Get the edges for a specific stage.
    pub fn get_edges(&self, client: &mut TraciClient, person_id: &str, next_stage_index: i32) -> Result<Vec<String>, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(next_stage_index);
        client.create_command(CMD_GET_PERSON_VARIABLE, VAR_EDGES, person_id, Some(&add));
        client.process_get(CMD_GET_PERSON_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    // -----------------------------------------------------------------------
    // Setters / commands
    // -----------------------------------------------------------------------

    /// Add a new person to the simulation.
    pub fn add(&self, client: &mut TraciClient, person_id: &str, edge_id: &str, pos: f64, depart: f64, type_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(4);
        add.write_u8(TYPE_STRING);
        add.write_string(type_id);
        add.write_u8(TYPE_STRING);
        add.write_string(edge_id);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(depart);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(pos);
        client.create_command(CMD_SET_PERSON_VARIABLE, ADD, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    /// Append a full stage to a person's plan.
    pub fn append_stage(&self, client: &mut TraciClient, person_id: &str, stage: &TraciStage) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(13);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(stage.type_);
        add.write_u8(TYPE_STRING);
        add.write_string(&stage.v_type);
        add.write_u8(TYPE_STRING);
        add.write_string(&stage.line);
        add.write_u8(TYPE_STRING);
        add.write_string(&stage.dest_stop);
        add.write_u8(TYPE_STRINGLIST);
        add.write_i32(stage.edges.len() as i32);
        for e in &stage.edges {
            add.write_string(e);
        }
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(stage.travel_time);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(stage.cost);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(stage.length);
        add.write_u8(TYPE_STRING);
        add.write_string(&stage.intended);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(stage.depart);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(stage.depart_pos);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(stage.arrival_pos);
        add.write_u8(TYPE_STRING);
        add.write_string(&stage.description);
        client.create_command(CMD_SET_PERSON_VARIABLE, APPEND_STAGE, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    /// Append a waiting stage.
    pub fn append_waiting_stage(&self, client: &mut TraciClient, person_id: &str, duration: f64, description: &str, stop_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(4);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(STAGE_WAITING as i32);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(duration);
        add.write_u8(TYPE_STRING);
        add.write_string(description);
        add.write_u8(TYPE_STRING);
        add.write_string(stop_id);
        client.create_command(CMD_SET_PERSON_VARIABLE, APPEND_STAGE, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    /// Append a walking stage.
    pub fn append_walking_stage(&self, client: &mut TraciClient, person_id: &str, edges: &[String], arrival_pos: f64, duration: f64, speed: f64, stop_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(6);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(STAGE_WALKING as i32);
        add.write_u8(TYPE_STRINGLIST);
        add.write_i32(edges.len() as i32);
        for e in edges {
            add.write_string(e);
        }
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(arrival_pos);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(duration);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        add.write_u8(TYPE_STRING);
        add.write_string(stop_id);
        client.create_command(CMD_SET_PERSON_VARIABLE, APPEND_STAGE, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    /// Append a driving stage.
    pub fn append_driving_stage(&self, client: &mut TraciClient, person_id: &str, to_edge: &str, lines: &str, stop_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(4);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(STAGE_DRIVING as i32);
        add.write_u8(TYPE_STRING);
        add.write_string(to_edge);
        add.write_u8(TYPE_STRING);
        add.write_string(lines);
        add.write_u8(TYPE_STRING);
        add.write_string(stop_id);
        client.create_command(CMD_SET_PERSON_VARIABLE, APPEND_STAGE, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    /// Remove one stage from a person's plan.
    pub fn remove_stage(&self, client: &mut TraciClient, person_id: &str, next_stage_index: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(next_stage_index);
        client.create_command(CMD_SET_PERSON_VARIABLE, REMOVE_STAGE, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    /// Remove all stages from a person's plan.
    pub fn remove_stages(&self, client: &mut TraciClient, person_id: &str) -> Result<(), TraciError> {
        while self.get_remaining_stages(client, person_id)? > 1 {
            self.remove_stage(client, person_id, 1)?;
        }
        self.remove_stage(client, person_id, 0)
    }

    pub fn reroute_traveltime(&self, client: &mut TraciClient, person_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(0);
        client.create_command(CMD_SET_PERSON_VARIABLE, CMD_REROUTE_TRAVELTIME, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn move_to(&self, client: &mut TraciClient, person_id: &str, edge_id: &str, position: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(edge_id);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(position);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_MOVE_TO, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn move_to_xy(&self, client: &mut TraciClient, person_id: &str, edge_id: &str, x: f64, y: f64, angle: f64, keep_route: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(5);
        add.write_u8(TYPE_STRING);
        add.write_string(edge_id);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(x);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(y);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(angle);
        add.write_u8(TYPE_BYTE);
        add.write_u8(keep_route as u8);
        client.create_command(CMD_SET_PERSON_VARIABLE, MOVE_TO_XY, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_speed(&self, client: &mut TraciClient, person_id: &str, speed: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_SPEED, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_type(&self, client: &mut TraciClient, person_id: &str, type_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(type_id);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_TYPE, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_speed_factor(&self, client: &mut TraciClient, person_id: &str, factor: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(factor);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_SPEED_FACTOR, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_length(&self, client: &mut TraciClient, person_id: &str, length: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(length);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_LENGTH, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_width(&self, client: &mut TraciClient, person_id: &str, width: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(width);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_WIDTH, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_height(&self, client: &mut TraciClient, person_id: &str, height: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(height);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_HEIGHT, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_min_gap(&self, client: &mut TraciClient, person_id: &str, min_gap: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(min_gap);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_MINGAP, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn set_color(&self, client: &mut TraciClient, person_id: &str, c: &TraciColor) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COLOR);
        add.write_u8(c.r);
        add.write_u8(c.g);
        add.write_u8(c.b);
        add.write_u8(c.a);
        client.create_command(CMD_SET_PERSON_VARIABLE, VAR_COLOR, person_id, Some(&add));
        client.process_set(CMD_SET_PERSON_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, person_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_PERSON_VARIABLE, person_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, person_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_PERSON_CONTEXT, person_id, begin, end, domain, range, vars)
    }
}
