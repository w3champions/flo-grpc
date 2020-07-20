fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("proto/auth.proto")?;
  tonic_build::compile_protos("proto/lobby.proto")?;
  tonic_build::compile_protos("proto/connect.proto")?;
  Ok(())
}
