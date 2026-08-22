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

> hyperfine
```bash
crafting_interpreters_rust_verison/rlox master  ? ❯ hyperfine --warmup 3 --export-markdown pref_results.md 'target/release/rlox fib.lox'
Benchmark 1: target/release/rlox fib.lox
  Time (mean ± σ):      6.069 s ±  0.062 s    [User: 6.050 s, System: 0.001 s]
  Range (min … max):    6.019 s …  6.232 s    10 runs
```

> per
```text
-    0.00%   100.00%        rlox                                                                                     ▒
   +    0.00%     0.00%        ld-linux-x86-64.so.2                                                                  ▒
   +    0.00%     0.02%        [unknown]                                                                             ▒
   +    0.00%    94.73%        rlox                                                                                  ▒
   -    0.00%     5.25%        libc.so.6                                                                             ▒
           1.70%     1.70%        [.] cfree                                                                          ◆
           0.94%     0.94%        [.] malloc                                                                         ▒
           0.73%     0.73%        [.] 0x0000000000185deb                                                             ▒
           0.68%     0.68%        [.] 0x0000000000185dde   
```
```text
-    0.00%    94.73%        rlox                                                                                  ▒
    +   22.64%    22.64%        [.] <rlox::backend::interpreter::Interpreter>::evaluate                            ▒
    +    8.87%     8.87%        [.] <hashbrown::map::HashMap<alloc::string::String, rlox::backend::value::LoxValue,▒
    +    8.39%     8.39%        [.] <rlox::backend::loxfunction::LoxFunction as rlox::backend::callable::LoxCallabl▒
    +    7.39%     7.39%        [.] <std::hash::random::RandomState as core::hash::BuildHasher>::hash_one::<&str>  ▒
    +    7.21%     7.21%        [.] <core::hash::sip::Hasher<core::hash::sip::Sip13Rounds> as core::hash::Hasher>::▒
    +    6.84%     6.84%        [.] <rlox::backend::interpreter::Interpreter>::execute                             ▒
    +    5.41%     5.41%        [.] rlox::backend::interpreter::less_then_or_equal                                 ▒
    +    4.87%     4.87%        [.] rlox::backend::interpreter::subtraction                                        ▒
    +    4.30%     4.30%        [.] <rlox::backend::environment::Environment>::new_enclosed                        ▒
    +    3.36%     3.36%        [.] rlox::backend::interpreter::addition                                           ▒
    +    3.03%     3.03%        [.] <rlox::backend::environment::Environment>::get_at                              ▒
    +    2.06%     2.06%        [.] core::ptr::drop_glue::<core::iter::adapters::zip::Zip<alloc::vec::into_iter::In▒
    +    1.73%     1.73%        [.] <rlox::backend::environment::Environment>::get       
    ```
