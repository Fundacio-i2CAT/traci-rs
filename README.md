# traci-rs

<img src="https://raw.githubusercontent.com/Fundacio-i2CAT/FlexStack/refs/heads/master/doc/img/i2cat_logo.png" alt="i2CAT Logo" width="200"/>

[![License: EPL-2.0](https://img.shields.io/badge/License-EPL_2.0-blue.svg)](https://www.eclipse.org/legal/epl-2.0/)
[![Crates.io](https://img.shields.io/crates/v/traci-rs.svg)](https://crates.io/crates/traci-rs)
[![Docs.rs](https://docs.rs/traci-rs/badge.svg)](https://docs.rs/traci-rs)

# Short description

`traci-rs` is a pure-Rust client library for the [SUMO](https://sumo.dlr.de) **TraCI** (Traffic Control Interface) protocol. It provides the same functionality as the official C++ `TraCIAPI` library, translated to idiomatic Rust, enabling full programmatic control of SUMO traffic simulations from Rust applications.

# Documentation

API documentation is available at [docs.rs/traci-rs](https://docs.rs/traci-rs).

# Pre-requisites

## Supported platforms

Any platform supported by Rust's stable toolchain (Linux, macOS, Windows).

## Dependencies

- **Rust** stable toolchain, edition 2021 or later
- **SUMO** ≥ 1.8 installed and available in `$PATH` ([sumo.dlr.de/docs/Downloads.php](https://sumo.dlr.de/docs/Downloads.php))

This crate has **zero external Rust dependencies** beyond `std`.

## Known Limitations

- The TraCI wire protocol does not provide a machine-readable schema; this library targets SUMO ≥ 1.8 and may require updates for future protocol revisions.

# Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
traci-rs = "0.1"
```

Or, to use a local checkout:

```toml
[dependencies]
traci-rs = { path = "../traci-rs" }
```

## Launching SUMO with TraCI

SUMO must be started with the `--remote-port` flag before connecting:

```bash
sumo --net-file net.xml \
     --route-files routes.rou.xml \
     --remote-port 8813 \
     --num-clients 1
```

Use `sumo-gui` instead of `sumo` for a graphical window.

# Usage

```rust
use traci_rs::TraciClient;

fn main() -> Result<(), traci_rs::TraciError> {
    // Connect to a running SUMO instance
    let mut client = TraciClient::connect("127.0.0.1", 8813)?;

    // Set client order (required for multi-client setups)
    client.set_order(1)?;

    // Check versions
    let (api_version, sumo_version) = client.get_version()?;
    println!("SUMO {}, TraCI API {}", sumo_version, api_version);

    // Run 50 simulation steps
    for _ in 0..50 {
        client.simulation_step(0.0)?;

        let ids = client.vehicle.get_id_list(&mut client)?;
        for id in &ids {
            let pos   = client.vehicle.get_position(&mut client, id)?;
            let speed = client.vehicle.get_speed(&mut client, id)?;
            println!("  {} @ ({:.1}, {:.1})  v={:.1} m/s", id, pos.x, pos.y, speed);
        }
    }

    client.close()?;
    Ok(())
}
```

Run the bundled example with:

```bash
cargo run --example simple_simulation
```

## Supported SUMO domains

| Scope | Type | Description |
|---|---|---|
| `client.edge` | `EdgeScope` | Road edge queries and travel-time adaptation |
| `client.gui` | `GuiScope` | GUI viewport and zoom control |
| `client.induction_loop` | `InductionLoopScope` | Single-lane loop detector data |
| `client.junction` | `JunctionScope` | Junction position/shape |
| `client.lane` | `LaneScope` | Lane attributes and allowed vehicle classes |
| `client.lane_area` | `LaneAreaScope` | Multi-lane area (E2) detector data |
| `client.multi_entry_exit` | `MultiEntryExitScope` | Multi-entry/exit (E3) detector data |
| `client.person` | `PersonScope` | Pedestrian/person control |
| `client.poi` | `PoiScope` | Point-of-interest management |
| `client.polygon` | `PolygonScope` | Polygon management |
| `client.rerouter` | `RerouterScope` | Rerouter queries |
| `client.route` | `RouteScope` | Route management |
| `client.route_probe` | `RouteProbeScope` | Route probe detector data |
| `client.simulation` | `SimulationScope` | Simulation control and coordinate conversion |
| `client.traffic_lights` | `TrafficLightScope` | Traffic-light state and program control |
| `client.vehicle` | `VehicleScope` | Full vehicle control + subscription filters |
| `client.vehicle_type` | `VehicleTypeScope` | Vehicle type parameter management |

## Error handling

All fallible operations return `Result<T, TraciError>`.

```rust
use traci_rs::TraciError;

match client.vehicle.get_speed(&mut client, "nonexistent") {
    Ok(speed) => println!("speed = {}", speed),
    Err(TraciError::Io(e))       => eprintln!("I/O error: {}", e),
    Err(TraciError::Protocol(s)) => eprintln!("TraCI protocol error: {}", s),
    Err(e)                       => eprintln!("Error: {}", e),
}
```

# Developers

- Jordi Marias-i-Parella (jordi.marias@i2cat.net)

# Source

This code has been developed within the following research and innovation projects:

- **SPRINGTIME** (PID2023-146378NB-I00) funded by the Spanish government (MCIU/AEI/10.13039/501100011033/FEDER/UE), this project focuses in techniques to get IP-based interconnection on multiple environments.

# Copyright

This code has been developed by Fundació Privada Internet i Innovació Digital a Catalunya (i2CAT).

i2CAT is a **non-profit research and innovation centre that** promotes mission-driven knowledge to solve business challenges, co-create solutions with a transformative impact, empower citizens through open and participative digital social innovation with territorial capillarity, and promote pioneering and strategic initiatives. i2CAT **aims to transfer** research project results to private companies in order to create social and economic impact via the out-licensing of intellectual property and the creation of spin-offs. Find more information of i2CAT projects and IP rights at https://i2cat.net/tech-transfer/

# License

This code is licensed under the terms of the Eclipse Public License 2.0 (EPL-2.0). Information about the license can be located at https://www.eclipse.org/legal/epl-2.0/.

If you find that this license doesn't fit with your requirements regarding the use, distribution or redistribution of our code for your specific work, please, don't hesitate to contact the intellectual property managers in i2CAT at the following address: techtransfer@i2cat.net

# Attributions

This is a Rust translation of the `TraCIAPI` C++ library from the [Eclipse SUMO](https://github.com/eclipse-sumo/sumo) project, which is also licensed under EPL-2.0.

> **Copyright (c) 2001-2024 German Aerospace Center (DLR) and others — Eclipse SUMO project**  
> Original C++ source: <https://github.com/eclipse-sumo/sumo>