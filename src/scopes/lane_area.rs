// SPDX-License-Identifier: EPL-2.0
//! TraCI LaneArea (E2) detector domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults},
};

/// Scope for interacting with SUMO lane area (E2) detectors.
///
/// No domain-specific getter/setter methods were defined in the original C++ API
/// (the detector's variables are read via subscriptions).
#[derive(Debug, Default)]
pub struct LaneAreaScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl LaneAreaScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_LANEAREA_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_LANEAREA_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_LANEAREA_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_LANEAREA_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, det_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_LANEAREA_VARIABLE, VAR_PARAMETER, det_id, Some(&add));
        client.process_get(CMD_GET_LANEAREA_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn subscribe(&self, client: &mut TraciClient, det_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_LANEAREA_VARIABLE, det_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, det_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_LANEAREA_CONTEXT, det_id, begin, end, domain, range, vars)
    }
}
