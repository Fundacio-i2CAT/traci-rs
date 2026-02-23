// SPDX-License-Identifier: EPL-2.0
//! TraCI Lane domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults, TraciConnection, TraciPosition},
};

/// Scope for interacting with SUMO lane objects.
#[derive(Debug, Default)]
pub struct LaneScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl LaneScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, lane_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_PARAMETER, lane_id, Some(&add));
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, lane_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_LANE_VARIABLE, VAR_PARAMETER, lane_id, Some(&add));
        client.process_set(CMD_SET_LANE_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    pub fn get_length(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_LENGTH, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_max_speed(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_MAXSPEED, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_width(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_WIDTH, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_allowed(&self, client: &mut TraciClient, lane_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LANE_ALLOWED, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_disallowed(&self, client: &mut TraciClient, lane_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LANE_DISALLOWED, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_link_number(&self, client: &mut TraciClient, lane_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LANE_LINK_NUMBER, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    /// Return the list of connections leaving this lane.
    ///
    /// Compound message format:
    /// ```text
    ///   TYPE_COMPOUND  int(total_bytes)
    ///   TYPE_INTEGER   int(link_count)
    ///   [for each link:]
    ///     TYPE_STRING  approached_lane
    ///     TYPE_STRING  approached_internal
    ///     TYPE_UBYTE   has_priority (0/1)
    ///     TYPE_UBYTE   is_open (0/1)
    ///     TYPE_UBYTE   has_foe (0/1)
    ///     TYPE_STRING  state
    ///     TYPE_STRING  direction
    ///     TYPE_DOUBLE  length
    /// ```
    pub fn get_links(&self, client: &mut TraciClient, lane_id: &str) -> Result<Vec<TraciConnection>, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LANE_LINKS, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_COMPOUND))?;

        // skip compound header
        let _tag = client.read_ubyte_from_input()?;
        let _total = client.read_int_from_input()?;

        let link_count = client.read_int_from_input()?;
        let mut result = Vec::with_capacity(link_count as usize);
        for _ in 0..link_count {
            let _tag = client.read_ubyte_from_input()?;
            let approached_lane = client.read_string_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let approached_internal = client.read_string_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let has_priority = client.read_ubyte_from_input()? != 0;

            let _tag = client.read_ubyte_from_input()?;
            let is_open = client.read_ubyte_from_input()? != 0;

            let _tag = client.read_ubyte_from_input()?;
            let has_foe = client.read_ubyte_from_input()? != 0;

            let _tag = client.read_ubyte_from_input()?;
            let state = client.read_string_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let direction = client.read_string_from_input()?;

            let _tag = client.read_ubyte_from_input()?;
            let length = client.read_double_from_input()?;

            result.push(TraciConnection {
                approached_lane,
                has_prio: has_priority,
                is_open,
                has_foe,
                approached_internal,
                state,
                direction,
                length,
            });
        }
        Ok(result)
    }

    pub fn get_shape(&self, client: &mut TraciClient, lane_id: &str) -> Result<Vec<TraciPosition>, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_SHAPE, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_POLYGON))?;
        client.read_polygon_from_input()
    }

    pub fn get_edge_id(&self, client: &mut TraciClient, lane_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LANE_EDGE_ID, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_co2_emission(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_CO2EMISSION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_co_emission(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_COEMISSION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_hc_emission(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_HCEMISSION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_pmx_emission(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_PMXEMISSION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_nox_emission(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_NOXEMISSION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_fuel_consumption(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_FUELCONSUMPTION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_noise_emission(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_NOISEEMISSION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_electricity_consumption(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_ELECTRICITYCONSUMPTION, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_mean_speed(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LAST_STEP_MEAN_SPEED, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_occupancy(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LAST_STEP_OCCUPANCY, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_length(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LAST_STEP_LENGTH, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_traveltime(&self, client: &mut TraciClient, lane_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_CURRENT_TRAVELTIME, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_last_step_vehicle_number(&self, client: &mut TraciClient, lane_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LAST_STEP_VEHICLE_NUMBER, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_last_step_halting_number(&self, client: &mut TraciClient, lane_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LAST_STEP_VEHICLE_HALTING_NUMBER, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_last_step_vehicle_ids(&self, client: &mut TraciClient, lane_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_LANE_VARIABLE, LAST_STEP_VEHICLE_ID_LIST, lane_id, None);
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    /// Return the IDs of lanes on which vehicles must yield to traffic on this lane entering `to_lane_id`.
    pub fn get_foes(
        &self,
        client: &mut TraciClient,
        lane_id: &str,
        to_lane_id: &str,
    ) -> Result<Vec<String>, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(to_lane_id);
        client.create_command(CMD_GET_LANE_VARIABLE, VAR_FOES, lane_id, Some(&add));
        client.process_get(CMD_GET_LANE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    /// Return the internal foes (same as `get_foes` with empty to_lane_id).
    pub fn get_internal_foes(&self, client: &mut TraciClient, lane_id: &str) -> Result<Vec<String>, TraciError> {
        self.get_foes(client, lane_id, "")
    }

    // -----------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------

    pub fn set_allowed(
        &self,
        client: &mut TraciClient,
        lane_id: &str,
        allowed_classes: &[String],
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRINGLIST);
        add.write_string_list(allowed_classes);
        client.create_command(CMD_SET_LANE_VARIABLE, LANE_ALLOWED, lane_id, Some(&add));
        client.process_set(CMD_SET_LANE_VARIABLE)?;
        Ok(())
    }

    pub fn set_disallowed(
        &self,
        client: &mut TraciClient,
        lane_id: &str,
        disallowed_classes: &[String],
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRINGLIST);
        add.write_string_list(disallowed_classes);
        client.create_command(CMD_SET_LANE_VARIABLE, LANE_DISALLOWED, lane_id, Some(&add));
        client.process_set(CMD_SET_LANE_VARIABLE)?;
        Ok(())
    }

    pub fn set_max_speed(&self, client: &mut TraciClient, lane_id: &str, speed: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(speed);
        client.create_command(CMD_SET_LANE_VARIABLE, VAR_MAXSPEED, lane_id, Some(&add));
        client.process_set(CMD_SET_LANE_VARIABLE)?;
        Ok(())
    }

    pub fn set_length(&self, client: &mut TraciClient, lane_id: &str, length: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(length);
        client.create_command(CMD_SET_LANE_VARIABLE, VAR_LENGTH, lane_id, Some(&add));
        client.process_set(CMD_SET_LANE_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, lane_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_LANE_VARIABLE, lane_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, lane_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_LANE_CONTEXT, lane_id, begin, end, domain, range, vars)
    }
}
