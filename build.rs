fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut prost_build = prost_build::Config::new();
    // prost_build.protoc_executable("protoc-27.1");
    tonic_prost_build::compile_protos("proto/*.proto")?;
    // prost_build::compile_protos(&["proto/helloworld.proto"], &["src/"])?;

    Ok(())
}