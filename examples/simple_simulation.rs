// SPDX-License-Identifier: EPL-2.0
// Simple SUMO simulation example using sumo-traci.
//
// Before running this example, start SUMO with a TraCI server:
//
//   sumo --net-file net.xml \
//        --route-files routes.rou.xml \
//        --remote-port 8813 \
//        --num-clients 1
//
// Then run:
//   cargo run --example simple_simulation

use traci_rs::TraciClient;

fn main() -> Result<(), traci_rs::TraciError> {
    // 1. Connect to the running SUMO instance.
    let mut client = TraciClient::connect("127.0.0.1", 8813)?;
    println!("Connected to SUMO TraCI server.");

    // 2. Set client order (required when multiple clients connect).
    client.set_order(1)?;

    // 3. Query the TraCI / SUMO version.
    let (api_version, sumo_version) = client.get_version()?;
    println!("TraCI API version : {}", api_version);
    println!("SUMO version      : {}", sumo_version);

    // 4. Run the simulation for 100 steps, printing vehicle positions.
    //
    // The pattern: obtain a raw pointer to the scope (which holds no socket or
    // buffer state) so we can call scope methods while also passing &mut client.
    for step in 0..100 {
        client.simulation_step(0.0)?;

        // SAFETY: VehicleScope contains only subscription-result HashMaps, no
        // socket or buffer data. We obtain a shared reference to the scope and
        // a mutable reference to the rest of the client without any aliased
        // mutable state.
        let veh_ptr: *const traci_rs::VehicleScope = &client.vehicle;
        let vehicle = unsafe { &*veh_ptr };

        let ids = vehicle.get_id_list(&mut client)?;
        if !ids.is_empty() {
            println!("--- Step {:>3}  ({} vehicles) ---", step + 1, ids.len());
            for id in &ids {
                let pos   = vehicle.get_position(&mut client, id)?;
                let speed = vehicle.get_speed(&mut client, id)?;
                println!(
                    "  Vehicle {:20}  pos=({:8.2}, {:8.2})  speed={:.2} m/s",
                    id, pos.x, pos.y, speed
                );
            }
        }
    }

    // 5. Close the connection gracefully.
    client.close()?;
    println!("Connection closed.");
    Ok(())
}
