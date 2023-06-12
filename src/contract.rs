#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Attribute, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdError, StdResult, Uint128, WasmMsg,
};



use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, Denom};
use cw_storage_plus::Bound;
use schemars::_serde_json::to_string;

use crate::msg::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg, VestingAccountResponse, VestingData,
    VestingSchedule,
};
use crate::state::{denom_to_key, VestingAccount, VESTING_ACCOUNTS};
const CONTRACT_NAME: &str = "crates.io:cw20-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::RegisterVestingAccount {
            admin_address,
            address,
            vesting_schedule,
        } => {
            // deposit validation
            if info.funds.len() != 1 {
                return Err(StdError::generic_err("must deposit only one type of token"));
            }

            let deposit_coin = info.funds[0].clone();
            register_vesting_account(
                deps,
                env,
                admin_address,
                address,
                Denom::Native(deposit_coin.denom),
                deposit_coin.amount,
                vesting_schedule,
            )
        }
       
        ExecuteMsg::Claim { denoms, recipient } => claim(deps, env, info, denoms, recipient),
    }
}

fn register_vesting_account(
    deps: DepsMut,
    env: Env,
    admin_address: String,
    address: String,
    deposit_denom: Denom,
    deposit_amount: Uint128,
    vesting_schedule: VestingSchedule,
) -> StdResult<Response> {
    let denom_key = denom_to_key(deposit_denom.clone());

    // vesting_account existence check
    if VESTING_ACCOUNTS.has(deps.storage, (address.as_str(), &denom_key)) {
        return Err(StdError::generic_err("already exists"));
    }

    // validate vesting schedule
    match vesting_schedule.clone() {
        VestingSchedule::Vesting {
            start_time,
            end_time,
            vesting_amount,
        } => {
            if vesting_amount.is_zero() {
                return Err(StdError::generic_err("assert(vesting_amount > 0)"));
            }

            let start_time = start_time
                .parse::<u64>()
                .map_err(|_| StdError::generic_err("invalid start_time"))?;

            let end_time = end_time
                .parse::<u64>()
                .map_err(|_| StdError::generic_err("invalid end_time"))?;

            if start_time < env.block.time.seconds() {
                return Err(StdError::generic_err("assert(start_time < block_time)"));
            }

            if end_time <= start_time {
                return Err(StdError::generic_err("assert(end_time <= start_time)"));
            }

            if vesting_amount != deposit_amount {
                return Err(StdError::generic_err(
                    "assert(deposit_amount == vesting_amount)",
                ));
            }
        }
        
        
    }

    VESTING_ACCOUNTS.save(
        deps.storage,
        (address.as_str(), &denom_key),
        &VestingAccount {
            admin_address: admin_address.clone(),
            address: address.to_string(),
            vesting_denom: deposit_denom.clone(),
            vesting_amount: deposit_amount,
            vesting_schedule,
            claimed_amount: Uint128::zero(),
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        ("action", "register_vesting_account"),
        (
            "admin_address",
            admin_address.as_str(),
        ),
        ("address", address.as_str()),
        ("vesting_denom",  &to_string(&deposit_denom).unwrap()),
        ("vesting_amount", &deposit_amount.to_string()),
    ]))
}



fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denoms: Vec<Denom>,
    recipient: String,
) -> StdResult<Response> {
    let sender = info.sender;
    

    let mut messages: Vec<CosmosMsg> = vec![];
    let mut attrs: Vec<Attribute> = vec![];
    for denom in denoms.iter() {
        let denom_key = denom_to_key(denom.clone());

        // vesting_account existence check
        let account = VESTING_ACCOUNTS.may_load(deps.storage, (sender.as_str(), &denom_key))?;
        if account.is_none() {
            return Err(StdError::generic_err(format!(
                "vesting entry is not found for denom {}",
                to_string(&denom).unwrap(),
            )));
        }

        let mut account = account.unwrap();
        let vested_amount = account
            .vesting_schedule
            .vested_amount(env.block.time.seconds())?;
        let claimed_amount = account.claimed_amount;

        let claimable_amount = vested_amount.checked_sub(claimed_amount)?;
        if claimable_amount.is_zero() {
            continue;
        }

        account.claimed_amount = vested_amount;
        if account.claimed_amount == account.vesting_amount {
            VESTING_ACCOUNTS.remove(deps.storage, (sender.as_str(), &denom_key));
        } else {
            VESTING_ACCOUNTS.save(deps.storage, (sender.as_str(), &denom_key), &account)?;
        }

        let message: CosmosMsg = match account.vesting_denom.clone() {
            Denom::Native(denom) => BankMsg::Send {
                to_address: recipient.clone(),
                amount: vec![Coin {
                    denom,
                    amount: claimable_amount,
                }],
            }
            .into(),
            Denom::Cw20(contract_addr) => WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: recipient.clone(),
                    amount: claimable_amount,
                })?,
                funds: vec![],
            }
            .into(),
        };

        messages.push(message);
        attrs.extend(
            vec![
                Attribute::new("vesting_denom", &to_string(&account.vesting_denom).unwrap()),
                Attribute::new("vesting_amount", &account.vesting_amount.to_string()),
                Attribute::new("vested_amount", &vested_amount.to_string()),
                Attribute::new("claim_amount", &claimable_amount.to_string()),
            ]
            .into_iter(),
        );
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attributes(vec![("action", "claim"), ("address", sender.as_str())])
        .add_attributes(attrs))
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let amount = cw20_msg.amount;
    let _sender = cw20_msg.sender;
    let contract = info.sender;

    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::RegisterVestingAccount {
            admin_address,
            address,
            vesting_schedule,
        }) => register_vesting_account(
            deps,
            env,
            admin_address,
            address,
            Denom::Cw20(contract),
            amount,
            vesting_schedule,
        ),
        Err(_) => Err(StdError::generic_err("invalid cw20 hook message")),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VestingAccount {
            address,
            start_after,
            limit,
        } => to_binary(&vesting_account(deps, env, address, start_after, limit)?),
    }
}

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;
fn vesting_account(
    deps: Deps,
    env: Env,
    address: String,
    start_after: Option<Denom>,
    limit: Option<u32>,
) -> StdResult<VestingAccountResponse> {
    let mut vestings: Vec<VestingData> = vec![];
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    for item in VESTING_ACCOUNTS
        .prefix(address.as_str())
        .range(
            deps.storage,
            start_after
                .map(denom_to_key)
                .map(|v| v.as_bytes().to_vec())
                .map(Bound::Exclusive),
            None,
            Order::Ascending,
        )
        .take(limit)
    {
        let (_, account) = item?;
        let vested_amount = account
            .vesting_schedule
            .vested_amount(env.block.time.seconds())?;

        vestings.push(VestingData {
            admin_address: account.admin_address,
            vesting_denom: account.vesting_denom,
            vesting_amount: account.vesting_amount,
            vested_amount,
            vesting_schedule: account.vesting_schedule,
            claimable_amount: vested_amount.checked_sub(account.claimed_amount)?,
        })
    }

    Ok(VestingAccountResponse { address, vestings })
}