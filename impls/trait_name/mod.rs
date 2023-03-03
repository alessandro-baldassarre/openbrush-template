pub use crate::{
    impls::trait_name,
    traits::{
        trait_name::*,
        TraitNameError,
    },
};

use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub value: bool,
}

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
