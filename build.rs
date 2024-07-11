fn main() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor_set_path = format!(
        "{}/helloworld_descriptor.bin",
        std::env::var("OUT_DIR").unwrap()
    );

    let files = &["helloworld.proto"];

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(descriptor_set_path)
        .compile(files, &["helloworld"])?;

    Ok(())
}
