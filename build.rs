fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "proto/messages/bar.proto",
                "proto/messages/bar.proto",
                "proto/services/first-service.proto",
                "proto/services/second-service.proto"
            ],
            &["proto/"],
        )?;
    Ok(())
}