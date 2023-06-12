use crate::contract::{execute, instantiate, query};
use crate::msg::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg, VestingAccountResponse, VestingData,
    VestingSchedule,
};

use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
    to_binary, Addr, Attribute, BankMsg, Coin, Response, StdError, SubMsg, Timestamp, Uint128,
    WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, Denom};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {};

    let info = mock_info("addr0000", &[]);

    
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
}

#[test]
fn linear_vesting_vested_amount() {
    let schedule = VestingSchedule::Vesting {
        start_time: "100".to_string(),
        end_time: "110".to_string(),
        vesting_amount: Uint128::new(1000000u128),
    };

    assert_eq!(schedule.vested_amount(100).unwrap(), Uint128::zero());
    assert_eq!(
        schedule.vested_amount(105).unwrap(),
        Uint128::new(500000u128)
    );
    assert_eq!(
        schedule.vested_amount(110).unwrap(),
        Uint128::new(1000000u128)
    );
    assert_eq!(
        schedule.vested_amount(115).unwrap(),
        Uint128::new(1000000u128)
    );
}

