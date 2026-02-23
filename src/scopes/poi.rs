// SPDX-License-Identifier: EPL-2.0
//! TraCI POI domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults, TraciColor, TraciPosition},
};

/// Scope for interacting with SUMO POI (Point of Interest) objects.
#[derive(Debug, Default)]
pub struct PoiScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl PoiScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, poi_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_POI_VARIABLE, VAR_PARAMETER, poi_id, Some(&add));
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, poi_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_PARAMETER, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    pub fn get_type(&self, client: &mut TraciClient, poi_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, VAR_TYPE, poi_id, None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_position(&self, client: &mut TraciClient, poi_id: &str) -> Result<TraciPosition, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, VAR_POSITION, poi_id, None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(POSITION_2D))?;
        client.read_pos_2d_from_input()
    }

    pub fn get_color(&self, client: &mut TraciClient, poi_id: &str) -> Result<TraciColor, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, VAR_COLOR, poi_id, None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_COLOR))?;
        client.read_color_from_input()
    }

    pub fn get_width(&self, client: &mut TraciClient, poi_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, VAR_WIDTH, poi_id, None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_height(&self, client: &mut TraciClient, poi_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, VAR_HEIGHT, poi_id, None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_angle(&self, client: &mut TraciClient, poi_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, VAR_ANGLE, poi_id, None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_image_file(&self, client: &mut TraciClient, poi_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_POI_VARIABLE, VAR_IMAGEFILE, poi_id, None);
        client.process_get(CMD_GET_POI_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    // -----------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------

    pub fn set_type(&self, client: &mut TraciClient, poi_id: &str, poi_type: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(poi_type);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_TYPE, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn set_position(&self, client: &mut TraciClient, poi_id: &str, x: f64, y: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(POSITION_2D);
        add.write_f64(x);
        add.write_f64(y);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_POSITION, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn set_color(&self, client: &mut TraciClient, poi_id: &str, c: &TraciColor) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COLOR);
        add.write_u8(c.r);
        add.write_u8(c.g);
        add.write_u8(c.b);
        add.write_u8(c.a);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_COLOR, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn set_width(&self, client: &mut TraciClient, poi_id: &str, width: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(width);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_WIDTH, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn set_height(&self, client: &mut TraciClient, poi_id: &str, height: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(height);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_HEIGHT, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn set_angle(&self, client: &mut TraciClient, poi_id: &str, angle: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(angle);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_ANGLE, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn set_image_file(&self, client: &mut TraciClient, poi_id: &str, image_file: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(image_file);
        client.create_command(CMD_SET_POI_VARIABLE, VAR_IMAGEFILE, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    /// Add a new POI to the simulation.
    #[allow(clippy::too_many_arguments)]
    pub fn add(
        &self,
        client: &mut TraciClient,
        poi_id: &str,
        x: f64,
        y: f64,
        color: &TraciColor,
        poi_type: &str,
        layer: i32,
        img_file: &str,
        width: f64,
        height: f64,
        angle: f64,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(8);
        add.write_u8(TYPE_STRING);
        add.write_string(poi_type);
        add.write_u8(TYPE_COLOR);
        add.write_u8(color.r);
        add.write_u8(color.g);
        add.write_u8(color.b);
        add.write_u8(color.a);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(layer);
        add.write_u8(POSITION_2D);
        add.write_f64(x);
        add.write_f64(y);
        add.write_u8(TYPE_STRING);
        add.write_string(img_file);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(width);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(height);
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(angle);
        client.create_command(CMD_SET_POI_VARIABLE, ADD, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn remove(&self, client: &mut TraciClient, poi_id: &str, layer: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(layer);
        client.create_command(CMD_SET_POI_VARIABLE, REMOVE, poi_id, Some(&add));
        client.process_set(CMD_SET_POI_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, poi_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_POI_VARIABLE, poi_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, poi_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_POI_CONTEXT, poi_id, begin, end, domain, range, vars)
    }
}
