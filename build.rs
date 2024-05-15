const PROTO_DIR: &str = "proto";
const PROTO_FILES: &[&str] = &[
    "proto/legacy.proto",
    "proto/ng_controller.proto",
    "proto/ng_devices.proto",
    "proto/ng_internal.proto",
    "proto/ng_heaters.proto",
    "proto/ng_garage.proto",
    "proto/common.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(PROTO_FILES, &[PROTO_DIR])?;

    // build on change
    for proto in PROTO_FILES.iter() {
        println!("cargo:rerun-if-changed={}", proto);
    }

    Ok(())
}
