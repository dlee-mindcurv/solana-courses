use anyhow::Result;
use bincode::deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signer::{Signer, keypair::Keypair},
    sysvar::{self, clock::Clock},
    pubkey,
    program_pack::Pack,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_token_2022_interface::{
    id as token_2022_program_id, instruction::initialize_mint, state::Mint,
};


fn main() -> Result<()> {
    // Generate a new keypair (Account) ownd System_Program (11111111111111111111111111111111)
    system_program_account();

    // access clock from the sysvar accounts (Sysvar Account) ownned by Sysvar program
    sysvar_account();

    // create, update state of  new programs (Program Account) ownned by BPFLoader program
    program_account();

    data_account()

}

#[tokio::main]
async fn system_program_account() -> Result<()> {
    let local_rpc_url: String = String::from("http://localhost:8899");

    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());

    // Create a connection to Solana cluster
    let connection = RpcClient::new_with_commitment(local_rpc_url, CommitmentConfig::confirmed());

    // Funding an address with SOL automatically creates an account
    let signature = connection
        .request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL)
        .await?;

    loop {
        let confirmed = connection.confirm_transaction(&signature).await?;
        if confirmed {
            break;
        }
    }

    let account_info = connection.get_account(&keypair.pubkey()).await?;
    println!("{:#?}", account_info);

    Ok(())
}

// sysvar accounts
// Clock
// EpochSchedule
//Fees
// Rent
//EpochRewards

#[tokio::main]
async fn sysvar_account() -> Result<()> {
    let local_rpc_url: String = String::from("http://localhost:8899");

    let connection = solana_client::nonblocking::rpc_client::RpcClient::new_with_commitment(
        local_rpc_url,
        CommitmentConfig::confirmed(),
    );

    let account = connection.get_account(&sysvar::clock::ID).await?;
    // Deserialize the account data
    let clock: Clock = deserialize(&account.data)?;

    // let clock:Clock = Clock::get()?;

    println!("{:#?}", account);
    println!("{:#?}", clock);

    Ok(())
}

#[tokio::main]
async fn program_account() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let program_id = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    let account_info = connection.get_account(&program_id).await?;
    println!("{:#?}", account_info);

    Ok(())
}

#[tokio::main]
async fn data_account() -> Result<()> {
    // Create connection to local validator
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"),
        CommitmentConfig::confirmed(),
    );
    let recent_blockhash = client.get_latest_blockhash().await?;

    // Generate a new keypair for the fee payer
    let fee_payer = Keypair::new();

    // Airdrop 1 SOL to fee payer
    let airdrop_signature = client
        .request_airdrop(&fee_payer.pubkey(), 1_000_000_000)
        .await?;

    loop {
        let confirmed = client.confirm_transaction(&airdrop_signature).await?;
        if confirmed {
            break;
        }
    }

    // Generate keypair to use as address of mint
    let mint = Keypair::new();

    let space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(space).await?;

    // Create account instruction
    let create_account_instruction = create_account(
        &fee_payer.pubkey(),      // fee payer
        &mint.pubkey(),           // mint address
        rent,                     // rent
        space as u64,             // space
        &token_2022_program_id(), // program id
    );

    // Initialize mint instruction
    let initialize_mint_instruction = initialize_mint(
        &token_2022_program_id(),
        &mint.pubkey(),            // mint address
        &fee_payer.pubkey(),       // mint authority
        Some(&fee_payer.pubkey()), // freeze authority
        9,                         // decimals
    )?;

    // Create transaction and add instructions
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        recent_blockhash,
    );

    // Send and confirm transaction
    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    println!("Fee Payer: {}", &fee_payer.pubkey());
    println!("Mint Address: {}", mint.pubkey());
    println!("Transaction Signature: {}", transaction_signature);

    let account_info = client.get_account(&mint.pubkey()).await?;
    println!("{:#?}", account_info);

    let mint_account = Mint::unpack(&account_info.data)?;
    println!("{:#?}", mint_account);

    Ok(())
}
