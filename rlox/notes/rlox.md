# Improving perfomance

## Low hanging fruit, getting rid of .clone() when we add new values to our local env
TODO: not sure anymore but we store local variables, parameters and args of functions, funciton names we call in our local environment, to protect the scope. What I saw in the perf report was allready that a lot of time was spend in define, which I found weird because here we store only values in a hashmap which I though would be a fast operation? 
The first thing I noticed was that I call . clone on a value that we pass to the funciton and we allready own. I call .clone() at the call site and then inside the function body I call .clone() again. Which means that I make a deep copy of the function and store it on the heap twice. Once is allready arguably wrong (fix explained later), but twice is definitly not okay.
.clone would have been okay if we passed a reference `&`, but not here where we allready own the value.

From the docs: 
> The Clone trait allows you to explicitly create a deep copy of a value, and the duplication process might involve running arbitrary code and copying heap data.

TODO this reddit thread seems informative with extra info of what I did wrong or interpreterd wrong
> https://www.reddit.com/r/rust/comments/1hx9nwm/why_should_i_not_use_clone_and_which_alternatives/


## TODO:
- [ ] store depth and slot in ast node u32, and u32 -> probably have to do the same for vec of ast nodes..., or somehting similar
- [ ] there is an iterator where we can probably index into the src string. We have self.current. pass src as vec utf8? -> that iterator that is slow in (the lexer?) can be tested with criterion I think?
- [ ] make base from clippy refactor
	- [ ] andrew kelly says math is faster then accessing standard memory look up video
- [ ] Nick barker gave example of binary tree with array
- [ ] code with neovim again?

after this switch to vec instead of hasmap, but first swithc to faster hashing and look up why standard hashing is slowhh

### ran cargo clippy
Nice. Detects unnecessary clones. What can be const,... Lots of Self missing

### Base version rlox
**Hyperfine:**
```bash
crafting_interpreters_rust_verison/rlox master ? ❯ hyperfine --warmup 3 'target/release/rlox fib.lox'
Benchmark 1: target/release/rlox fib.lox
  Time (mean ± σ):      5.845 s ±  0.040 s    [User: 5.827 s, System: 0.001 s]
  Range (min … max):    5.816 s …  5.913 s    10 runs
```

**Perf report**
```text

```

### Profiling results

```bash
crafting_interpreters_rust_verison/rlox master ? ❯ perf record -F 997 --call-graph fp -g -o rlox.perf.data target/release/rlox fib.lox
9227465
[ perf record: Woken up 22 times to write data ]
[ perf record: Captured and wrote 5.723 MB rlox.perf.data (5694 samples) ]

crafting_interpreters_rust_verison/rlox master  ? ❯ perf report --demangle -i rlox.perf.data
```

#### cleaned up perf report results
Perf report shows that a lot of time is spend inside the environment hashmap

```rust
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Env>,
}
```

The profile makes the bottleneck fairly apparent: **a significant amount of time is being spent hashing strings and performing hash map lookups and insertions.**

In particular, `define` eventually spends most of its time inside `insert`, while `get_at` is dominated by hash map lookups. 

### Switching to a faster hashing algorithm
**TODO** why is FxHashMap faster? Give some info or links

