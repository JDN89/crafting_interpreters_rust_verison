# Learned rust

With anyhow crate you can create custom errors on the fly. See error helper function in the code. Downside, you don't have any strongly typed errors and seeing that your error is not defined in an enum, you also can't do any pattern matchin on the error

```Rust
#[derive(Debug)]
enum MyCustomError {
    Marco,
    Polo,
}

// Example of pattern matching on a strongly typed error
fn do_something(e: MyCustomError) {
    match e {
        MyCustomError::Marco => println!("Doing Marco thing!"),
        MyCustomError::Polo => println!("Doing Polo thing!"),
    }
}
```

### ? returns early on error

if you want to execute the rest of the function, don't add ? 
but store in 
``` Rust
let result = do_somehting();
bla bla
result.map|() -> |

```
