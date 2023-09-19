use cosmwasm_schema::write_api;
use mars_oracle_wasm::WasmPriceSourceUnchecked;
use mars_red_bank_types::oracle::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    WasmOracleCustomExecuteMsg, WasmOracleCustomInitParams,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<WasmOracleCustomInitParams>,
        execute: ExecuteMsg<WasmPriceSourceUnchecked, WasmOracleCustomExecuteMsg>,
        query: QueryMsg,
    }
}