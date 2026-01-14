use ethers::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;

// 定义我们要调用的合约“接口清单” (ABI)
// 就像法律合同的摘要，告诉程序这个合约有 name() 和 symbol() 两个条款
abigen!(
    WETH_Contract,
    r#"[
        function name() stdcall view returns (string)
        function symbol() stdcall view returns (string)
        function decimals() stdcall view returns (uint8)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 连接到 Arbitrum Sepolia 节点
    let provider = Provider::<Http>::try_from("https://sepolia-rollup.arbitrum.io/rpc")?;
    let client = Arc::new(provider);

    // 2. 设定 WETH 合约在测试网的地址
    let address: Address = "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73".parse()?;

   // 3. 先做安全性检查（使用原件 client）
    let code = client.get_code(address, None).await?;
    if code.is_empty() {
        println!("警告：该地址上没有部署任何智能合约！");
        return Ok(());
    }

    // 4. 检查通过后，实例化合约（使用复印件 client.clone()）
    let contract = WETH_Contract::new(address, client.clone());

    // 5. 调用只读方法 (无需私钥，无需消耗 Gas)
    println!("正在查询合约信息...");
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;

    println!("---------------------------------------");
    println!("合约地址: {:?}", address);
    println!("代币名称 (Name): {}", name);
    println!("代币符号 (Symbol): {}", symbol);
    println!("精度 (Decimals): {}", decimals);
    println!("---------------------------------------");

    Ok(())
}
