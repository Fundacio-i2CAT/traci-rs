// SPDX-License-Identifier: EPL-2.0
//! TraCI VehicleType domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults, TraciColor},
};

/// Scope for interacting with SUMO vehicle type objects.
#[derive(Debug, Default)]
pub struct VehicleTypeScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl VehicleTypeScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, type_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_PARAMETER, type_id, Some(&add));
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, type_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_VEHICLETYPE_VARIABLE, VAR_PARAMETER, type_id, Some(&add));
        client.process_set(CMD_SET_VEHICLETYPE_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    pub fn get_length(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_LENGTH, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_max_speed(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_MAXSPEED, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_speed_factor(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_SPEED_FACTOR, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_speed_deviation(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_SPEED_DEVIATION, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_accel(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_ACCEL, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_decel(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_DECEL, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_emergency_decel(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_EMERGENCY_DECEL, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_apparent_decel(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_APPARENT_DECEL, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_imperfection(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_IMPERFECTION, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_tau(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_TAU, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_vehicle_class(&self, client: &mut TraciClient, type_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_VEHICLECLASS, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_emission_class(&self, client: &mut TraciClient, type_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_EMISSIONCLASS, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_shape_class(&self, client: &mut TraciClient, type_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_SHAPECLASS, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_min_gap(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_MINGAP, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_width(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_WIDTH, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_height(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_HEIGHT, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_color(&self, client: &mut TraciClient, type_id: &str) -> Result<TraciColor, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_COLOR, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_COLOR))?;
        client.read_color_from_input()
    }

    pub fn get_min_gap_lat(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_MINGAP_LAT, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_max_speed_lat(&self, client: &mut TraciClient, type_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_MAXSPEED_LAT, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_lateral_alignment(&self, client: &mut TraciClient, type_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_LATALIGNMENT, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_person_capacity(&self, client: &mut TraciClient, type_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_VEHICLETYPE_VARIABLE, VAR_PERSON_CAPACITY, type_id, None);
        client.process_get(CMD_GET_VEHICLETYPE_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    // -----------------------------------------------------------------------
    // Setters (each writes TYPE_* tag + value into compound)
    // -----------------------------------------------------------------------

    fn set_double_var(&self, client: &mut TraciClient, type_id: &str, var: u8, value: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(value);
        client.create_command(CMD_SET_VEHICLETYPE_VARIABLE, var, type_id, Some(&add));
        client.process_set(CMD_SET_VEHICLETYPE_VARIABLE)?;
        Ok(())
    }

    fn set_string_var(&self, client: &mut TraciClient, type_id: &str, var: u8, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_VEHICLETYPE_VARIABLE, var, type_id, Some(&add));
        client.process_set(CMD_SET_VEHICLETYPE_VARIABLE)?;
        Ok(())
    }

    pub fn set_length(&self, client: &mut TraciClient, type_id: &str, length: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_LENGTH, length)
    }

    pub fn set_max_speed(&self, client: &mut TraciClient, type_id: &str, speed: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_MAXSPEED, speed)
    }

    pub fn set_vehicle_class(&self, client: &mut TraciClient, type_id: &str, clazz: &str) -> Result<(), TraciError> {
        self.set_string_var(client, type_id, VAR_VEHICLECLASS, clazz)
    }

    pub fn set_speed_factor(&self, client: &mut TraciClient, type_id: &str, factor: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_SPEED_FACTOR, factor)
    }

    pub fn set_speed_deviation(&self, client: &mut TraciClient, type_id: &str, deviation: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_SPEED_DEVIATION, deviation)
    }

    pub fn set_emission_class(&self, client: &mut TraciClient, type_id: &str, clazz: &str) -> Result<(), TraciError> {
        self.set_string_var(client, type_id, VAR_EMISSIONCLASS, clazz)
    }

    pub fn set_shape_class(&self, client: &mut TraciClient, type_id: &str, shape_class: &str) -> Result<(), TraciError> {
        self.set_string_var(client, type_id, VAR_SHAPECLASS, shape_class)
    }

    pub fn set_width(&self, client: &mut TraciClient, type_id: &str, width: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_WIDTH, width)
    }

    pub fn set_height(&self, client: &mut TraciClient, type_id: &str, height: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_HEIGHT, height)
    }

    pub fn set_min_gap(&self, client: &mut TraciClient, type_id: &str, min_gap: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_MINGAP, min_gap)
    }

    pub fn set_accel(&self, client: &mut TraciClient, type_id: &str, accel: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_ACCEL, accel)
    }

    pub fn set_decel(&self, client: &mut TraciClient, type_id: &str, decel: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_DECEL, decel)
    }

    pub fn set_emergency_decel(&self, client: &mut TraciClient, type_id: &str, decel: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_EMERGENCY_DECEL, decel)
    }

    pub fn set_apparent_decel(&self, client: &mut TraciClient, type_id: &str, decel: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_APPARENT_DECEL, decel)
    }

    pub fn set_imperfection(&self, client: &mut TraciClient, type_id: &str, imperfection: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_IMPERFECTION, imperfection)
    }

    pub fn set_tau(&self, client: &mut TraciClient, type_id: &str, tau: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_TAU, tau)
    }

    pub fn set_color(&self, client: &mut TraciClient, type_id: &str, c: &TraciColor) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COLOR);
        add.write_u8(c.r);
        add.write_u8(c.g);
        add.write_u8(c.b);
        add.write_u8(c.a);
        client.create_command(CMD_SET_VEHICLETYPE_VARIABLE, VAR_COLOR, type_id, Some(&add));
        client.process_set(CMD_SET_VEHICLETYPE_VARIABLE)?;
        Ok(())
    }

    pub fn set_min_gap_lat(&self, client: &mut TraciClient, type_id: &str, min_gap_lat: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_MINGAP_LAT, min_gap_lat)
    }

    pub fn set_max_speed_lat(&self, client: &mut TraciClient, type_id: &str, speed: f64) -> Result<(), TraciError> {
        self.set_double_var(client, type_id, VAR_MAXSPEED_LAT, speed)
    }

    pub fn set_lateral_alignment(&self, client: &mut TraciClient, type_id: &str, lat_alignment: &str) -> Result<(), TraciError> {
        self.set_string_var(client, type_id, VAR_LATALIGNMENT, lat_alignment)
    }

    /// Copy a vehicle type under a new name.
    pub fn copy(&self, client: &mut TraciClient, orig_type_id: &str, new_type_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(new_type_id);
        client.create_command(CMD_SET_VEHICLETYPE_VARIABLE, COPY, orig_type_id, Some(&add));
        client.process_set(CMD_SET_VEHICLETYPE_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, type_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_VEHICLETYPE_VARIABLE, type_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, type_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_VEHICLETYPE_CONTEXT, type_id, begin, end, domain, range, vars)
    }
}
