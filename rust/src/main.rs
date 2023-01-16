use std::env;
use std::str::FromStr;
use rand::Rng;

use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::types::{Address, H160, U256};
use secp256k1::SecretKey;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    // Connect to Infura with websocket protocol
    let websocket = web3::transports::WebSocket::new(
        &env::var("INFURA_GOERLI").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    // Accounts in node. Adding local account.
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("RASPBERRY_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    // Get balance of accounts
    let gwei_conv: U256 = U256::exp10(9);
    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!(
            "Eth balance of {:?}: {} Gwei",
            account,
            balance.checked_div(gwei_conv).unwrap()
        );
    }

    // Configure smart contract (address and ABI interface)
    let contract_addr = Address::from_str(&env::var("CONTRACT_ADDRESS").unwrap()).unwrap();
    let contract = Contract::from_json(
        web3s.eth(), contract_addr,
        include_bytes!("../../erc20_sensor_abi.json")).unwrap();

    // Read value from smart contract
    let result = contract.query("read", (1_u32,), None, Options::default(), None);
    let (temps, timestamps): (Vec<i32>, Vec<U256>) = result.await.unwrap();
    println!("Reading 1 temperature from contract:");
    println!("Temperature = {}  Timestamp = {}", temps[0], timestamps[0]);

    // Read private key from the account to be able to sign transactions
    // locally. (required beacuse using Infura instead of local node).
    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk: secp256k1::SecretKey = SecretKey::from_str(
        &env::var("RASPBERRY_PRIV").unwrap()).unwrap();

    // Send one single temperature and timestamp to smart contract
    println!("Sending 1 temperature to contract. Calling save()");
    let tx = contract
        .signed_call("save", (35_i32, 1672905713_u32),
                     Options::with(|opt| opt.gas = Some(2_000_000.into())),
                     SecretKeyRef::new(&prvk))
        .await.unwrap();
    println!("TxHash: {:#x}", tx);

    // Send array of temperature and timestamps
    println!("Sending 10 temperatures at once to contract. Calling save_many()");

    // genera 10 timestamps comenzando en 2023-01-12 9:00:00, 1 cada hora
    let start = 1673510400_u32;
    let timestamps: Vec<u32> = (0..10).map(|i| start + i * 3600).collect();
    
    // genera 10 temperaturas aleatorias entre 13ºC y 26ºC
    let mut rng = rand::thread_rng();
    let temps: Vec<i32> = (0..10).map(|_| rng.gen_range(1300..=2600)).collect();
    println!("timestamps: {:?}", timestamps);
    println!("temps: {:?}", temps);
    
    let result = contract
        .signed_call("save_many", (temps, timestamps),
                     Options::with(|opt| {
                         opt.gas = Some(4_000_000.into());
                         //opt.gas_price = Some(20_000_000_000.into());
                     }), SecretKeyRef::new(&prvk))
        .await;
    match result {
        Ok(tx) => println!("TxHash: {:#x}", tx),
        Err(error) => panic!("Error en transacción: {:?}", error),
    }
    
    Ok(())
}
