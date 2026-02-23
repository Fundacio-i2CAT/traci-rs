// SPDX-License-Identifier: EPL-2.0
//! TraCI Junction domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults, TraciPosition},
};

/// Scope for interacting with SUMO junction objects.
#[derive(Debug, Default)]
pub struct JunctionScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl JunctionScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_JUNCTION_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_JUNCTION_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_JUNCTION_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_JUNCTION_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, junction_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_JUNCTION_VARIABLE, VAR_PARAMETER, junction_id, Some(&add));
        client.process_get(CMD_GET_JUNCTION_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, junction_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_JUNCTION_VARIABLE, VAR_PARAMETER, junction_id, Some(&add));
        client.process_set(CMD_SET_JUNCTION_VARIABLE)?;
        Ok(())
    }

    pub fn get_position(&self, client: &mut TraciClient, junction_id: &str) -> Result<TraciPosition, TraciError> {
        client.create_command(CMD_GET_JUNCTION_VARIABLE, VAR_POSITION, junction_id, None);
        client.process_get(CMD_GET_JUNCTION_VARIABLE, Some(POSITION_2D))?;
        client.read_pos_2d_from_input()
    }

    pub fn get_shape(&self, client: &mut TraciClient, junction_id: &str) -> Result<Vec<TraciPosition>, TraciError> {
        client.create_command(CMD_GET_JUNCTION_VARIABLE, VAR_SHAPE, junction_id, None);
        client.process_get(CMD_GET_JUNCTION_VARIABLE, Some(TYPE_POLYGON))?;
        client.read_polygon_from_input()
    }

    pub fn subscribe(&self, client: &mut TraciClient, junction_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_JUNCTION_VARIABLE, junction_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, junction_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_JUNCTION_CONTEXT, junction_id, begin, end, domain, range, vars)
    }
}
