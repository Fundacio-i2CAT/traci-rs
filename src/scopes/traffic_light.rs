// SPDX-License-Identifier: EPL-2.0
//! TraCI TrafficLight domain scope.

use crate::{
    client::TraciClient,
    constants::*,
    error::TraciError,
    storage::Storage,
    types::{
        ContextSubscriptionResults, SubscriptionResults, TraciLink, TraciLogic, TraciPhase,
    },
};

/// Scope for interacting with SUMO traffic light objects.
#[derive(Debug, Default)]
pub struct TrafficLightScope {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

impl TrafficLightScope {
    crate::impl_scope_accessors!();

    pub fn get_id_list(&self, client: &mut TraciClient) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TRACI_ID_LIST, "", None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    pub fn get_id_count(&self, client: &mut TraciClient) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, ID_COUNT, "", None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_parameter(&self, client: &mut TraciClient, tls_id: &str, key: &str) -> Result<String, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        client.create_command(CMD_GET_TL_VARIABLE, VAR_PARAMETER, tls_id, Some(&add));
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn set_parameter(&self, client: &mut TraciClient, tls_id: &str, key: &str, value: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(2);
        add.write_u8(TYPE_STRING);
        add.write_string(key);
        add.write_u8(TYPE_STRING);
        add.write_string(value);
        client.create_command(CMD_SET_TL_VARIABLE, VAR_PARAMETER, tls_id, Some(&add));
        client.process_set(CMD_SET_TL_VARIABLE)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Getters
    // -----------------------------------------------------------------------

    pub fn get_red_yellow_green_state(&self, client: &mut TraciClient, tls_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_RED_YELLOW_GREEN_STATE, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_program(&self, client: &mut TraciClient, tls_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_CURRENT_PROGRAM, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_phase(&self, client: &mut TraciClient, tls_id: &str) -> Result<i32, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_CURRENT_PHASE, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_phase_name(&self, client: &mut TraciClient, tls_id: &str) -> Result<String, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, VAR_NAME, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_STRING))?;
        client.read_string_from_input()
    }

