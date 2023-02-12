use crate::traits::TraitNameError;
pub use crate::{
    impls::trait_name::{
        data,
        data::*,
        trait_name,
        *,
    },
    traits::trait_name::*,
};
use openbrush::traits::Storage;

impl<T: Storage<Data>> TraitName for T {
    default fn flip(&self) -> Result<bool, TraitNameError> {
        let result = !self.data::<Data>().value;
        self._emit_flip_event(result);
        Ok(result)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_flip_event(&self, _value: bool) {}
}
