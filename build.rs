const PROTO_DIR: &str = "proto";
const PROTO_FILES: &[&str] = &["proto/ng.proto", "proto/common.proto", "proto/legacy.proto"];

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
