# Repro case for AM perf bug

Run with:

```
cargo run --release
```

The behaviour I'm seeing on my M1 macbook pro:

```
    Finished release [optimized] target(s) in 0.26s
     Running `target/release/am-repro`
Loading C1_fast.am into automerge (82920 bytes)
Automerge took 134 ms to load
Getting text content...
Text document 21362 bytes long

Loading C1_slow.am into automerge (1524538 bytes)
Automerge took 158872 ms to load
Getting text content...
Text document 534050 bytes long
```