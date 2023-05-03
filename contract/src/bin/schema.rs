use std::env::current_dir;

use cosmwasm_schema::{export_schema, schema_for, write_api};
use cosmwasm_std::Empty;
use crypto_pawn::msg::{ExecuteMsg, InstantiateMsg, SudoMsg};

fn main() {
    // Clear & write standard API
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        sudo: SudoMsg,
        migrate: Empty,
    }

    // Schemas for inter-contract communication
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    // export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(SudoMsg), &out_dir);
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
}
