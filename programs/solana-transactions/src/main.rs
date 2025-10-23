use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_system_interface::instruction::transfer;

fn main(){
    example_transaction_structure();
}

#[tokio::main]
async fn example_transaction_structure() -> Result<()> {
    let connection = RpcClient::new_with_commitment("http://localhost:8899".to_string(), CommitmentConfig::confirmed());

    let blockhash = connection.get_latest_blockhash().await?;

    let sender = Keypair::new();
    let receiver = Keypair::new();

    let airdrop_sig = connection.request_airdrop(&sender.pubkey(),LAMPORTS_PER_SOL).await?;

    loop {
        let confirmed = connection.confirm_transaction(&airdrop_sig).await?;
        if confirmed {
            break;
        }
    }

    let amount = LAMPORTS_PER_SOL / 100;

    let transfer_instruction = transfer(&sender.pubkey(), &receiver.pubkey(), amount);
    let transfer_instruction_2= transfer(&sender.pubkey(), &receiver.pubkey(), amount);
    let transfer_instruction_3= transfer(&sender.pubkey(), &receiver.pubkey(), amount);

    let mut transfer_transaction = Transaction::new_with_payer(&[transfer_instruction, transfer_instruction_2, transfer_instruction_3], Some(&sender.pubkey()));
    transfer_transaction.sign(&[&sender], blockhash);

    println!("transfer_transaction: {:#?}", transfer_transaction);

    let tarnsfer_signature = connection.send_and_confirm_transaction(&transfer_transaction).await?;

    println!("tarnsfer_signature: {:#?}", transfer_transaction);



    Ok(())
}

