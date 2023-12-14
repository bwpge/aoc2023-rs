use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    EmitBuilder::builder().build_date().git_sha(true).emit()?;

    Ok(())
}
