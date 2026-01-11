<img width="1353" height="659" alt="Meta Mask和测试币领取截图" src="https://github.com/user-attachments/assets/30b856ea-e29a-45c6-89b8-73562bd3bd28" />

## Task-2: 查询 Arbitrum 测试网地址余额

### 1. 代码路径
`/Task2-balance-query/src/balance.rs`

### 2. 运行结果截图
![运行结果](./result.png)
## Task-3: 计算 Arbitrum 转账 Gas 费用

### 1. 代码路径
`/Task3-gas-estimator/src/gas_estimator.rs`

### 2. 计算逻辑说明
本关卡通过 `ethers-rs` 库动态获取实时 Gas 价格。
计算公式：`Gas 费用 = 实时 Gas 价格 (get_gas_price) * 标准转账 Gas 限额 (21000)`。

### 3. 运行结果截图
![Task3结果](./task3_result.png)
