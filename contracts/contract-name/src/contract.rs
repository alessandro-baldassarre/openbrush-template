#[openbrush::contract]
pub mod contract_name {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;
    use pack_name::{
        impls::*,
        traits::Internal,
    };

    use crate::error::ContractError;

    #[ink(event)]
    pub struct ValueFlipped {
        #[ink(topic)]
        pub value: bool,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pack_name: pack_name::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn try_new() -> Result<Self, ContractError> {
            Ok(Self::default())
        }
    }

    impl PackName for Contract {}

    impl Internal for Contract {
        fn _emit_flip_event(&self, value: bool) {
            EmitEvent::<Contract>::emit_event(self.env(), ValueFlipped { value })
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::test::EmittedEvent;

        use super::*;

        type Event = <Contract as ::ink::reflect::ContractEventBase>::Type;

        fn decode_events(emittend_events: Vec<EmittedEvent>) -> Vec<Event> {
            emittend_events
                .into_iter()
                .map(|event| {
                    <Event as scale::Decode>::decode(&mut &event.data[..]).expect("invalid data")
                })
                .collect()
        }

        #[ink::test]
        fn construction_work() {
            let contract = Contract::try_new().unwrap();
            assert_eq!(contract.pack_name.value, false);
        }

        #[ink::test]
        fn flip_work() {
            let contract = Contract::try_new().unwrap();
            let result = contract.flip().unwrap();
            assert_eq!(result, true);
        }

        #[ink::test]
        fn emit_event_work() {
            let contract = Contract::try_new().unwrap();
            let result = contract.flip().unwrap();
            assert_eq!(result, true);
            let emitted_events: Vec<EmittedEvent> = ink::env::test::recorded_events().collect();
            let decoded_events = decode_events(emitted_events);
            let Event::ValueFlipped(ValueFlipped { value }) = decoded_events[0];
            assert_eq!(value, true);
        }
    }
}