    pub fn get_phase_duration(&self, client: &mut TraciClient, tls_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_PHASE_DURATION, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_next_switch(&self, client: &mut TraciClient, tls_id: &str) -> Result<f64, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_NEXT_SWITCH, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_DOUBLE))?;
        client.read_double_from_input()
    }

    pub fn get_served_person_count(&self, client: &mut TraciClient, tls_id: &str, index: i32) -> Result<i32, TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(index);
        client.create_command(CMD_GET_TL_VARIABLE, VAR_PERSON_NUMBER, tls_id, Some(&add));
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_INTEGER))?;
        client.read_int_from_input()
    }

    pub fn get_controlled_lanes(&self, client: &mut TraciClient, tls_id: &str) -> Result<Vec<String>, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_CONTROLLED_LANES, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_STRINGLIST))?;
        client.read_string_list_from_input()
    }

    /// Returns the complete list of program logics (phase definitions) for a traffic light.
    pub fn get_all_program_logics(&self, client: &mut TraciClient, tls_id: &str) -> Result<Vec<TraciLogic>, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_COMPLETE_DEFINITION_RYG, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_COMPOUND))?;

        let logic_no = client.read_int_from_input()?;
        let mut logics = Vec::with_capacity(logic_no as usize);

        for _ in 0..logic_no {
            // header compound
            client.read_ubyte_from_input()?; // TYPE_COMPOUND
            client.read_int_from_input()?;   // element count

            client.read_ubyte_from_input()?; // TYPE_STRING
            let program_id = client.read_string_from_input()?;

            client.read_ubyte_from_input()?; // TYPE_INTEGER
            let type_ = client.read_int_from_input()?;

            client.read_ubyte_from_input()?; // TYPE_INTEGER
            let phase_index = client.read_int_from_input()?;

            client.read_ubyte_from_input()?; // TYPE_INTEGER
            let phase_number = client.read_int_from_input()?;

            let mut phases = Vec::with_capacity(phase_number as usize);
            for _ in 0..phase_number {
                client.read_ubyte_from_input()?; // TYPE_COMPOUND
                client.read_int_from_input()?;   // element count

                client.read_ubyte_from_input()?; // TYPE_DOUBLE
                let duration = client.read_double_from_input()?;

                client.read_ubyte_from_input()?; // TYPE_STRING
                let state = client.read_string_from_input()?;

                client.read_ubyte_from_input()?; // TYPE_DOUBLE
                let min_dur = client.read_double_from_input()?;

                client.read_ubyte_from_input()?; // TYPE_DOUBLE
                let max_dur = client.read_double_from_input()?;

                client.read_ubyte_from_input()?; // TYPE_COMPOUND (next indices)
                let num_next = client.read_int_from_input()?;
                let mut next = Vec::with_capacity(num_next as usize);
                for _ in 0..num_next {
                    client.read_ubyte_from_input()?; // TYPE_INTEGER
                    next.push(client.read_int_from_input()?);
                }

                client.read_ubyte_from_input()?; // TYPE_STRING
                let name = client.read_string_from_input()?;

                phases.push(TraciPhase { duration, state, min_dur, max_dur, next, name });
            }

            client.read_ubyte_from_input()?; // TYPE_COMPOUND (params)
            let param_number = client.read_int_from_input()?;
            let mut sub_parameter = std::collections::HashMap::new();
            for _ in 0..param_number {
                client.read_ubyte_from_input()?; // TYPE_STRINGLIST
                let pair = client.read_string_list_from_input()?;
                if pair.len() >= 2 {
                    sub_parameter.insert(pair[0].clone(), pair[1].clone());
                }
            }

            logics.push(TraciLogic {
                program_id,
                type_,
                current_phase_index: phase_index,
                phases,
                sub_parameter,
            });
        }

        Ok(logics)
    }

    /// Returns the controlled links for each signal group.
    pub fn get_controlled_links(&self, client: &mut TraciClient, tls_id: &str) -> Result<Vec<Vec<TraciLink>>, TraciError> {
        client.create_command(CMD_GET_TL_VARIABLE, TL_CONTROLLED_LINKS, tls_id, None);
        client.process_get(CMD_GET_TL_VARIABLE, Some(TYPE_COMPOUND))?;

        client.read_ubyte_from_input()?; // outer TYPE_COMPOUND (skip type tag)
        client.read_int_from_input()?;   // total element count

        let link_no = client.read_int_from_input()?;
        let mut result = Vec::with_capacity(link_no as usize);

        for _ in 0..link_no {
            client.read_ubyte_from_input()?; // TYPE_COMPOUND (inner)
            let no = client.read_int_from_input()?;
            let mut group = Vec::with_capacity(no as usize);
            for _ in 0..no {
                client.read_ubyte_from_input()?; // TYPE_COMPOUND
                client.read_int_from_input()?;   // 3 strings
                let from_lane = client.read_string_from_input()?;
                let to_lane = client.read_string_from_input()?;
                let via_lane = client.read_string_from_input()?;
                group.push(TraciLink { from_lane, via_lane, to_lane });
            }
            result.push(group);
        }

        Ok(result)
    }

    // -----------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------

    pub fn set_red_yellow_green_state(&self, client: &mut TraciClient, tls_id: &str, state: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(state);
        client.create_command(CMD_SET_TL_VARIABLE, TL_RED_YELLOW_GREEN_STATE, tls_id, Some(&add));
        client.process_set(CMD_SET_TL_VARIABLE)?;
        Ok(())
    }

    pub fn set_phase(&self, client: &mut TraciClient, tls_id: &str, index: i32) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_INTEGER);
        add.write_i32(index);
        client.create_command(CMD_SET_TL_VARIABLE, TL_PHASE_INDEX, tls_id, Some(&add));
        client.process_set(CMD_SET_TL_VARIABLE)?;
        Ok(())
    }

    pub fn set_phase_name(&self, client: &mut TraciClient, tls_id: &str, name: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(name);
        client.create_command(CMD_SET_TL_VARIABLE, VAR_NAME, tls_id, Some(&add));
        client.process_set(CMD_SET_TL_VARIABLE)?;
        Ok(())
    }

    pub fn set_program(&self, client: &mut TraciClient, tls_id: &str, program_id: &str) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_STRING);
        add.write_string(program_id);
        client.create_command(CMD_SET_TL_VARIABLE, TL_PROGRAM, tls_id, Some(&add));
        client.process_set(CMD_SET_TL_VARIABLE)?;
        Ok(())
    }

    pub fn set_phase_duration(&self, client: &mut TraciClient, tls_id: &str, phase_duration: f64) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_DOUBLE);
        add.write_f64(phase_duration);
        client.create_command(CMD_SET_TL_VARIABLE, TL_PHASE_DURATION, tls_id, Some(&add));
        client.process_set(CMD_SET_TL_VARIABLE)?;
        Ok(())
    }

    /// Upload a complete traffic light program logic to the server.
    pub fn set_program_logic(&self, client: &mut TraciClient, tls_id: &str, logic: &TraciLogic) -> Result<(), TraciError> {
        let mut add = Storage::new();
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(5);

        add.write_u8(TYPE_STRING);
        add.write_string(&logic.program_id);

        add.write_u8(TYPE_INTEGER);
        add.write_i32(logic.type_);

        add.write_u8(TYPE_INTEGER);
        add.write_i32(logic.current_phase_index);

        // phases compound
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(logic.phases.len() as i32);
        for p in &logic.phases {
            add.write_u8(TYPE_COMPOUND);
            add.write_i32(6);
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(p.duration);
            add.write_u8(TYPE_STRING);
            add.write_string(&p.state);
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(p.min_dur);
            add.write_u8(TYPE_DOUBLE);
            add.write_f64(p.max_dur);
            // next indices compound
            add.write_u8(TYPE_COMPOUND);
            add.write_i32(p.next.len() as i32);
            for &n in &p.next {
                add.write_u8(TYPE_INTEGER);
                add.write_i32(n);
            }
            add.write_u8(TYPE_STRING);
            add.write_string(&p.name);
        }

        // sub-parameters compound
        add.write_u8(TYPE_COMPOUND);
        add.write_i32(logic.sub_parameter.len() as i32);
        for (k, v) in &logic.sub_parameter {
            add.write_u8(TYPE_STRINGLIST);
            add.write_i32(2);
            add.write_string(k);
            add.write_string(v);
        }

        client.create_command(CMD_SET_TL_VARIABLE, TL_COMPLETE_PROGRAM_RYG, tls_id, Some(&add));
        client.process_set(CMD_SET_TL_VARIABLE)?;
        Ok(())
    }

    pub fn subscribe(&self, client: &mut TraciClient, tls_id: &str, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_variable(CMD_SUBSCRIBE_TL_VARIABLE, tls_id, begin, end, vars)
    }

    pub fn subscribe_context(&self, client: &mut TraciClient, tls_id: &str, domain: u8, range: f64, vars: &[u8], begin: f64, end: f64) -> Result<(), TraciError> {
        client.subscribe_object_context(CMD_SUBSCRIBE_TL_CONTEXT, tls_id, begin, end, domain, range, vars)
    }
}
