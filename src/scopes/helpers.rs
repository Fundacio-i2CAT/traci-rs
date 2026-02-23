// SPDX-License-Identifier: EPL-2.0
//! Internal helper macros used by scope modules.
//!
//! These are `macro_rules!` macros (not proc-macros) that generate the
//! repetitive get*/set* wrappers every scope implements.

/// Generate the `get_id_list` and `get_id_count` methods for a scope, plus
/// the generic parameter API, and subscription helpers.
///
/// Usage inside a scope impl block:
/// ```ignore
/// impl_scope_common!(MyScope, CMD_GET_MY_VARIABLE, CMD_SET_MY_VARIABLE,
///                    CMD_SUBSCRIBE_MY_VARIABLE, CMD_SUBSCRIBE_MY_CONTEXT);
/// ```
#[macro_export]
macro_rules! scope_get_double {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::TYPE_DOUBLE))?;
        $client.read_double_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_int {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::TYPE_INTEGER))?;
        $client.read_int_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_string {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::TYPE_STRING))?;
        $client.read_string_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_string_list {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::TYPE_STRINGLIST))?;
        $client.read_string_list_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_double_list {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::TYPE_DOUBLELIST))?;
        $client.read_f64_list_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_pos {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::POSITION_2D))?;
        $client.read_pos_2d_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_pos3d {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::POSITION_3D))?;
        $client.read_pos_3d_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_color {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::TYPE_COLOR))?;
        $client.read_color_from_input()
    }};
}

#[macro_export]
macro_rules! scope_get_polygon {
    ($client:expr, $cmd_get:expr, $var:expr, $id:expr) => {{
        let mut add: Option<&$crate::storage::Storage> = None;
        $client.create_command($cmd_get, $var, $id, add.take());
        $client.process_get($cmd_get, Some($crate::constants::TYPE_POLYGON))?;
        $client.read_polygon_from_input()
    }};
}

#[macro_export]
macro_rules! scope_set_double {
    ($client:expr, $cmd_set:expr, $var:expr, $id:expr, $val:expr) => {{
        let mut add = $crate::storage::Storage::new();
        add.write_u8($crate::constants::TYPE_DOUBLE);
        add.write_f64($val);
        $client.create_command($cmd_set, $var, $id, Some(&add));
        $client.process_set($cmd_set)
    }};
}

#[macro_export]
macro_rules! scope_set_int {
    ($client:expr, $cmd_set:expr, $var:expr, $id:expr, $val:expr) => {{
        let mut add = $crate::storage::Storage::new();
        add.write_u8($crate::constants::TYPE_INTEGER);
        add.write_i32($val);
        $client.create_command($cmd_set, $var, $id, Some(&add));
        $client.process_set($cmd_set)
    }};
}

#[macro_export]
macro_rules! scope_set_string {
    ($client:expr, $cmd_set:expr, $var:expr, $id:expr, $val:expr) => {{
        let mut add = $crate::storage::Storage::new();
        add.write_u8($crate::constants::TYPE_STRING);
        add.write_string($val);
        $client.create_command($cmd_set, $var, $id, Some(&add));
        $client.process_set($cmd_set)
    }};
}

#[macro_export]
macro_rules! scope_set_string_list {
    ($client:expr, $cmd_set:expr, $var:expr, $id:expr, $val:expr) => {{
        let mut add = $crate::storage::Storage::new();
        add.write_u8($crate::constants::TYPE_STRINGLIST);
        add.write_string_list($val);
        $client.create_command($cmd_set, $var, $id, Some(&add));
        $client.process_set($cmd_set)
    }};
}

/// Generate the common subscription-result accessors for a scope struct.
#[macro_export]
macro_rules! impl_scope_accessors {
    () => {
        /// Return all variable subscription results cached after the last `simulation_step`.
        pub fn get_all_subscription_results(
            &self,
        ) -> &$crate::types::SubscriptionResults {
            &self.subscription_results
        }

        /// Return the variable subscription results for a single object.
        pub fn get_subscription_results(
            &self,
            obj_id: &str,
        ) -> Option<&$crate::types::TraciResults> {
            self.subscription_results.get(obj_id)
        }

        /// Return all context subscription results cached after the last `simulation_step`.
        pub fn get_all_context_subscription_results(
            &self,
        ) -> &$crate::types::ContextSubscriptionResults {
            &self.context_subscription_results
        }

        /// Return the context subscription results for a single object.
        pub fn get_context_subscription_results(
            &self,
            obj_id: &str,
        ) -> Option<&$crate::types::SubscriptionResults> {
            self.context_subscription_results.get(obj_id)
        }
    };
}
