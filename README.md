# ArtCraft Services & Cluster Monitor

High-performance background services, asset pipelines, and worker daemon cluster management for the ArtCraft creative ecosystem, featuring a native **Martensite** GPU-accelerated telemetry and administrative control dashboard.

![ArtCraft Services Monitor on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode administrative dashboard, cluster health telemetry, and worker pool manager.
- **`crates/services`**: Distributed job queue, asset caching service, render farm dispatch, and WebAssembly plugin runtime.

## Features

- Real-time node status inspection, CPU/GPU compute metrics, and memory utilization monitors.
- Zero-dependency native Rust UI utilizing the Martensite engine.
- Instant cluster rebalancing, queue management, and service health diagnostics.

## Legal & Compliance Notice

ArtCraft Services is an independent open-source service architecture for creative tooling. It is not affiliated with Adobe Inc.

## License

ArtCraft Services is dual-licensed under MIT or Apache-2.0.
