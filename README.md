## How to Use

### Testing

```
cargo test
```

### Backtesting

```
cargo run --features=backtest
```

- Loads backtesting data either from the API of from the local cache.
- Stops execution in no more data is left.
- Plots the performance of the strategy the end of execution.

### Live Trading

```
cargo run --release
```