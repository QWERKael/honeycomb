fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/communication.proto")?;
    tonic_build::compile_protos("proto/dance.proto")?;
    Ok(())
}