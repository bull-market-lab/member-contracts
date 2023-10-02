// use thread::{
//     account::{AccountResponse, AccountsResponse},
//     job::{JobResponse, JobsResponse},
//     QueryMsg, {Config, ConfigResponse, ExecuteMsg, InstantiateMsg},
// };
use cosmwasm_schema::write_api;
use thread::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    // let mut out_dir = current_dir().unwrap();
    // out_dir.push("schema");
    // create_dir_all(&out_dir).unwrap();
    // remove_schemas(&out_dir).unwrap();

    // export_schema(&schema_for!(InstantiateMsg), &out_dir);
    // export_schema(&schema_for!(ExecuteMsg), &out_dir);
    // export_schema(&schema_for!(QueryMsg), &out_dir);

    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
    // export_schema(&schema_for!(Config), &out_dir);
    // export_schema(&schema_for!(ConfigResponse), &out_dir);
    // export_schema(&schema_for!(JobResponse), &out_dir);
    // export_schema(&schema_for!(JobsResponse), &out_dir);
    // export_schema(&schema_for!(AccountResponse), &out_dir);
    // export_schema(&schema_for!(AccountsResponse), &out_dir);
}
