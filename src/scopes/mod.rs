// SPDX-License-Identifier: EPL-2.0
//! Scope modules and helper macros.
//!
//! Every scope (edge, vehicle, …) owns two `HashMap`s that hold the
//! subscription and context-subscription results accumulated after the most
//! recent `simulation_step` call.  The macro `impl_scope_base!` generates
//! the boilerplate getters/setters used by every scope.

pub mod edge;
pub mod gui;
pub mod induction_loop;
pub mod junction;
pub mod lane;
pub mod lane_area;
pub mod multi_entry_exit;
pub mod person;
pub mod poi;
pub mod polygon;
pub mod rerouter;
pub mod route;
pub mod route_probe;
pub mod simulation;
pub mod traffic_light;
pub mod vehicle;
pub mod vehicle_type;
pub(crate) mod helpers;

use crate::types::{SubscriptionResults, ContextSubscriptionResults, TraciResults};

/// State shared by every scope: subscription result caches.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ScopeData {
    pub subscription_results: SubscriptionResults,
    pub context_subscription_results: ContextSubscriptionResults,
}

#[allow(dead_code)]
impl ScopeData {
    pub fn get_subscription_results(&self, obj_id: &str) -> Option<&TraciResults> {
        self.subscription_results.get(obj_id)
    }
    pub fn get_all_subscription_results(&self) -> &SubscriptionResults {
        &self.subscription_results
    }
    pub fn get_context_subscription_results(&self, obj_id: &str) -> Option<&SubscriptionResults> {
        self.context_subscription_results.get(obj_id)
    }
    pub fn get_all_context_subscription_results(&self) -> &ContextSubscriptionResults {
        &self.context_subscription_results
    }
}
