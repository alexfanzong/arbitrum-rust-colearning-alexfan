use ethers::prelude::*;
use ethers::utils::format_units;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 连接到 Arbitrum Sepolia 测试网节点
    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc"
    )?;

    // 2. 获取实时 Gas 价格 (Gas Price)
    let gas_price = provider.get_gas_price().await?;

    // 3. 设置基础转账的 Gas 限额 (Gas Limit)
    // 以太坊和 Arbitrum 标准转账通常固定为 21000 units
    let gas_limit = U256::from(21000);

    // 4. 计算总 Gas 费 (Gas 费 = Gas 价格 * Gas 限额)
    let gas_fee = gas_price * gas_limit;

    // 5. 格式化输出 (从 Wei 转换为常见的 ETH 单位)
    let gas_price_in_gwei = format_units(gas_price, "gwei")?;
    let gas_fee_in_eth = format_units(gas_fee, "ether")?;

    println!("---------------------------------------");
    println!("Arbitrum 实时 Gas 价格: {} Gwei", gas_price_in_gwei);
    println!("标准转账 Gas 限额: {}", gas_limit);
    println!("预估转账 Gas 费用: {} ETH", gas_fee_in_eth);
    println!("计算逻辑说明: Gas 费 = 实时价格 ({}) * 标准限额 ({})", gas_price, gas_limit);
    println!("---------------------------------------");

    Ok(())
}