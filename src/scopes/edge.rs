// SPDX-License-Identifier: EPL-2.0
//! TraCI Edge domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults},
};
use std::f64;

/// Scope for interacting with SUMO edge objects.
#[derive(Debug, Default)]
pub struct EdgeScope {
    /// Variable subscription results cached after each `simulation_step`.
    pub subscription_results: SubscriptionResults,
    /// Context subscription results cached after each `simulation_step`.
    pub context_subscription_results: ContextSubscriptionResults,
}

impl EdgeScope {
    // -----------------------------------------------------------------------
    // Subscription accessors
    // -----------------------------------------------------------------------
    crate::impl_scope_accessors!();

    // -----------------------------------------------------------------------
    // Generic parameter API
    // -----------------------------------------------------------------------

    /// Retrieve a generic key/value parameter for an edge.
    pub fn get_parameter(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        key: &str,
    ) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_PARAMETER, edge_id, Some(&add));
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    /// Set a generic key/value parameter for an edge.
    pub fn set_parameter(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_EDGE_VARIABLE, VAR_PARAMETER, edge_id, Some(&add));
        client.process_set(CMD_SET_EDGE_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // ID queries
    // -----------------------------------------------------------------------

    /// Return the list of all edge IDs in the network.
    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    /// Return the total number of edges in the network.
    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    /// Return the adapted travel time of an edge at a given simulation time.
    pub fn get_adapted_traveltime(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        time: f64,
    ) -> Result<f64, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(time);
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_EDGE_TRAVELTIME, edge_id, Some(&add));
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    /// Return the effort value of an edge at a given simulation time.
    pub fn get_effort(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        time: f64,
    ) -> Result<f64, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(time);
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_EDGE_EFFORT, edge_id, Some(&add));
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_co2_emission(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_CO2EMISSION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_co_emission(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_COEMISSION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_hc_emission(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_HCEMISSION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_pmx_emission(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_PMXEMISSION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_nox_emission(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_NOXEMISSION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_fuel_consumption(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_FUELCONSUMPTION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_noise_emission(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_NOISEEMISSION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_electricity_consumption(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_ELECTRICITYCONSUMPTION, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_mean_speed(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, LAST_STEP_MEAN_SPEED, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_occupancy(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, LAST_STEP_OCCUPANCY, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_length(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, LAST_STEP_LENGTH, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_traveltime(&self, client: &mut TraciClient, edge_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_CURRENT_TRAVELTIME, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_vehicle_number(&self, client: &mut TraciClient, edge_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, LAST_STEP_VEHICLE_NUMBER, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_last_step_halting_number(&self, client: &mut TraciClient, edge_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, LAST_STEP_VEHICLE_HALTING_NUMBER, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_last_step_vehicle_ids(&self, client: &mut TraciClient, edge_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, LAST_STEP_VEHICLE_ID_LIST, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_lane_number(&self, client: &mut TraciClient, edge_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_LANE_INDEX, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_street_name(&self, client: &mut TraciClient, edge_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_EDGE_VARIABLE, VAR_NAME, edge_id, None);
        client.process_get(CMD_GET_EDGE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    // -----------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------

    /// Set an adapted travel time for an edge, optionally bounded by [begin_seconds, end_seconds).
    ///
    /// Pass `end_seconds = f64::MAX` (or `f64::INFINITY`) to set a permanent value.
    pub fn adapt_traveltime(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        time: f64,
        begin_seconds: f64,
        end_seconds: f64,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        if end_seconds < f64::MAX {
            add.write_i32(3);
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(begin_seconds);
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(end_seconds);
        } else {
            add.write_i32(1);
        }
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(time);
        client.create_command(CMD_SET_EDGE_VARIABLE, VAR_EDGE_TRAVELTIME, edge_id, Some(&add));
        client.process_set(CMD_SET_EDGE_VARIABLE)?;
        Ok(())
    }

    /// Set an effort value for an edge, optionally bounded by [begin_seconds, end_seconds).
    pub fn set_effort(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        effort: f64,
        begin_seconds: f64,
        end_seconds: f64,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        if end_seconds < f64::MAX {
            add.write_i32(3);
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(begin_seconds);
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(end_seconds);
        } else {
            add.write_i32(1);
        }
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(effort);
        client.create_command(CMD_SET_EDGE_VARIABLE, VAR_EDGE_EFFORT, edge_id, Some(&add));
        client.process_set(CMD_SET_EDGE_VARIABLE)?;
        Ok(())
    }

    /// Set the maximum speed for all lanes on an edge (m/s).
    pub fn set_max_speed(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        speed: f64,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        client.create_command(CMD_SET_EDGE_VARIABLE, VAR_MAXSPEED, edge_id, Some(&add));
        client.process_set(CMD_SET_EDGE_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Subscriptions
    // -----------------------------------------------------------------------

    /// Subscribe to a set of variables for the given edge.
    pub fn subscribe(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        vars: &[u8],
        begin: f64,
        end: f64,
    ) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_EDGE_VARIABLE, edge_id, begin, end, vars)
    }

    /// Subscribe to a context around the given edge.
    pub fn subscribe_context(
        &self,
        client: &mut TraciClient,
        edge_id: &str,
        domain: u8,
        range: f64,
        vars: &[u8],
        begin: f64,
        end: f64,
    ) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_EDGE_CONTEXT, edge_id, begin, end, domain, range, vars)
    }
}
