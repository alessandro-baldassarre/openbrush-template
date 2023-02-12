use crate::{
    data::Data,
    error::PackNameError,
    traits::Internal,
};

pub use crate::traits::*;

use openbrush::traits::Storage;

impl<T: Storage<Data>> PackName for T {
    default fn flip(&self) -> Result<bool, PackNameError> {
        let result = !self.data::<Data>().value;
        self._emit_flip_event(result);
        Ok(result)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_flip_event(&self, _value: bool) {}
}
