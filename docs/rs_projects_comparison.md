# quantaxis-rs 项目分析报告

**日期**: 2026-05-09
**分析人**: Claude

---

## 1. 项目定位对比

| 项目 | 定位 | 成熟度 |
|------|------|--------|
| **quantaxis-rs** | 量化回测引擎 | v0.3.4 |
| **qaexchange-rs** | 完整交易所系统 | v0.1.0 (功能完整) |

### quantaxis-rs 特点
- 独立运行，无需外部依赖
- 简单 CSV 输出
- 单账户/单进程
- 适合策略回测

### qaexchange-rs 特点
- 微服务架构（多进程）
- 依赖 qars2 核心库
- 高并发（万级账户）
- 完整交易所功能

---

## 2. 依赖关系

```
qaexchange-rs
    │
    └── qars2 (本地路径) ──→ qars (核心库)
            │
            ├── qaaccount   ←── quantaxis-rs 也有此模块
            ├── qaorder    ←── quantaxis-rs 也有此模块
            ├── qaposition  ←── quantaxis-rs 也有此模块
            └── qaprotocol  (QIFI 协议)

quantaxis-rs 独立包含上述模块（复制）
```

**结论**: qars 是被两个项目共享的核心库，quantaxis-rs 可能从 qars 分离出来。

---

## 3. 代码模块对比

### quantaxis-rs 模块结构
```
src/
├── qaaccount.rs      # 账户管理 (~1458 行)
├── qaposition.rs     # 持仓管理 (~632 行)
├── qaorder.rs        # 订单结构
├── qaperformance.rs   # 性能分析
├── market_preset.rs  # 合约规则配置
├── trade_date.rs      # 交易日历 (~93KB)
├── indicators/        # 技术指标 (21个)
│   ├── sma.rs
│   ├── ema.rs
│   ├── rsi.rs
│   └── ...
└── pybindings.rs     # Python 绑定 (新增)
```

### qaexchange-rs 模块结构
```
src/
├── core/             # 重导出 qars 核心类型
│   ├── account_ext.rs
│   └── order_ext.rs
├── account/          # 账户系统（独立进程）
│   └── core/         # 异步账户更新
├── matching/         # 撮合引擎
│   ├── engine.rs     # Orderbook 封装
│   └── auction.rs    # 集合竞价
├── market/          # 市场数据
├── storage/         # WAL + MemTable + SSTable
├── replication/     # 主从复制
├── query/           # Polars 查询引擎
├── service/        # 业务服务
├── exchange/       # 交易所服务
└── user/           # 用户管理
```

---

## 4. 核心类型对比

### quantaxis-rs 中的 QA_Account
```rust
// qaaccount.rs
pub struct QA_Account {
    pub money: f64,
    pub hold: HashMap<String, QA_Postions>,
    pub accounts: account,
    pub dailyorders: BTreeMap<String, Order>,
    pub dailytrades: BTreeMap<String, Trade>,
    // ...
}
```

### qaexchange-rs 使用的 QA_Account
```rust
// core/mod.rs
pub use qars::qaaccount::account::QA_Account;
pub use qars::qaaccount::position::QA_Position;
pub use qars::qaprotocol::qifi::account::{Account, Position, Trade, QIFI};
```

---

## 5. 关键技术对比

| 技术 | quantaxis-rs | qaexchange-rs |
|------|---------------|----------------|
| 序列化 | serde_json | rkyv (零拷贝) |
| Web 框架 | 无 | actix-web |
| 数据库 | 无 | mongodb |
| 查询引擎 | 无 | polars |
| 存储 | CSV | WAL+MemTable+SSTable |
| 复制 | 无 | Raft 风格 |
| IPC | 无 | gRPC / iceoryx2 |
| 前端 | 无 | React (web/) |

---

## 6. 建议

### quantaxis-rs 的价值
1. **独立部署** - 无需 qars 依赖，可独立运行
2. **Python 集成** - 已添加 PyO3 绑定
3. **快速回测** - 轻量级回测场景

### 可能的整合方向
1. **抽取 qars 核心** - 将 quantaxis-rs 的核心模块重构为独立 crate
2. **统一架构** - qaexchange-rs 使用 qars + 服务层，quantaxis-rs 使用 qars 独立运行
3. **避免重复** - quantaxis-rs 的 account/position/order 应该来自 qars

### 当前问题
- qars2 作为 git submodule 未正确初始化
- quantaxis-rs 复用了 qars 中的类型定义
- 版本可能不同步

---

## 7. 结论

**qaexchange-rs 已经有完整的 qars 引擎**，quantaxis-rs 的功能是 qars 的子集。

- 如果需要**快速回测**，使用 quantaxis-rs
- 如果需要**生产级交易所**，使用 qaexchange-rs + qars

未来可以考虑让 quantaxis-rs 直接依赖 qars 而非复制代码。