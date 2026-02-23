// SPDX-License-Identifier: EPL-2.0
//! TraCI RouteProbe domain scope (no domain-specific methods in the C++ API).

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults},
};

/// Scope for interacting with SUMO route probe detectors.
#[derive(Debug, Default)]
pub struct RouteProbeScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl RouteProbeScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_ROUTEPROBE_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_ROUTEPROBE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_ROUTEPROBE_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_ROUTEPROBE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, obj_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_ROUTEPROBE_VARIABLE, VAR_PARAMETER, obj_id, Some(&add));
        client.process_get(CMD_GET_ROUTEPROBE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, obj_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_ROUTEPROBE_VARIABLE, VAR_PARAMETER, obj_id, Some(&add));
        client.process_set(CMD_SET_ROUTEPROBE_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, obj_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_ROUTEPROBE_VARIABLE, obj_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, obj_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_ROUTEPROBE_CONTEXT, obj_id, begin, end, domain, range, vars)
    }
}
