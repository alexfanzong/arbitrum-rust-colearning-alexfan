use ethers::prelude::*;
use ethers::utils::parse_ether; // 明确引入 parse_ether
use std::convert::TryFrom;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // 加载 .env 里的 PRIVATE_KEY

    // 1. 建立连接
    let provider = Provider::<Http>::try_from("https://sepolia-rollup.arbitrum.io/rpc")?;
    
    // 2. 配置钱包
    let priv_key = env::var("PRIVATE_KEY")?;
    let wallet: LocalWallet = priv_key.parse::<LocalWallet>()?.with_chain_id(421614u64);
    let client = SignerMiddleware::new(provider, wallet);

    // 3. 交易参数
    let to_address = "0xd10c17307908CF460BD650Ff593eA7fFeB32173E".parse::<Address>()?;
    let amount = parse_ether("0.0001")?; 

    println!("正在签署并发送交易...");

    // 4. 构建交易请求
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount)
        .gas_price(100_000_000u64); // 关键：在这里手动加价

    // 5. 发送并等待 (明确指定 PendingTransaction 的泛型)
    let pending_tx = client.send_transaction(tx, None).await?;
    let tx_hash = pending_tx.tx_hash();
    println!("交易已提交！哈希: {:?}", tx_hash);
    
    println!("等待区块确认收据...");
    
    // 6. 最终确认：等待收据并明确类型
    let receipt = pending_tx.await?.ok_or("无法获取交易收据")?;
    
    println!("---------------------------------------");
    println!("转账成功！状态: {:?}", receipt.status);
    println!("交易哈希: {:?}", receipt.transaction_hash);
    println!("区块高度: {:?}", receipt.block_number.unwrap_or_default());
    println!("---------------------------------------");

    Ok(())
}