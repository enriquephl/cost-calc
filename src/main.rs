use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey, program_pack::Pack};
use spl_token::state::Account as TokenAccount;
use std::str::FromStr;

const SOL_ADDR: &str = "4cmpbNjZjpUrs48ZUc741YFjdhpeGxoZgURf2VnmYFEj";
const SOL_TOKEN_ADDR: &str = "7BgBvyjrZX1YKz4oh9mjb8ZScatkkwb8DzFx7LoiVkM3"; // $SLERF

fn main() {
    // solana mainnet api
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = RpcClient::new(rpc_url);

    // the solana account address to query
    let sol_pubkey = Pubkey::from_str(SOL_ADDR).expect("Invalid public key");
    let token_pubkey = Pubkey::from_str(SOL_TOKEN_ADDR).expect("Invalid token mint address");

    // get $sol balance
    match client.get_balance(&sol_pubkey) {
        Ok(balance) => {
            println!("The balance of {} is {} SOL", SOL_ADDR, lamports_to_sol(balance));     
        }
        Err(e) => {
            eprintln!("Error retrieving balance: {}", e);
        }
    }

    // get spl token balance
    match client.get_token_accounts_by_owner(&sol_pubkey, spl_token::token::option::TokenAccountsFilter::Mint(token_pubkey)) {
        Ok(token_accounts) => {
            for wrapped_account in token_accounts {
                match TokenAccount::unpack(&wrapped_account.account.data) {
                    Ok(account_info) => {
                        if account_info.mint == token_pubkey {
                            println!("The balance of SPL Token for {} is {}", SOL_ADDR, account_info.amount);
                        }
                    },
                    Err(_) => eprintln!("Failed to unpack token account data for account {}", wrapped_account.pubkey),
                }
            }
        },
        Err(e) => {
            eprintln!("Error retrieving SPL token balance: {}", e);
        }
    }
}