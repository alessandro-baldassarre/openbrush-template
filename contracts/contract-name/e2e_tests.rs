use crate::contract_name::*;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn e2e_can_update_members(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let constructor = ContractRef::try_new();

    let _contract_acc_id = client
        .instantiate("contract-name", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    Ok(())
}
