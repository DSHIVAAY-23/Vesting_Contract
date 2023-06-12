// #[entry_point]
// pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> StdResult<Response> {
//     // delete all state
//     let keys: Vec<_> = deps
//         .storage
//         .range(None, None, Order::Ascending)
//         .map(|(k, _)| k)
//         .collect();
//     let count = keys.len();
//     for k in keys {
//         deps.storage.remove(&k);
//     }

//     // get balance and send all to recipient
//     let balance = deps.querier.query_all_balances(env.contract.address)?;
//     let send = BankMsg::Send {
//         to_address: msg.payout.clone(),
//         amount: balance,
//     };

//     let data_msg = format!("burnt {} keys", count).into_bytes();

//     Ok(Response::new()
//         .add_message(send)
//         .add_attribute("action", "burn")
//         .add_attribute("payout", msg.payout)
//         .set_data(data_msg))
// }
