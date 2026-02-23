// SPDX-License-Identifier: EPL-2.0
//! TraCI Polygon domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults, TraciColor, TraciPosition},
};

/// Scope for interacting with SUMO polygon objects.
#[derive(Debug, Default)]
pub struct PolygonScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl PolygonScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_POLYGON_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_POLYGON_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, poly_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_POLYGON_VARIABLE, VAR_PARAMETER, poly_id, Some(&add));
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, poly_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_POLYGON_VARIABLE, VAR_PARAMETER, poly_id, Some(&add));
        client.process_set(CMD_SET_POLYGON_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    pub fn get_line_width(&self, client: &mut TraciClient, poly_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_POLYGON_VARIABLE, VAR_WIDTH, poly_id, None);
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_filled(&self, client: &mut TraciClient, poly_id: &str) -> Result<bool, TraciError> {
        client.create_command(CMD_GET_POLYGON_VARIABLE, VAR_FILL, poly_id, None);
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_INTEGER))?;
        Ok(client.read_int_from_input()? != 0)
    }

    pub fn get_type(&self, client: &mut TraciClient, poly_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_POLYGON_VARIABLE, VAR_TYPE, poly_id, None);
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_shape(&self, client: &mut TraciClient, poly_id: &str) -> Result<Vec<TraciPosition>, TraciError> {
        client.create_command(CMD_GET_POLYGON_VARIABLE, VAR_SHAPE, poly_id, None);
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_POLYGON))?;
        client.read_polygon_from_input()
    }

    pub fn get_color(&self, client: &mut TraciClient, poly_id: &str) -> Result<TraciColor, TraciError> {
        client.create_command(CMD_GET_POLYGON_VARIABLE, VAR_COLOR, poly_id, None);
        client.process_get(CMD_GET_POLYGON_VARIABLE, Some(TYPE_COLOR))?;
        client.read_color_from_input()
    }

    // -----------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------

    pub fn set_type(&self, client: &mut TraciClient, poly_id: &str, poly_type: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(poly_type);
        client.create_command(CMD_SET_POLYGON_VARIABLE, VAR_TYPE, poly_id, Some(&add));
        client.process_set(CMD_SET_POLYGON_VARIABLE)?;
        Ok(())
    }

    pub fn set_shape(&self, client: &mut TraciClient, poly_id: &str, shape: &[TraciPosition]) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_POLYGON);
        if shape.len() < 256 {
            add.write_u8(shape.len() as u8);
        } else {
            add.write_u8(0);
            add.write_i32(shape.len() as i32);
        }
        for p in shape {
            add.write_f64(p.x);
            add.write_f64(p.y);
        }
        client.create_command(CMD_SET_POLYGON_VARIABLE, VAR_SHAPE, poly_id, Some(&add));
        client.process_set(CMD_SET_POLYGON_VARIABLE)?;
        Ok(())
    }

    pub fn set_color(&self, client: &mut TraciClient, poly_id: &str, c: &TraciColor) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COLOR);
        add.write_u8(c.r);
        add.write_u8(c.g);
        add.write_u8(c.b);
        add.write_u8(c.a);
        client.create_command(CMD_SET_POLYGON_VARIABLE, VAR_COLOR, poly_id, Some(&add));
        client.process_set(CMD_SET_POLYGON_VARIABLE)?;
        Ok(())
    }

    pub fn set_line_width(&self, client: &mut TraciClient, poly_id: &str, line_width: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(line_width);
        client.create_command(CMD_SET_POLYGON_VARIABLE, VAR_WIDTH, poly_id, Some(&add));
        client.process_set(CMD_SET_POLYGON_VARIABLE)?;
        Ok(())
    }

    /// Add a polygon to the simulation.
    pub fn add(
        &self,
        client: &mut TraciClient,
        poly_id: &str,
        shape: &[TraciPosition],
        color: &TraciColor,
        fill: bool,
        poly_type: &str,
        layer: i32,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(5);
        add.write_u8(TYPE_STRING);
        add.write_string(poly_type);
        add.write_u8(TYPE_COLOR);
        add.write_u8(color.r);
        add.write_u8(color.g);
        add.write_u8(color.b);
        add.write_u8(color.a);
        add.write_u8(TYPE_UBYTE);
        add.write_u8(if fill { 1 } else { 0 });
        add.write_u8(TYPE_INTEGER);
        add.write_i32(layer);
        add.write_u8(TYPE_POLYGON);
        add.write_u8(shape.len() as u8);
        for p in shape {
            add.write_f64(p.x);
            add.write_f64(p.y);
        }
        client.create_command(CMD_SET_POLYGON_VARIABLE, ADD, poly_id, Some(&add));
        client.process_set(CMD_SET_POLYGON_VARIABLE)?;
        Ok(())
    }

    pub fn remove(&self, client: &mut TraciClient, poly_id: &str, layer: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(layer);
        client.create_command(CMD_SET_POLYGON_VARIABLE, REMOVE, poly_id, Some(&add));
        client.process_set(CMD_SET_POLYGON_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, poly_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_POLYGON_VARIABLE, poly_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, poly_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_POLYGON_CONTEXT, poly_id, begin, end, domain, range, vars)
    }
}
