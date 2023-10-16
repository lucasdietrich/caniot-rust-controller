fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                "proto/ng.proto",
                "proto/common.proto",
                "proto/legacy.proto"
            ],
            &["proto"]
        )?;
    Ok(())
}
