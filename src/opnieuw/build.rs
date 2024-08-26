fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the output directory is typically in /target, we specifically want to override this
    // environment variables passed through terminal don't make it here, so we need to set it ourselves
    std::env::set_var("OUT_DIR", "src/grpc");
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .compile(&["src/grpc/all.proto"], &["src/grpc"])?;

    Ok(())
}
