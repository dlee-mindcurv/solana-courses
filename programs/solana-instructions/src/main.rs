use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_system_interface::instruction::transfer;

#[tokio::main]
async fn main() -> Result<()> {
    example_transfer_instruction().await?;
    create_instruction_example().await
}

const RPC_URL_LOCAL: &str = "http://localhost:8899";


async fn example_transfer_instruction() -> Result<()> {
    //create a client connection
    let client =
        RpcClient::new_with_commitment(RPC_URL_LOCAL.to_string(), CommitmentConfig::confirmed());

    //generate keypair for sender and receiver
    let sender = Keypair::new();
    let receiver = Keypair::new();

    //airdrop some SOL to the sender;
    let airdrop_signature = client
        .request_airdrop(&sender.pubkey(), LAMPORTS_PER_SOL)
        .await?;

    loop {
        let confirmation = client.confirm_transaction(&airdrop_signature).await?;
        if confirmation {
            break;
        }
    }

    println!("Airdrop succeeded");

    // check balance before transfer
    let sender_pre_balance = client.get_balance(&sender.pubkey()).await?;
    let recipient_pre_balance = client.get_balance(&receiver.pubkey()).await?;

    println!("Sender pre balance {}", sender_pre_balance);
    println!("Receiver pre balance {}", recipient_pre_balance);

    // define the amunt to send
    let amount_to_send = LAMPORTS_PER_SOL / 100;

    //Create an instruction
    let transfer_instruction = transfer(&sender.pubkey(), &receiver.pubkey(), amount_to_send);

    // add the instruction to a new transaction
    let mut transaction =
        Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
    let blockhash = client.get_latest_blockhash().await?;

    // this is why transaction is defined as being mutable
    transaction.sign(&[&sender], blockhash);

    //send the transaction on the network
    match client.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("transfer tx successful: {}", signature)
        }
        Err(error) => {
            println!("{:?}", error)
        }
    }

    let sender_post_balance = client.get_balance(&sender.pubkey()).await?;
    let receiver_post_balance = client.get_balance(&receiver.pubkey()).await?;

    println!("Sender post balance {}", sender_post_balance);
    println!("Receiver post balance {}", receiver_post_balance);

    Ok(())
}


async fn create_instruction_example() -> Result<()> {
    //create sender
    let sender = Keypair::new();
    let recipient = Keypair::new();

    println!("sender address {}", &sender.pubkey());
    println!("receiver address {}",&recipient.pubkey());

    let solana_client = RpcClient::new_with_commitment(
        "http://localhost:8899".to_string(),
        CommitmentConfig::confirmed(),
    );





    let airdrop_signature = solana_client
        .request_airdrop(&sender.pubkey(), LAMPORTS_PER_SOL)
        .await?;


    loop {
        let confirmed = solana_client.confirm_transaction(&airdrop_signature).await?;
        if confirmed {
            break;
        }
    }

    println!("Airdrop Succeeded");

    let send_amount = LAMPORTS_PER_SOL / 100;
    let transfer_instruction = transfer(&sender.pubkey(), &recipient.pubkey(), send_amount);

    let mut transfer_transaction = Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));

    println!("{:#?}", transfer_transaction);

    // let recent_blockchainhash = solana_client.get_latest_blockhash().await?;
    //
    // transfer_transaction.sign(&[sender],recent_blockchainhash);
    //
    // match solana_client.send_and_confirm_transaction(&transfer_transaction).await {
    //     Ok(signature) => {
    //         println!("Transfer succeeded {}", signature)
    //     },
    //     Err(E) => {
    //         println!("There was an error transferring funds")
    //     }
    // };

    Ok(())
}
