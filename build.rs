fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/session/client.proto")?;
    tonic_build::compile_protos("proto/session/token_verifier.proto")?;
    tonic_build::compile_protos("proto/backend/client.proto")?;
    Ok(())
}
