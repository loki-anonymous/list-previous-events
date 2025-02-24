// use tiny_keccak::{Hasher, Keccak};
// use hex;

// fn keccak256(input: &str) -> String {
//     let mut hasher = Keccak::v256();
//     let mut output = [0u8; 32];
//     hasher.update(input.as_bytes());
//     hasher.finalize(&mut output);
//     format!("0x{}", hex::encode(output))
// }

// fn main() {
//     let event1 = "ContractDeployed(address)";
//     let event2 = "NumberUpdatedEvent(uint64,address)"; 

//     println!("ContractDeployed Signature: {}", keccak256(event1));
//     println!("NumberUpdatedEvent Signature: {}", keccak256(event2));
// }




use web3::types::{BlockNumber, FilterBuilder, H256, H160};
use web3::transports::Http;
use web3::Web3;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Load environment variables from .env
    dotenv().ok();

    // Read variables from .env
    let infura_url = env::var("INFURA_URL").expect("INFURA_URL not found in .env");
    let contract_address = H160::from_str(&env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not found in .env")).unwrap();
    let event_signature: H256 = H256::from_str(&env::var("EVENT_SIGNATURE").expect("EVENT_SIGNATURE not found in .env")).unwrap();

    // Connect to Ethereum node
    let http = Http::new(&infura_url)?;
    let web3 = Web3::new(http);
    let latest_block = web3.eth().block_number().await?.as_u64();
    let from_block = latest_block - 500;

    // Build filter for logs
    let filter = FilterBuilder::default()
        .from_block(from_block.into())
        .to_block(BlockNumber::Latest)
        .address(vec![contract_address])
        .topics(Some(vec![event_signature]), None, None, None)
        .build();

    // Fetch logs
    let logs = web3.eth().logs(filter).await?;

    // Print logs
    for log in logs {
        println!("Log: {:?}", log);
    }
    println!("Using Infura URL: {}", infura_url);
    println!("Contract Address: {:?}", contract_address);
    println!("Event Signature: {:?}", event_signature);

    Ok(())
}
