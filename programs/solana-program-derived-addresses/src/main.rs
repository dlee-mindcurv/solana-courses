use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    derive_pda_address_with_optional_string().await;
    derive_pda_address_with_address_seed().await;
    derive_pda_from_multiple_seeds().await

}

// Address 11111111111111111111111111111111 is the System Program


async fn derive_pda_address_with_optional_string() -> anyhow::Result<()> {
    println!("find_pda_address");
    // let program_address = Pubkey::from_str("11111111111111111111111111111111")?;
    // let seed = &[b"helloWorld"];
    // let (pda_address, bump) = Pubkey::find_program_address(seed, &program_address);

    let program_address = Pubkey::from_str("11111111111111111111111111111111")?;
    let seeds:&[&[u8]] = &[b"helloWorld"];

    let (pda, bump) = Pubkey::find_program_address(seeds, &program_address);


    println!("pda_address: {}", pda);
    println!("bump: {}", bump);

    Ok(())
}

async fn derive_pda_address_with_address_seed() -> anyhow::Result<()> {
    println!("derive_pda_address_with_multiple_seeds");
    let program_address = Pubkey::from_str("11111111111111111111111111111111")?;
    let option_seed_address = Pubkey::from_str("DC7R43exz5Uhgi6dxFzNG64YUWooVB7PWnzvbAuLd561")?;
    println!("option_seed_address: {:?}", option_seed_address);
    let seeds: &[&[u8]] = &[option_seed_address.as_ref()];

    let (pda, bump) = Pubkey::find_program_address(seeds, &program_address);

    println!("pda_address: {}", pda);
    println!("bump: {}", bump);


    Ok(())
}

async fn derive_pda_from_multiple_seeds() -> Result<()> {
    println!("derive_pda_from_multiple_seeds");
    let program_id = Pubkey::from_str("11111111111111111111111111111111")?;
    let optional_public_address = Pubkey::from_str("DC7R43exz5Uhgi6dxFzNG64YUWooVB7PWnzvbAuLd561")?;
    let string_seed: &[u8] = b"this sis a seed";

    let optional_address_seed = optional_public_address.as_ref();

    let (pda, bump) = Pubkey::find_program_address(&[string_seed, optional_address_seed],&program_id);

    println!("pda_address: {}", pda);
    println!("bump: {}", bump);

    Ok(())
}
