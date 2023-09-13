use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, Coin, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw_utils::one_coin;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    MsgSwapExactAmountIn, MsgSwapExactAmountOut, SwapAmountInRoute, SwapAmountOutRoute,
};

#[entry_point]
fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

#[cw_serde]
enum ExecuteMsg {
    Direct { token_out_denom: String },
    Reverse { want: Coin, token_in_denom: String },
}

#[entry_point]
fn execute(_deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    let swap_msg: CosmosMsg = match msg {
        ExecuteMsg::Direct { token_out_denom } => {
            let offer_coin = one_coin(&info).unwrap();

            MsgSwapExactAmountIn {
                sender: env.contract.address.to_string(),
                routes: vec![SwapAmountInRoute {
                    pool_id: 86,
                    token_out_denom,
                }],
                token_in: Some(offer_coin.into()),
                token_out_min_amount: "1".to_string(),
            }
            .into()
        }
        ExecuteMsg::Reverse {
            want,
            token_in_denom,
        } => MsgSwapExactAmountOut {
            sender: info.sender.to_string(),
            routes: vec![SwapAmountOutRoute {
                pool_id: 86,
                token_in_denom,
            }],
            token_in_max_amount: u128::MAX.to_string(),
            token_out: Some(want.into()),
        }
        .into(),
    };

    Ok(Response::new().add_message(swap_msg))
}
