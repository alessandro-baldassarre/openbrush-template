use crate::error::PackNameError;

#[openbrush::wrapper]
pub type PackNameRef = dyn PackName;

#[openbrush::trait_definition]
pub trait PackName {
    #[ink(message)]
    fn flip(&self) -> Result<bool, PackNameError>;
}

pub trait Internal {
    fn _emit_flip_event(&self, _value: bool);
}
