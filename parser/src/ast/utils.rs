use crate::error::FilePosition;

///
/// Represents a fragment of the code with its location.
///
pub trait Locatable {
    fn get_position(&self) -> &FilePosition;
}
