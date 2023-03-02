use openbrush::traits::String;
use project_name::traits::TraitNameError;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    Custom(String),
    TraitNameError(TraitNameError),
}

impl From<TraitNameError> for ContractError {
    fn from(_trait_error: TraitNameError) -> Self {
        ContractError::Custom(String::from("TE trait_name error"))
    }
}
