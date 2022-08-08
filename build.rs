fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .include_file("mod.rs")
        .compile(
            &[
                "proto/messages/bar.proto",
                "proto/messages/bar.proto",
                "proto/services/first-service.proto",
                "proto/services/second-service.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}