use cosmwasm_std::{StdResult, Uint128};
use cw20::{Cw20ReceiveMsg, Denom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),

    

    RegisterVestingAccount {
        admin_address:String, 
        address: String,
        vesting_schedule: VestingSchedule,
    },
    /// only available when admin_address was set
    
   
    Claim {
        denoms: Vec<Denom>,
        recipient:String,
    },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Register vesting account with token transfer
    RegisterVestingAccount {
        admin_address:String, // if given, the vesting account can be unregistered
        address: String,
        vesting_schedule: VestingSchedule,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    VestingAccount {
        address: String,
        start_after: Option<Denom>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct VestingAccountResponse {
    pub address: String,
    pub vestings: Vec<VestingData>,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct VestingData {
    pub admin_address:String,
    pub vesting_denom: Denom,
    pub vesting_amount: Uint128,
    pub vested_amount: Uint128,
    pub vesting_schedule: VestingSchedule,
    pub claimable_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VestingSchedule {
    
    Vesting {
        start_time: String,      // vesting start time in second unit
        end_time: String,        // vesting end time in second unit
        vesting_amount: Uint128, // total vesting amount
    },
    
}

impl VestingSchedule {
    pub fn vested_amount(&self, block_time: u64) -> StdResult<Uint128> {
        match self {
            VestingSchedule::Vesting {
                start_time,
                end_time,
                vesting_amount,
            } => { 
                let start_time = start_time.parse::<u64>().unwrap();
                let end_time = end_time.parse::<u64>().unwrap();

                if block_time <= start_time {
                    return Ok(Uint128::zero());
                }

                if block_time >= end_time {
                    return Ok(*vesting_amount);
                }

                let vested_token = vesting_amount
                    .checked_mul(Uint128::from(block_time - start_time))?
                    .checked_div(Uint128::from(end_time - start_time))?;

                Ok(vested_token)
            }
        }
    }
}

