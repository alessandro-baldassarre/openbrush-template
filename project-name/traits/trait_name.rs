use super::TraitNameError;

#[openbrush::wrapper]
pub type TraitNameRef = dyn TraitName;

#[openbrush::trait_definition]
pub trait TraitName {
    #[ink(message)]
    fn flip(&self) -> Result<bool, TraitNameError>;
}

pub trait Internal {
    fn _emit_flip_event(&self, _value: bool);
}
