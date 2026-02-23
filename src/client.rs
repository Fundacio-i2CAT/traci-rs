// SPDX-License-Identifier: EPL-2.0
//! Core `TraciClient` — connection, lifecycle, protocol helpers, and scope access.
//!
//! This is the Rust equivalent of the `TraCIAPI` class in the C++ library.
//! All scope fields (e.g. `client.vehicle`, `client.simulation`) are owned
//! directly by the struct and borrow the underlying socket via a shared
//! reference to the client itself (passed through each scope method call).

use std::collections::HashMap;

use crate::{
    constants::*,
    error::TraciError,
    socket::TraciSocket,
    storage::Storage,
    types::*,
};

// ============================================================================
// Public default view constant (mirrors #define DEFAULT_VIEW "View #0")
// ============================================================================
pub const DEFAULT_VIEW: &str = "View #0";

// ============================================================================
// TraciClient
// ============================================================================

/// A connected SUMO TraCI client.
///
/// Create one with [`TraciClient::connect`], then use the scope fields to
/// interact with the simulation:
///
/// ```no_run
/// # use sumo_traci::TraciClient;
/// let mut client = TraciClient::connect("localhost", 8813)?;
/// client.set_order(1)?;
/// loop {
///     client.simulation_step(0.0)?;
///     let ids = client.vehicle.get_id_list(&mut client)?;
///     // ...
/// }
/// # Ok::<(), sumo_traci::TraciError>(())
/// ```
pub struct TraciClient {
    socket: Option<TraciSocket>,
    // Reusable output / input staging buffers (mirrors myOutput / myInput in C++)
    output: Storage,
    input: Storage,
    // Domain map: response-subscribe command id → domain name (for dispatch)
    domains: HashMap<u8, DomainId>,

    // -----------------------------------------------------------------------
    // Public scopes — each wraps CMD_GET_*, CMD_SET_*, CMD_SUBSCRIBE_* ids
    // -----------------------------------------------------------------------
    pub edge:             crate::scopes::edge::EdgeScope,
    pub gui:              crate::scopes::gui::GuiScope,
    pub induction_loop:   crate::scopes::induction_loop::InductionLoopScope,
    pub junction:         crate::scopes::junction::JunctionScope,
    pub lane:             crate::scopes::lane::LaneScope,
    pub lane_area:        crate::scopes::lane_area::LaneAreaScope,
    pub multi_entry_exit: crate::scopes::multi_entry_exit::MultiEntryExitScope,
    pub person:           crate::scopes::person::PersonScope,
    pub poi:              crate::scopes::poi::PoiScope,
    pub polygon:          crate::scopes::polygon::PolygonScope,
    pub rerouter:         crate::scopes::rerouter::RerouterScope,
    pub route:            crate::scopes::route::RouteScope,
    pub route_probe:      crate::scopes::route_probe::RouteProbeScope,
    pub simulation:       crate::scopes::simulation::SimulationScope,
    pub traffic_lights:   crate::scopes::traffic_light::TrafficLightScope,
    pub vehicle:          crate::scopes::vehicle::VehicleScope,
    pub vehicle_type:     crate::scopes::vehicle_type::VehicleTypeScope,
}

/// Internal identifier used to route subscription responses back to the right scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainId {
    Edge, Gui, InductionLoop, Junction, Lane, LaneArea,
    MultiEntryExit, Person, Poi, Polygon, Rerouter, Route,
    RouteProbe, Simulation, TrafficLight, Vehicle, VehicleType,
}

impl TraciClient {
    // -----------------------------------------------------------------------
    // Connection
    // -----------------------------------------------------------------------

