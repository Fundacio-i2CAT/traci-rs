// SPDX-License-Identifier: EPL-2.0
//! TraCI Multi-Entry/Exit (E3) detector domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults},
};

/// Scope for interacting with SUMO multi-entry/exit (E3) detectors.
#[derive(Debug, Default)]
pub struct MultiEntryExitScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl MultiEntryExitScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, det_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, VAR_PARAMETER, det_id, Some(&add));
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_last_step_vehicle_number(&self, client: &mut TraciClient, det_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, LAST_STEP_VEHICLE_NUMBER, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_last_step_mean_speed(&self, client: &mut TraciClient, det_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, LAST_STEP_MEAN_SPEED, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_vehicle_ids(&self, client: &mut TraciClient, det_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, LAST_STEP_VEHICLE_ID_LIST, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_last_step_halting_number(&self, client: &mut TraciClient, det_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, LAST_STEP_VEHICLE_HALTING_NUMBER, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_entry_lanes(&self, client: &mut TraciClient, det_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, VAR_LANES, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_exit_lanes(&self, client: &mut TraciClient, det_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, VAR_EXIT_LANES, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_entry_positions(&self, client: &mut TraciClient, det_id: &str) -> Result<Vec<f64>, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, VAR_POSITION, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_DOUBLELIST))?;
        client.read_f64_list_from_input()
    }

    pub fn get_exit_positions(&self, client: &mut TraciClient, det_id: &str) -> Result<Vec<f64>, TraciError> {
        client.create_command(CMD_GET_MULTIENTRYEXIT_VARIABLE, VAR_EXIT_POSITIONS, det_id, None);
        client.process_get(CMD_GET_MULTIENTRYEXIT_VARIABLE, Some(TYPE_DOUBLELIST))?;
        client.read_f64_list_from_input()
    }

    pub fn subscribe(&self, client: &mut TraciClient, det_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE, det_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, det_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_MULTIENTRYEXIT_CONTEXT, det_id, begin, end, domain, range, vars)
    }
}
