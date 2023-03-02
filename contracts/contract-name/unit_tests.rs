use ink::env::test::EmittedEvent;
use project_name::traits::trait_name::TraitName;

use crate::contract_name::*;

type Event = <Contract as ::ink::reflect::ContractEventBase>::Type;

fn decode_events(emittend_events: Vec<EmittedEvent>) -> Vec<Event> {
    emittend_events
        .into_iter()
        .map(|event| <Event as scale::Decode>::decode(&mut &event.data[..]).expect("invalid data"))
        .collect()
}

#[ink::test]
fn construction_work() {
    let contract = Contract::try_new().unwrap();
    assert_eq!(contract.get_value().unwrap(), false);
}

#[ink::test]
fn flip_work() {
    let mut contract = Contract::try_new().unwrap();
    contract.flip().unwrap();
    assert_eq!(contract.get_value().unwrap(), true);
}

#[ink::test]
fn emit_event_work() {
    let mut contract = Contract::try_new().unwrap();
    contract.flip().unwrap();
    let emitted_events: Vec<EmittedEvent> = ink::env::test::recorded_events().collect();
    let decoded_events = decode_events(emitted_events);
    let Event::ValueFlipped(ValueFlipped { value }) = decoded_events[0];
    assert_eq!(value, true);
}
