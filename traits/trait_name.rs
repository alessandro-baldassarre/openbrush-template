use super::TraitNameError;

#[openbrush::wrapper]
pub type TraitNameRef = dyn TraitName;

#[openbrush::trait_definition]
pub trait TraitName {
    #[ink(message)]
    fn get_value(&self) -> Result<bool, TraitNameError>;

    #[ink(message)]
    fn flip(&mut self) -> Result<(), TraitNameError>;
}
