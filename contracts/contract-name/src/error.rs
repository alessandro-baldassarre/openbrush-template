use pack_name::error::PackNameError;
use thiserror_no_std::Error;

#[derive(Error, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    #[error("template error")]
    PackErr(#[from] PackNameError),
}
