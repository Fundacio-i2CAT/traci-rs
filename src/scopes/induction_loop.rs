// SPDX-License-Identifier: EPL-2.0
//! TraCI Induction Loop domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults, TraciVehicleData},
};

/// Scope for interacting with SUMO inductive loop detectors.
#[derive(Debug, Default)]
pub struct InductionLoopScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl InductionLoopScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, loop_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, VAR_PARAMETER, loop_id, Some(&add));
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    /// Number of vehicles that passed the detector in the last measurement interval.
    pub fn get_interval_vehicle_number(&self, client: &mut TraciClient, loop_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, VAR_LAST_INTERVAL_NUMBER, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_position(&self, client: &mut TraciClient, loop_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, VAR_POSITION, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_lane_id(&self, client: &mut TraciClient, loop_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, VAR_LANE_ID, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_last_step_vehicle_number(&self, client: &mut TraciClient, loop_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, LAST_STEP_VEHICLE_NUMBER, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_last_step_mean_speed(&self, client: &mut TraciClient, loop_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, LAST_STEP_MEAN_SPEED, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_vehicle_ids(&self, client: &mut TraciClient, loop_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, LAST_STEP_VEHICLE_ID_LIST, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_last_step_occupancy(&self, client: &mut TraciClient, loop_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, LAST_STEP_OCCUPANCY, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_mean_length(&self, client: &mut TraciClient, loop_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, LAST_STEP_LENGTH, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_time_since_detection(&self, client: &mut TraciClient, loop_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, LAST_STEP_TIME_SINCE_DETECTION, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    /// Return per-vehicle data collected at this induction loop in the last step.
    ///
    /// The compound message layout is:
    /// ```text
    ///   TYPE_COMPOUND  int(n_items)
    ///   TYPE_INTEGER   int(n)       -- count of vehicle records
    ///   [for each vehicle:]
    ///     TYPE_STRING  id
    ///     TYPE_DOUBLE  length
    ///     TYPE_DOUBLE  entry_time
    ///     TYPE_DOUBLE  leave_time
    ///     TYPE_STRING  type_id
    /// ```
    pub fn get_vehicle_data(
        &self,
        client: &mut TraciClient,
        loop_id: &str,
    ) -> Result<Vec<TraciVehicleData>, TraciError> {
        client.create_command(CMD_GET_INDUCTIONLOOP_VARIABLE, LAST_STEP_VEHICLE_DATA, loop_id, None);
        client.process_get(CMD_GET_INDUCTIONLOOP_VARIABLE, Some(TYPE_COMPOUND))?;

        // number of compounds
        let _components = client.read_int_from_input()?;
        // inner list type tag (TYPE_INTEGER)
        let _tag = client.read_ubyte_from_input()?;
        let n = client.read_int_from_input()?;

        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let _tag = client.read_ubyte_from_input()?;
            let id = client.read_string_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let length = client.read_double_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let entry_time = client.read_double_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let leave_time = client.read_double_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let type_id = client.read_string_from_input()?;

            result.push(TraciVehicleData {
                id,
                length,
                entry_time,
                leave_time,
                type_id,
            });
        }
        Ok(result)
    }

    pub fn subscribe(&self, client: &mut TraciClient, loop_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE, loop_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, loop_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT, loop_id, begin, end, domain, range, vars)
    }
}