[Adding FxHasmap](https://docs.rs/fxhash/latest/fxhash/type.FxHashMap.html)
```bash
crafting_interpreters_rust_verison/rlox master  ? ❯ cargo add rustc-hash
```

```rust
#[derive(Debug, Clone)]
pub struct Environment {
    values: FxHashMap<String, LoxValue>,
    enclosing: Option<Env>,
}
```

```rust
    pub fn new() -> Env {
        Rc::new(RefCell::new(Self {
            values: FxHashMap::default(),
            enclosing: None,
        }))
    }

```

```rust
 pub fn new_enclosed(enclosing: Env) -> Env {
        Rc::new(RefCell::new(Self {
            values: FxHashMap::default(),
            enclosing: Some(enclosing),
        }))
    }
```

#### hyperfine results
```bash
crafting_interpreters_rust_verison/rlox master  ? ❯ hyperfine --warmup 3 'target/release/rlox fib.lox'
Benchmark 1: target/release/rlox fib.lox
  Time (mean ± σ):      4.432 s ±  0.037 s    [User: 4.418 s, System: 0.001 s]
  Range (min … max):    4.360 s …  4.490 s    10 runs
```

#### perf report results
### Before: `HashMap` with the default hasher

```text
14.33%  hash_one<RandomState, &str>
 ├─ 8.49%  finish
 ├─ 4.54%  hash<str, DefaultHasher>
 └─ 1.00%  build_hasher

13.26%  get_at
 └─ 10.74%  HashMap::get
     └─ 10.42%  find

12.57%  define
 └─ 11.78%  drop_glue<Option<LoxValue>>
     └─ 11.55%  HashMap::insert
         ├─ 7.15%  find_or_find_insert_index
         ├─ 2.54%  insert_at_index
         └─ 1.11%  hash_one

11.94%  HashMap::insert
 ├─ 7.15%  find_or_find_insert_index
 ├─ 2.54%  insert_at_index
 └─ 1.11%  hash_one

10.74%  HashMap::get
 └─ 10.42%  find
```

### After: `FxHashMap`

```text
11.28%  get_at
 ├─ 6.57%  cloned<LoxValue>
 │   └─ 4.91%  HashMap::get
 │       ├─ 2.85%  find
 │       └─ 1.39%  make_hash
 └─ 1.74%  ancestor

11.03%  define
 └─ 9.79%  drop_glue<Option<LoxValue>>
     └─ 9.57%  HashMap::insert
         └─ 7.54%  find_or_find_insert_index
             └─ 6.71%  find_or_find_insert_index_inner

7.22%  HashMap::get
 ├─ 3.92%  find
 └─ 1.84%  make_hash
```

### Change dyn function calls with enums
I saw a video where [Casey muratory says why clean code is slow](https://www.youtube.com/watch?v=8xBJPa_480Q&t=1632s) 'The cost of a compiler not being able to do any optimizations'. what would happen with



``` text
13.37%  LoxFunction::call
 └─ 4.87%  evaluate_expression
     └─ 4.56%  call
```

```rust
pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue>;
}

```rust
#[derive(Clone)]
pub enum LoxValue {
    Str(String),
    Boolean(bool),
    Float(f64),
    Nil,
    // TODO: replace dyn trait later with enum variant. more explicit
    Callable(Rc<dyn LoxCallable>),
}
```



### changing env hashmap to Vec

We store the reference in the hashmap, which is alway a name, plus the correspondin LoxValue, which can be a literal value, class, function,...
The solution is to drop the mame and each time we encounter a reference with correosonding calue, fhat we would store in the Env hash as key vake, then we just now add the depth and slot to the expression node. So when we encounter it during the interoreter face we point it to the right env location via indexing.



We spend a lot of time adding function names, calls, args parameters to the environment, during the execution of the fibonacci sequence. A first optimization I can try is trying to find a faster hash function. 

A second optimization is instead of using a hashmap I can use a Vec<Vec< Env >>. At the moment we use the lexeme as a key, to find the name and value in the hashmap. Seeing that I allready store the depth of where I can find an ast node in the ENV, I can also add it's slot. At the moment only the depth of the assign expr gets stored during the resolving face, because of lexical scoping we have to know to which value a usage to a variable points to.  We store Loxvalues in the AST so now during parsing we will also have to add depth and slot (both u:32) to LoxValue, so we know where in the ENV we have to retrieve the value.
We store  Loxvalues in the ENV during interperting and resolving face. An issue I have is that during the resolving face I am not sure if I allready have access or have transformed the Literel (ast node representaiton of LoxValue) and Loxvuale, which is the evaluated LoxValue.  
The leaves of an AST node will always be a litereal, exists always of a literal value. Maybe I am wrong with this. Can a leave be a funciton or something else besides literal?
anyway in the ENV we store LoxCallable, AssignExpr, VarExpre and Expr::Litereal. In al these we will have to add scope and depth.


                // var a = 1-> ath this moment define in environment.
                // becasue these are the values we want to retrieve that are bound to a scope -> var a = global { var a = local; print a;} print a;
                // SAME for Function names and arguments, because a global function may be used in an inner scope if there is no function defined in the inner scope

SOLUTION
I think that the first step is to replace the environment hasmap with a VEC, for this during resolving I have to keep replicate the env of the interpreter vec, so or keep count, like okay for each assing, var, funciton,... we have to probably store it in the env at this depth, but also a this slot, meaning that we have to keep a slot tracker around that we reset with each level change -> each time we go a level deeper.
I was thinking of the wrong hashmap, It's probalby not worth changing the resolver hashmap, because that one goes over the AST onece. whilest the runtime environemnt gets accessed over and over again, especially during recursion. so I just have to figure out what we store in the env at what point and then store in the Expr node, this will get stored in this level of the ENV at this slot!!

count access and print of both hashmaps to see how much it is accessed. Can i see it from the other thing as well? the one that gives a call stack, perf record and pref REPORT.

do criterion here around the hasmpa define and get functions!!

SOLUTION
**I think that the first step is to replace the environment hasmap with a VEC, for this during resolving I have to keep replicate the env of the interpreter vec, so or keep count, like okay for each assing, var, funciton,... we have to probably store it in the env at this depth, but also a this slot, meaning that we have to keep a slot tracker around that we reset with each level change -> each time we go a level deeper.**

WE STORE only refs etc int he env not LITERALS, but a ref is a loxvalue, like name of an arg, funcitn,... it's named LoxValue in my env but it will never be just a number, or a boolea,... But we store LoxVlaue maybe it's the reference. Thats what tripped me up, we don't store LoxValues, well acutally yes but we store reference which are stirng (enum variant of Loxvalue)!!!!

DROP NAME FROM hasmap and keep the value and for the value you store the slot. so each time you encounter a name, ref during the resolver, you store the depth and slot fo the loxvalue, even name it loxvalue_env_depth loxvalue_env_slot

```
And for a function call:

```
foo(123);
```

the call environment might contain:

```
a → LoxValue::Number(123)
```

So **the argument binding `a` does not store a "reference" to the argument**. It stores the actual runtime value `123`.

Likewise, `foo` doesn't store a string reference to `"foo"` as its value. It stores the actual callable:

```
foo → LoxValue::Callable(...)
```
```

I was thinking of the wrong hashmap, It's probalby not worth changing the resolver hashmap, because that one goes over the AST onece. whilest the runtime environemnt gets accessed over and over again, especially during recursion.

count access and print of both hashmaps to see how much it is accessed. Can i see it from the other thing as well? the one that gives a call stack, perf record and pref REPORT.

**DISTINCTION only for references we need to store the slot and scope**
We loop over  ast nodes:
in 

do criterion here around the hasmpa define and get functions!!


- globals  LoxFucntions -> func

Like that I can transform
```rust
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Env>,
}
```

to 

```rust
#[derive(Debug, Clone)]
pub struct Environment {
    values: Vec<LoxValue>
    enclosing: Option<Env>,
}
```

### Not all CPU operations are equal
![[Pasted image 20260827220447.png]]

### References

[Intro to Data Oriented Design for Games](https://www.youtube.com/watch?v=WwkuAqObplU)
[Andrew Kelley: A Practical Guide to Applying Data Oriented Design (DOD)](https://www.youtube.com/watch?v=IroPQ150F6c&t=11s)
[clippy video](https://www.youtube.com/watch?v=IroPQ150F6c&t=11s)
