#[derive(Debug)]
pub enum Error {
    InvalidDosSignature,
    InvalidNtHeaderOffset,
    InvalidNtSignature,
    UnsupportedMachine,
    InvalidFileHeaderCharacteristics,
    InvalidOptionalHeaderMagic,
}

pub type Result<T> = core::result::Result<T, Error>;
