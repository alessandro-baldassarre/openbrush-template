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
    default fn get_value(&self) -> Result<bool, TraitNameError> {
        Ok(self.data().value)
    }
    default fn flip(&mut self) -> Result<(), TraitNameError> {
        self.data().value = !self.data().value;
        let result = self.data().value;
        self._emit_flip_event(result);
        Ok(())
    }
}

pub trait Internal {
    fn _emit_flip_event(&self, _value: bool) {}
}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_flip_event(&self, _value: bool) {}
}
