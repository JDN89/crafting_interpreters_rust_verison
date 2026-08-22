# Measuring Performance

## Commands to measure performance

keep copy of binaries to measure performance improvements over time.

```bash
cp target/release/rlox rlox-v1 # and so forth
```

`perf tool linux`

```bash
cargo build --release

perf record -F 997 --call-graph fp -g -o rlox.perf.data target/release/rlox fib.lox

perf report --hierarchy --demangle -i rlox.perf.data
```

`hyperfine`

```bash
hyperfine --warmup 3 --export-json baseline.json 'target/release/rlox fib.lox'

# or export to markdown
hyperfine --warmup 3 --export-markdown pref_results.md 'target/release/rlox fib.lox'
```

## Results

### baseline -- slow version

```bash
crafting_interpreters_rust_verison/rlox master  ? ❯ hyperfine --warmup 3 --export-markdown pref_results.md 'target/release/rlox fib.lox'
Benchmark 1: target/release/rlox fib.lox
  Time (mean ± σ):      6.069 s ±  0.062 s    [User: 6.050 s, System: 0.001 s]
  Range (min … max):    6.019 s …  6.232 s    10 runs
```
