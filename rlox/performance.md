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