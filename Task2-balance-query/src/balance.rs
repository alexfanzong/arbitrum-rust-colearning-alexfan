use ethers::prelude::*;
use ethers::utils::format_units;
use std::convert::TryFrom;

// 这是一个异步的主函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // 1. 连接到 Arbitrum Sepolia 测试网节点
    // 这里我们用官方的公共节点地址
    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc"
    )?;

    // 2. 定义你要查询的钱包地址
    let wallet_address = "0x7867C71D072df6392B6a429A32550Eb90E82c56A".parse::<Address>()?;

    // 3. 查询余额 (返回的是 Wei，最小单位)
    let balance_in_wei = provider.get_balance(wallet_address, None).await?;

    // 4. 将余额转换为可读格式 (从 Wei 转为 ETH)
    let balance_in_eth = format_units(balance_in_wei, "ether")?;

    // 5. 打印结果到终端
    println!("--------------------------------");
    println!("查询地址: {:?}", wallet_address);
    println!("当前余额: {} ETH", balance_in_eth);
    println!("--------------------------------");

    Ok(())
}