    /// Connect to a SUMO server and return a fully initialised client.
    ///
    /// Equivalent to `TraCIAPI::connect` + the constructor initialiser list.
    pub fn connect(host: &str, port: u16) -> Result<Self, TraciError> {
        let socket = TraciSocket::connect(host, port)?;
        let mut domains = HashMap::new();
        domains.insert(RESPONSE_SUBSCRIBE_EDGE_VARIABLE,           DomainId::Edge);
        domains.insert(RESPONSE_SUBSCRIBE_GUI_VARIABLE,            DomainId::Gui);
        domains.insert(RESPONSE_SUBSCRIBE_INDUCTIONLOOP_VARIABLE,  DomainId::InductionLoop);
        domains.insert(RESPONSE_SUBSCRIBE_JUNCTION_VARIABLE,       DomainId::Junction);
        domains.insert(RESPONSE_SUBSCRIBE_LANE_VARIABLE,           DomainId::Lane);
        domains.insert(RESPONSE_SUBSCRIBE_LANEAREA_VARIABLE,       DomainId::LaneArea);
        domains.insert(RESPONSE_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE, DomainId::MultiEntryExit);
        domains.insert(RESPONSE_SUBSCRIBE_PERSON_VARIABLE,         DomainId::Person);
        domains.insert(RESPONSE_SUBSCRIBE_POI_VARIABLE,            DomainId::Poi);
        domains.insert(RESPONSE_SUBSCRIBE_POLYGON_VARIABLE,        DomainId::Polygon);
        domains.insert(RESPONSE_SUBSCRIBE_REROUTER_VARIABLE,       DomainId::Rerouter);
        domains.insert(RESPONSE_SUBSCRIBE_ROUTE_VARIABLE,          DomainId::Route);
        domains.insert(RESPONSE_SUBSCRIBE_ROUTEPROBE_VARIABLE,     DomainId::RouteProbe);
        domains.insert(RESPONSE_SUBSCRIBE_SIM_VARIABLE,            DomainId::Simulation);
        domains.insert(RESPONSE_SUBSCRIBE_TL_VARIABLE,             DomainId::TrafficLight);
        domains.insert(RESPONSE_SUBSCRIBE_VEHICLE_VARIABLE,        DomainId::Vehicle);
        domains.insert(RESPONSE_SUBSCRIBE_VEHICLETYPE_VARIABLE,    DomainId::VehicleType);

        Ok(Self {
            socket: Some(socket),
            output: Storage::new(),
            input: Storage::new(),
            domains,
            edge:             crate::scopes::edge::EdgeScope::default(),
            gui:              crate::scopes::gui::GuiScope::default(),
            induction_loop:   crate::scopes::induction_loop::InductionLoopScope::default(),
            junction:         crate::scopes::junction::JunctionScope::default(),
            lane:             crate::scopes::lane::LaneScope::default(),
            lane_area:        crate::scopes::lane_area::LaneAreaScope::default(),
            multi_entry_exit: crate::scopes::multi_entry_exit::MultiEntryExitScope::default(),
            person:           crate::scopes::person::PersonScope::default(),
            poi:              crate::scopes::poi::PoiScope::default(),
            polygon:          crate::scopes::polygon::PolygonScope::default(),
            rerouter:         crate::scopes::rerouter::RerouterScope::default(),
            route:            crate::scopes::route::RouteScope::default(),
            route_probe:      crate::scopes::route_probe::RouteProbeScope::default(),
            simulation:       crate::scopes::simulation::SimulationScope::default(),
            traffic_lights:   crate::scopes::traffic_light::TrafficLightScope::default(),
            vehicle:          crate::scopes::vehicle::VehicleScope::default(),
            vehicle_type:     crate::scopes::vehicle_type::VehicleTypeScope::default(),
        })
    }

    // -----------------------------------------------------------------------
    // Top-level API
    // -----------------------------------------------------------------------

    /// Set the client execution order (priority among co-simulating clients).
    pub fn set_order(&mut self, order: i32) -> Result<(), TraciError> {
        let mut msg = Storage::new();
        msg.write_u8(1 + 1 + 4);
        msg.write_u8(CMD_SETORDER);
        msg.write_i32(order);
        self.socket_mut()?.send_exact(&msg)?;
        let mut in_msg = self.socket_mut()?.receive_exact()?;
        Self::check_result_state_static(&mut in_msg, CMD_SETORDER, false, None)?;
        Ok(())
    }

