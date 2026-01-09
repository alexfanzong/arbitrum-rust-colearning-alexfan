use alloy::{
    providers::{ProviderBuilder, Provider},
    sol,
};
use eyre::Result;
use std::str::FromStr;
use alloy::primitives::Address;

// 1. 定义老师作业里的合约接口 (HelloWeb3)
// 这就像是告诉代码：我们要找的这个合约，只有一个叫 hello_web3 的功能
sol! {
    #[sol(rpc)]
    contract HelloWeb3 {
        function hello_web3() pure public returns(string memory);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 2. 切换到老师用的【测试网节点】 (Arbitrum Sepolia)
    // 注意：这个网址跟刚才不一样，这是测试用的
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    // 3. 连接网络
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 4. 输入老师指定的【合约地址】
    // 这就是我们要去敲门的那家店
    let contract_address = Address::from_str("0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90")?;

    // 5. 创建合约实例
    let contract = HelloWeb3::new(contract_address, provider);

    // 6. 调用 hello_web3 方法，拿到作业答案
    let message = contract.hello_web3().call().await?;

    // 7. 打印出来！
    // 这里的 message 就是老师要的 "Hello web3"
    println!("Contract message: {}", message);

    Ok(())
}
