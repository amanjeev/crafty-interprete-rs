use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("We got unexpected character.")]
    UnexpectedCharacter,
}

fn error(line: usize, message: &str) -> Result<(), io::Error> {
    report(line, "", message)?;
    Ok(())
}

fn report(line: usize, r#where: &str, message: &str) -> Result<bool, io::Error> {
    let is_error = true;
    println!("[line: {}] Error {}: {}", line, r#where, message);
    Ok(is_error)
}
