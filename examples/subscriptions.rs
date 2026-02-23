// SPDX-License-Identifier: EPL-2.0
// Kinematic-subscription example using traci-rs.
//
// Demonstrates the v0.2 subscription API: subscribe once per vehicle and let
// SUMO push position/speed/acceleration/angle back in every simulation_step
// response — zero extra round-trips per step regardless of fleet size.
//
// Before running this example, start SUMO with a TraCI server:
//
//   sumo --net-file net.xml \
//        --route-files routes.rou.xml \
//        --remote-port 8813 \
//        --num-clients 1
//
// Then run:
//   cargo run --example subscriptions

use std::collections::HashSet;
use traci_rs::TraciClient;

const NUM_VEHICLES: usize = 5;

fn main() -> Result<(), traci_rs::TraciError> {
    let mut client = TraciClient::connect("127.0.0.1", 8813)?;
    println!("Connected to SUMO TraCI server.");

    client.set_order(1)?;

    let (api_version, sumo_version) = client.get_version()?;
    println!("TraCI API version : {api_version}");
    println!("SUMO version      : {sumo_version}");

    // Add a route, then insert 5 vehicles on it.
    let routes = client.route_get_id_list()?;
    let route_id = if let Some(r) = routes.first() {
        r.clone()
    } else {
        let edges = client.edge_get_id_list()?;
        let first_edge = edges.into_iter()
            .find(|e| !e.starts_with(':'))
            .expect("no drivable edge found in network");
        client.route_add("route_traci", &[&first_edge])?;
        "route_traci".to_string()
    };
    println!("Using route '{route_id}'");

    for i in 0..NUM_VEHICLES {
        let vid = format!("traci_veh_{i}");
        client.vehicle_add(&vid, &route_id, "DEFAULT_VEHTYPE")?;
        println!("Added vehicle '{vid}'");
    }

    let mut subscribed: HashSet<String> = HashSet::new();
    let mut step = 0u32;

    // simulation_step returns Ok(false) when SUMO sends CMD_CLOSE at end-of-sim.
    while client.simulation_step(0.0)? {
        step += 1;

        let ids = client.vehicle_get_id_list()?;

        // Stop as soon as all vehicles have left the network.
        if ids.is_empty() {
            println!("All vehicles finished. Stopping after {step} steps.");
            break;
        }

        // Subscribe any vehicle we haven't seen before (one call per vehicle lifetime).
        for id in &ids {
            if subscribed.insert(id.clone()) {
                client.vehicle_subscribe_kinematics(id, 0.0, 1e9)?;
                println!("Step {step:>4} — subscribed '{id}'");
            }
        }

        println!("--- Step {:>4}  ({} vehicles) ---", step, ids.len());
        for id in &ids {
            // Reads from the local cache — no socket I/O.
            if let Some(k) = client.vehicle_get_subscribed_kinematics(id) {
                println!(
                    "  Vehicle {:20}  pos=({:8.2}, {:8.2})  \
                     speed={:6.2} m/s  accel={:6.2} m/s\u{00b2}  angle={:6.1}\u{00b0}",
                    id, k.position.x, k.position.y, k.speed, k.acceleration, k.angle
                );
            }
        }
    }

    client.close()?;
    Ok(())
}
