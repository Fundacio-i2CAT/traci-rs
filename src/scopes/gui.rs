// SPDX-License-Identifier: EPL-2.0
//! TraCI GUI domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{ContextSubscriptionResults, SubscriptionResults, TraciPosition},
};

/// Scope for interacting with the SUMO GUI.
#[derive(Debug, Default)]
pub struct GuiScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl GuiScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_GUI_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_GUI_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    /// Return the current zoom level of a GUI view.
    pub fn get_zoom(&self, client: &mut TraciClient, view_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_GUI_VARIABLE, VAR_VIEW_ZOOM, view_id, None);
        client.process_get(CMD_GET_GUI_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    /// Return the 2-D offset (centre) of the given GUI view.
    pub fn get_offset(&self, client: &mut TraciClient, view_id: &str) -> Result<TraciPosition, TraciError> {
        client.create_command(CMD_GET_GUI_VARIABLE, VAR_VIEW_OFFSET, view_id, None);
        client.process_get(CMD_GET_GUI_VARIABLE, Some(POSITION_2D))?;
        client.read_pos_2d_from_input()
    }

    /// Return the name of the colour scheme currently active in a view.
    pub fn get_schema(&self, client: &mut TraciClient, view_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_GUI_VARIABLE, VAR_VIEW_SCHEMA, view_id, None);
        client.process_get(CMD_GET_GUI_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    /// Return the bounding box of a GUI view as a 4-point polygon.
    pub fn get_boundary(&self, client: &mut TraciClient, view_id: &str) -> Result<Vec<TraciPosition>, TraciError> {
        client.create_command(CMD_GET_GUI_VARIABLE, VAR_VIEW_BOUNDARY, view_id, None);
        client.process_get(CMD_GET_GUI_VARIABLE, Some(TYPE_POLYGON))?;
        client.read_polygon_from_input()
    }

    // -----------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------

    pub fn set_zoom(&self, client: &mut TraciClient, view_id: &str, zoom: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(zoom);
        client.create_command(CMD_SET_GUI_VARIABLE, VAR_VIEW_ZOOM, view_id, Some(&add));
        client.process_set(CMD_SET_GUI_VARIABLE)?;
        Ok(())
    }

    pub fn set_offset(&self, client: &mut TraciClient, view_id: &str, x: f64, y: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(POSITION_2D);
        add.write_f64(x);
        add.write_f64(y);
        client.create_command(CMD_SET_GUI_VARIABLE, VAR_VIEW_OFFSET, view_id, Some(&add));
        client.process_set(CMD_SET_GUI_VARIABLE)?;
        Ok(())
    }

    pub fn set_schema(&self, client: &mut TraciClient, view_id: &str, schema_name: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(schema_name);
        client.create_command(CMD_SET_GUI_VARIABLE, VAR_VIEW_SCHEMA, view_id, Some(&add));
        client.process_set(CMD_SET_GUI_VARIABLE)?;
        Ok(())
    }

    /// Set the view boundary with (xmin, ymin, xmax, ymax).
    pub fn set_boundary(
        &self,
        client: &mut TraciClient,
        view_id: &str,
        xmin: f64,
        ymin: f64,
        xmax: f64,
        ymax: f64,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_POLYGON);
        add.write_u8(2);
        add.write_f64(xmin);
        add.write_f64(ymin);
        add.write_f64(xmax);
        add.write_f64(ymax);
        client.create_command(CMD_SET_GUI_VARIABLE, VAR_VIEW_BOUNDARY, view_id, Some(&add));
        client.process_set(CMD_SET_GUI_VARIABLE)?;
        Ok(())
    }

    /// Save a screenshot from the given view.
    pub fn screenshot(
        &self,
        client: &mut TraciClient,
        view_id: &str,
        filename: &str,
        width: i32,
        height: i32,
    ) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(3);
        add.write_u8(TYPE_STRING);
        add.write_string(filename);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(width);
        add.write_u8(TYPE_INTEGER);
        add.write_i32(height);
        client.create_command(CMD_SET_GUI_VARIABLE, VAR_SCREENSHOT, view_id, Some(&add));
        client.process_set(CMD_SET_GUI_VARIABLE)?;
        Ok(())
    }

    /// Set the view to track a vehicle.
    pub fn track_vehicle(&self, client: &mut TraciClient, view_id: &str, veh_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(veh_id);
        client.create_command(CMD_SET_GUI_VARIABLE, VAR_VIEW_SCHEMA, view_id, Some(&add));
        client.process_set(CMD_SET_GUI_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, view_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_GUI_VARIABLE, view_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, view_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_GUI_CONTEXT, view_id, begin, end, domain, range, vars)
    }
}