    /// Advance the simulation by one step (or up to `time` if > 0).
    ///
    /// After every step all stale subscription results are cleared and the new
    /// ones received from the server are parsed into the scope caches.
    pub fn simulation_step(&mut self, time: f64) -> Result<(), TraciError> {
        self.send_simulation_step(time)?;
        let mut in_msg = self.socket_mut()?.receive_exact()?;
        Self::check_result_state_static(&mut in_msg, CMD_SIMSTEP, false, None)?;

        // Clear stale subscription results
        self.edge.subscription_results.clear();
        self.gui.subscription_results.clear();
        self.induction_loop.subscription_results.clear();
        self.junction.subscription_results.clear();
        self.lane.subscription_results.clear();
        self.lane_area.subscription_results.clear();
        self.multi_entry_exit.subscription_results.clear();
        self.person.subscription_results.clear();
        self.poi.subscription_results.clear();
        self.polygon.subscription_results.clear();
        self.rerouter.subscription_results.clear();
        self.route.subscription_results.clear();
        self.route_probe.subscription_results.clear();
        self.simulation.subscription_results.clear();
        self.traffic_lights.subscription_results.clear();
        self.vehicle.subscription_results.clear();
        self.vehicle_type.subscription_results.clear();

        let num_subs = in_msg.read_i32()?;
        for _ in 0..num_subs {
            let cmd_id = Self::check_command_get_result_static(&mut in_msg, 0, None, true)?;
            if (RESPONSE_SUBSCRIBE_INDUCTIONLOOP_VARIABLE..=RESPONSE_SUBSCRIBE_PERSON_VARIABLE)
                .contains(&cmd_id)
            {
                self.read_variable_subscription(cmd_id, &mut in_msg)?;
            } else {
                self.read_context_subscription(cmd_id.wrapping_add(0x50), &mut in_msg)?;
            }
        }
        Ok(())
    }

    /// Tell SUMO to load a new simulation with the given command-line arguments.
    pub fn load(&mut self, args: &[String]) -> Result<(), TraciError> {
        let num_chars: usize = args.iter().map(|s| s.len()).sum();
        let mut content = Storage::new();
        content.write_u8(0);
        // payload length = cmd(1) + type_ubyte(1) + string_list_header(4) + per-string(4+len)
        let total = 1 + 1 + 1 + 4 + num_chars + 4 * args.len();
        content.write_i32((total + 4) as i32);
        content.write_u8(CMD_LOAD);
        content.write_u8(TYPE_STRINGLIST);
        content.write_string_list(&args.iter().cloned().collect::<Vec<_>>());
        self.socket_mut()?.send_exact(&content)?;
        let mut in_msg = self.socket_mut()?.receive_exact()?;
        Self::check_result_state_static(&mut in_msg, CMD_LOAD, false, None)?;
        Ok(())
    }

    /// Return the (TraCI version number, SUMO version string) pair.
    pub fn get_version(&mut self) -> Result<(i32, String), TraciError> {
        let mut content = Storage::new();
        content.write_u8(2);
        content.write_u8(CMD_GETVERSION);
        self.socket_mut()?.send_exact(&content)?;
        let mut in_msg = self.socket_mut()?.receive_exact()?;
        Self::check_result_state_static(&mut in_msg, CMD_GETVERSION, false, None)?;
        in_msg.read_u8()?; // msg length
        in_msg.read_u8()?; // CMD_GETVERSION echo
        let version = in_msg.read_i32()?;
        let sumo_version = in_msg.read_string()?;
        Ok((version, sumo_version))
    }

