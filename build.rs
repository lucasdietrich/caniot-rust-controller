const PROTO_DIR: &str = "proto";
const PROTO_FILES: &[&str] = &[
    "proto/legacy.proto",
    "proto/ng_controller.proto",
    "proto/ng_devices.proto",
    "proto/ng_internal.proto",
    "proto/ng_heaters.proto",
    "proto/ng_garage.proto",
    "proto/ng_alarms.proto",
    "proto/common.proto",
];

const PROTO_CAN_IFACE: &str = "proto/ng_can_iface.proto";

const DB_MIGRATION_DIR: &str = "migrations";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(PROTO_FILES, &[PROTO_DIR])?;

    #[cfg(any(feature = "grpc-can-iface-server", feature = "grpc-can-iface-client"))]
    tonic_build::configure()
        .build_server(cfg!(feature = "grpc-can-iface-server"))
        .build_client(cfg!(feature = "grpc-can-iface-client"))
        .compile(&[PROTO_CAN_IFACE], &[PROTO_DIR])?;

    // build on change
    for proto in PROTO_FILES.iter() {
        println!("cargo:rerun-if-changed={}", proto);
    }

    #[cfg(any(feature = "grpc-can-iface-server", feature = "grpc-can-iface-client"))]
    println!("cargo:rerun-if-changed={}", PROTO_CAN_IFACE);

    println!("cargo:rerun-if-changed={}", DB_MIGRATION_DIR);

    Ok(())
}
