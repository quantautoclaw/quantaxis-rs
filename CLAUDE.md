# quantaxis-rs

High-performance quantitative trading backtest engine written in Rust.

## Python Extension

Build and install:
```bash
cd /Users/james/code/quantaxis-rs
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop
```

Usage:
```python
from quantaxis_rs import AccountWrapper

acc = AccountWrapper('test', 'test', 'admin', 1000000.0, 'backtest')
acc.init_h('RB2005')
acc.buy_open('RB2005', 10.0, '2020-01-20', 3500.0)
acc.on_price_change('RB2005', 3520.0, '2020-01-21 10:00:00')
print(acc.get_floatprofit())
```

## Rust Development

Build: `cargo build`
Test: `cargo test`
Release: `cargo build --release`

Python feature: `cargo build --features python`