    /// Send the close command and shut down the socket.
    pub fn close(&mut self) -> Result<(), TraciError> {
        self.send_close()?;
        let mut in_msg = self.socket_mut()?.receive_exact()?;
        Self::check_result_state_static(&mut in_msg, CMD_CLOSE, false, None)?;
        self.close_socket();
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Subscription access
    // -----------------------------------------------------------------------

    /// Subscribe an object to receive variable updates every simulation step.
    pub fn subscribe_object_variable(
        &mut self,
        dom_id: u8,
        obj_id: &str,
        begin_time: f64,
        end_time: f64,
        vars: &[u8],
    ) -> Result<(), TraciError> {
        let mut msg = Storage::new();
        let var_no = vars.len();
        msg.write_u8(0);
        msg.write_i32((5 + 1 + 8 + 8 + 4 + obj_id.len() + 1 + var_no) as i32);
        msg.write_u8(dom_id);
        msg.write_f64(begin_time);
        msg.write_f64(end_time);
        msg.write_string(obj_id);
        msg.write_u8(var_no as u8);
        for &v in vars {
            msg.write_u8(v);
        }
        self.socket_mut()?.send_exact(&msg)?;
        Ok(())
    }

    /// Subscribe a context (range around object) to receive variable updates.
    pub fn subscribe_object_context(
        &mut self,
        dom_id: u8,
        obj_id: &str,
        begin_time: f64,
        end_time: f64,
        domain: u8,
        range: f64,
        vars: &[u8],
    ) -> Result<(), TraciError> {
        let mut msg = Storage::new();
        let var_no = vars.len();
        msg.write_u8(0);
        msg.write_i32((5 + 1 + 8 + 8 + 4 + obj_id.len() + 1 + 8 + 1 + var_no) as i32);
        msg.write_u8(dom_id);
        msg.write_f64(begin_time);
        msg.write_f64(end_time);
        msg.write_string(obj_id);
        msg.write_u8(domain);
        msg.write_f64(range);
        msg.write_u8(var_no as u8);
        for &v in vars {
            msg.write_u8(v);
        }
        self.socket_mut()?.send_exact(&msg)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Internal protocol helpers (pub(crate) so scopes can use them)
    // -----------------------------------------------------------------------

    /// Build a GET or SET command in `self.output`.
    pub(crate) fn create_command(
        &mut self,
        cmd_id: u8,
        var_id: u8,
        obj_id: &str,
        add: Option<&Storage>,
    ) {
        self.output.reset();
        let extra = add.map_or(0, |s| s.len());
        let length = 1 + 1 + 1 + 4 + obj_id.len() + extra;
        if length <= 255 {
            self.output.write_u8(length as u8);
        } else {
            self.output.write_u8(0);
            self.output.write_i32((length + 4) as i32);
        }
        self.output.write_u8(cmd_id);
        self.output.write_u8(var_id);
        self.output.write_string(obj_id);
        if let Some(s) = add {
            self.output.write_packet(s.as_bytes());
        }
    }

    /// Build a subscription-filter command in `self.output`.
    pub(crate) fn create_filter_command(&mut self, cmd_id: u8, var_id: u8, add: Option<&Storage>) {
        self.output.reset();
        let extra = add.map_or(0, |s| s.len());
        let length = 1 + 1 + 1 + extra;
        if length <= 255 {
            self.output.write_u8(length as u8);
        } else {
            self.output.write_u8(0);
            self.output.write_i32((length + 4) as i32);
        }
        self.output.write_u8(cmd_id);
        self.output.write_u8(var_id);
        if let Some(s) = add {
            self.output.write_packet(s.as_bytes());
        }
    }

    /// Send the command built in `self.output`, receive the response, and validate it.
    /// Returns `true` on success, `false` if no socket is connected.
    pub(crate) fn process_get(
        &mut self,
        command: u8,
        expected_type: Option<u8>,
    ) -> Result<bool, TraciError> {
        let out_bytes = self.output.as_bytes().to_vec();
        let out = Storage::from_bytes(out_bytes);
        if let Some(sock) = self.socket.as_mut() {
            sock.send_exact(&out)?;
            self.input = sock.receive_exact()?;
            Self::check_result_state_static(&mut self.input, command, false, None)?;
            Self::check_command_get_result_static(&mut self.input, command, expected_type, false)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Send the SET command built in `self.output` and validate the response.
    pub(crate) fn process_set(&mut self, command: u8) -> Result<bool, TraciError> {
        let out_bytes = self.output.as_bytes().to_vec();
        let out = Storage::from_bytes(out_bytes);
        if let Some(sock) = self.socket.as_mut() {
            sock.send_exact(&out)?;
            self.input = sock.receive_exact()?;
            Self::check_result_state_static(&mut self.input, command, false, None)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Validate a result-state response message (RTYPE_OK / RTYPE_ERR / RTYPE_NOTIMPLEMENTED).
    pub(crate) fn check_result_state_static(
        in_msg: &mut Storage,
        command: u8,
        ignore_command_id: bool,
        acknowledgement: Option<&mut String>,
    ) -> Result<(), TraciError> {
        let cmd_len  = in_msg.read_u8()? as usize;
        let cmd_id   = in_msg.read_u8()?;
        if !ignore_command_id && cmd_id != command {
            return Err(TraciError::Protocol(format!(
                "Received status for command 0x{cmd_id:02x} but expected 0x{command:02x}"
            )));
        }
        let result_type = in_msg.read_u8()?;
        let msg = in_msg.read_string()?;
        // cmd_len includes the length byte itself, so consumed = 1 (len) + 1 (id) + 1 (type) + 4+msg.len()
        let _ = cmd_len; // length already validated by Storage bounds checking
        match result_type {
            RTYPE_OK => {
                if let Some(ack) = acknowledgement {
                    *ack = format!("Command 0x{command:02x} acknowledged: {msg}");
                }
                Ok(())
            }
            RTYPE_NOTIMPLEMENTED => Err(TraciError::NotImplemented(format!(
                "Command 0x{command:02x} not implemented: {msg}"
            ))),
            RTYPE_ERR => Err(TraciError::SimulationError(format!(
                "Command 0x{command:02x} failed: {msg}"
            ))),
            other => Err(TraciError::Protocol(format!(
                "Unknown result type 0x{other:02x} for command 0x{command:02x}: {msg}"
            ))),
        }
    }

    /// Validate and advance past a GET-variable response header.
    /// Returns the actual response command id.
    pub(crate) fn check_command_get_result_static(
        in_msg: &mut Storage,
        command: u8,
        expected_type: Option<u8>,
        ignore_command_id: bool,
    ) -> Result<u8, TraciError> {
        let length = in_msg.read_u8()?;
        let length = if length == 0 {
            in_msg.read_i32()? as usize
        } else {
            length as usize
        };
        let _ = length;
        let cmd_id = in_msg.read_u8()?;
        if !ignore_command_id && cmd_id != command.wrapping_add(0x10) {
            return Err(TraciError::Protocol(format!(
                "Received response for command 0x{cmd_id:02x} but expected 0x{:02x}",
                command.wrapping_add(0x10)
            )));
        }
        if let Some(exp_type) = expected_type {
            in_msg.read_u8()?; // variable id
            in_msg.read_string()?; // object id
            let value_type = in_msg.read_u8()?;
            if value_type != exp_type {
                return Err(TraciError::Protocol(format!(
                    "Expected type 0x{exp_type:02x} but got 0x{value_type:02x}"
                )));
            }
        }
        Ok(cmd_id)
    }

    // -----------------------------------------------------------------------
    // Reading typed values out of `self.input` (used by scope helper methods)
    // -----------------------------------------------------------------------

    pub(crate) fn read_double_from_input(&mut self) -> Result<f64, TraciError> {
        self.input.read_f64()
    }
    pub(crate) fn read_int_from_input(&mut self) -> Result<i32, TraciError> {
        self.input.read_i32()
    }
    pub(crate) fn read_ubyte_from_input(&mut self) -> Result<u8, TraciError> {
        self.input.read_u8()
    }
    pub(crate) fn read_string_from_input(&mut self) -> Result<String, TraciError> {
        self.input.read_string()
    }
    pub(crate) fn read_string_list_from_input(&mut self) -> Result<Vec<String>, TraciError> {
        self.input.read_string_list()
    }
    pub(crate) fn read_f64_list_from_input(&mut self) -> Result<Vec<f64>, TraciError> {
        self.input.read_f64_list()
    }

    /// Read a 2-D or 3-D position from `self.input` (POSITION_2D tag already consumed).
    pub(crate) fn read_pos_2d_from_input(&mut self) -> Result<TraciPosition, TraciError> {
        let x = self.input.read_f64()?;
        let y = self.input.read_f64()?;
        Ok(TraciPosition::new_2d(x, y))
    }

    pub(crate) fn read_pos_3d_from_input(&mut self) -> Result<TraciPosition, TraciError> {
        let x = self.input.read_f64()?;
        let y = self.input.read_f64()?;
        let z = self.input.read_f64()?;
        Ok(TraciPosition::new_3d(x, y, z))
    }

    /// Read a polygon (list of 2-D positions) from `self.input`.
    /// The TYPE_POLYGON tag has already been consumed; the next byte is the vertex count.
    pub(crate) fn read_polygon_from_input(&mut self) -> Result<Vec<TraciPosition>, TraciError> {
        let n = self.input.read_u8()? as usize;
        let mut pts = Vec::with_capacity(n);
        for _ in 0..n {
            let x = self.input.read_f64()?;
            let y = self.input.read_f64()?;
            pts.push(TraciPosition::new_2d(x, y));
        }
        Ok(pts)
    }

    /// Read a colour from `self.input` (TYPE_COLOR tag already consumed).
    pub(crate) fn read_color_from_input(&mut self) -> Result<TraciColor, TraciError> {
        let r = self.input.read_u8()?;
        let g = self.input.read_u8()?;
        let b = self.input.read_u8()?;
        let a = self.input.read_u8()?;
        Ok(TraciColor::new(r, g, b, a))
    }

    // -----------------------------------------------------------------------
    // Subscription parsing
    // -----------------------------------------------------------------------

    fn read_variable_subscription(
        &mut self,
        cmd_id: u8,
        in_msg: &mut Storage,
    ) -> Result<(), TraciError> {
        let object_id = in_msg.read_string()?;
        let var_count = in_msg.read_u8()? as usize;
        let results = Self::read_variables_static(in_msg, var_count)?;

        // Route to the correct scope cache
        use DomainId::*;
        if let Some(domain) = self.domains.get(&cmd_id).copied() {
            let cache: &mut SubscriptionResults = match domain {
                Edge           => &mut self.edge.subscription_results,
                Gui            => &mut self.gui.subscription_results,
                InductionLoop  => &mut self.induction_loop.subscription_results,
                Junction       => &mut self.junction.subscription_results,
                Lane           => &mut self.lane.subscription_results,
                LaneArea       => &mut self.lane_area.subscription_results,
                MultiEntryExit => &mut self.multi_entry_exit.subscription_results,
                Person         => &mut self.person.subscription_results,
                Poi            => &mut self.poi.subscription_results,
                Polygon        => &mut self.polygon.subscription_results,
                Rerouter       => &mut self.rerouter.subscription_results,
                Route          => &mut self.route.subscription_results,
                RouteProbe     => &mut self.route_probe.subscription_results,
                Simulation     => &mut self.simulation.subscription_results,
                TrafficLight   => &mut self.traffic_lights.subscription_results,
                Vehicle        => &mut self.vehicle.subscription_results,
                VehicleType    => &mut self.vehicle_type.subscription_results,
            };
            cache.insert(object_id, results);
        }
        Ok(())
    }

    fn read_context_subscription(
        &mut self,
        cmd_id: u8,
        in_msg: &mut Storage,
    ) -> Result<(), TraciError> {
        let context_id = in_msg.read_string()?;
        in_msg.read_u8()?; // context domain
        let var_count  = in_msg.read_u8()? as usize;
        let num_objects = in_msg.read_i32()?;
        let mut ctx_results: SubscriptionResults = HashMap::new();
        for _ in 0..num_objects {
            let object_id = in_msg.read_string()?;
            let results   = Self::read_variables_static(in_msg, var_count)?;
            ctx_results.insert(object_id, results);
        }

        use DomainId::*;
        if let Some(domain) = self.domains.get(&cmd_id).copied() {
            let cache: &mut ContextSubscriptionResults = match domain {
                Edge           => &mut self.edge.context_subscription_results,
                Gui            => &mut self.gui.context_subscription_results,
                InductionLoop  => &mut self.induction_loop.context_subscription_results,
                Junction       => &mut self.junction.context_subscription_results,
                Lane           => &mut self.lane.context_subscription_results,
                LaneArea       => &mut self.lane_area.context_subscription_results,
                MultiEntryExit => &mut self.multi_entry_exit.context_subscription_results,
                Person         => &mut self.person.context_subscription_results,
                Poi            => &mut self.poi.context_subscription_results,
                Polygon        => &mut self.polygon.context_subscription_results,
                Rerouter       => &mut self.rerouter.context_subscription_results,
                Route          => &mut self.route.context_subscription_results,
                RouteProbe     => &mut self.route_probe.context_subscription_results,
                Simulation     => &mut self.simulation.context_subscription_results,
                TrafficLight   => &mut self.traffic_lights.context_subscription_results,
                Vehicle        => &mut self.vehicle.context_subscription_results,
                VehicleType    => &mut self.vehicle_type.context_subscription_results,
            };
            cache.insert(context_id, ctx_results);
        }
        Ok(())
    }

    /// Parse `var_count` typed variable responses from `in_msg`.
    /// Mirrors `TraCIAPI::readVariables` in the C++ implementation.
    fn read_variables_static(
        in_msg: &mut Storage,
        var_count: usize,
    ) -> Result<TraciResults, TraciError> {
        let mut results = TraciResults::new();
        for _ in 0..var_count {
            let var_id  = in_msg.read_u8()?;
            let status  = in_msg.read_u8()?;
            let type_id = in_msg.read_u8()?;
            if status != RTYPE_OK {
                return Err(TraciError::Protocol(format!(
                    "Subscription variable 0x{var_id:02x} returned status 0x{status:02x}"
                )));
            }
            let value = Self::read_typed_value(in_msg, type_id)?;
            results.insert(var_id, value);
        }
        Ok(results)
    }

    /// Decode a single typed value (type tag already consumed).
    pub(crate) fn read_typed_value(
        in_msg: &mut Storage,
        type_id: u8,
    ) -> Result<TraciValue, TraciError> {
        match type_id {
            TYPE_DOUBLE => Ok(TraciValue::Double(in_msg.read_f64()?)),
            TYPE_INTEGER => Ok(TraciValue::Int(in_msg.read_i32()?)),
            TYPE_STRING  => Ok(TraciValue::String(in_msg.read_string()?)),
            TYPE_STRINGLIST => Ok(TraciValue::StringList(in_msg.read_string_list()?)),
            TYPE_DOUBLELIST => Ok(TraciValue::DoubleList(in_msg.read_f64_list()?)),
            TYPE_COLOR => {
                let r = in_msg.read_u8()?;
                let g = in_msg.read_u8()?;
                let b = in_msg.read_u8()?;
                let a = in_msg.read_u8()?;
                Ok(TraciValue::Color(TraciColor::new(r, g, b, a)))
            }
            POSITION_2D => {
                let x = in_msg.read_f64()?;
                let y = in_msg.read_f64()?;
                Ok(TraciValue::Pos2D { x, y })
            }
            POSITION_3D => {
                let x = in_msg.read_f64()?;
                let y = in_msg.read_f64()?;
                let z = in_msg.read_f64()?;
                Ok(TraciValue::Pos3D { x, y, z })
            }
            TYPE_UBYTE => {
                // Read as Int for uniformity (matches TraCIInt in C++ with traciType=TYPE_UBYTE)
                Ok(TraciValue::Int(in_msg.read_u8()? as i32))
            }
            other => {
                // Forward-compatible: capture remaining bytes if we don't know the type.
                // For now store the raw single byte tag and an empty payload.
                Ok(TraciValue::Unknown { type_id: other, raw: Vec::new() })
            }
        }
    }

    // -----------------------------------------------------------------------
    // Low-level send helpers
    // -----------------------------------------------------------------------

    fn send_simulation_step(&mut self, time: f64) -> Result<(), TraciError> {
        let mut msg = Storage::new();
        msg.write_u8(1 + 1 + 8);
        msg.write_u8(CMD_SIMSTEP);
        msg.write_f64(time);
        self.socket_mut()?.send_exact(&msg)
    }

    fn send_close(&mut self) -> Result<(), TraciError> {
        let mut msg = Storage::new();
        msg.write_u8(1 + 1);
        msg.write_u8(CMD_CLOSE);
        self.socket_mut()?.send_exact(&msg)
    }

    fn close_socket(&mut self) {
        if let Some(mut sock) = self.socket.take() {
            let _ = sock.close();
        }
    }

    fn socket_mut(&mut self) -> Result<&mut TraciSocket, TraciError> {
        self.socket.as_mut().ok_or_else(|| {
            TraciError::Connection(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "TraCI socket is not connected",
            ))
        })
    }
}

impl Drop for TraciClient {
    fn drop(&mut self) {
        self.close_socket();
    }
}
