# TODO

- [ ] continue with 11.3.3 page 195 -> store in AST. Storing in seperate vec is not necessary for us. We are not saving pages and ink :p
- [ ] chapter 11. Proably should store a slice instead of String? lexemes once declared won't change anymore I think? AST and tokens are read only. after finishing the interpreter. Look for ways to optimize this. storing slice, just start and end of the lexeme u32 start and u32 end, and in case you need to look up you just look up in the source code,... At the end so i can see the impact of .clone() in this case
- [ ] no longer return last expression. Deviates to much from crafting interpreters. Check todo in interpreter.rs
- [ ] look into: ';' is this indication of an expression statement. check lexer and parser
- [ ] resultaat van `cargo run debug.txt`
 >  "Hi, ""Dear"" ""Reader""!"
zoek uit waarom we dubbele quotes krijgen
- [ ] [return statements 10.5](https://craftinginterpreters.com/functions.html#return-statements)
- [ ] Herlees hoofdstuke 10 volledig en vat samen in learned of notes. Ik be bijna alles vergeten (maand pauze en moet alles eens opnieuw in kaart brengen):Volgens mij werkt het zo: we definiëren een functie met `Stmt::Function` en roepen die later aan met `Expr::Call`. De interpreter slaat de functie-definitie op als een `LoxValue::Callable` in de environment. Wanneer een call-expression wordt uitgevoerd, evalueert de interpreter eerst de callee, zoekt de bijbehorende callable in de environment op, en voert daarna `call(...)` uit.

## Nice to have

- [ ] finish statement chapter and fix integration tests
- [ ] decend error reporter. See chapter 4.1.1
- [ ] Arana alloc
  - [ ] push Expr on the arena and store the index of the location in the arena in the AST
  - [ ] for subslices just store the start and end of the ss, and when you actually need the slice, index into the soruce code...
- [ ] reduce memory footprint Tokens
  - [ ] research memory profiling tools in Rust. What is there?
- [ ] Struct of Arrays instead of Array of struct

## NEXT steps
- implement VM
- add types to the language -> evaluate after build ast -> generate bytecode -> types no longer necessary in op code omdat we op dit moment al weten wat de types zijn.
> Ik denk dat na het afwerken van deze interpreter het leuk zou zijn om direct een VM voor deze interpreter te implementeren. Hiervoor moet ik wel het tweede deel van het boek lezen en de benlangrijkste stukken adopteren, maar dat zal wel moeten lukken. Lees boek uit en begin dan aan VM. Als laatste kan ik dan types toevoegen. Ik denk dat als ik al dit gedaan heb wel de basis van interprters onder de knie zal hebben.

Dan chip 8 emulator
Daarna boek from zero to prod, maar in Axum en chat app maken (wat bijleren over websockets)
Dan een OS rust project. Waarschijnlijk zed omdat ik recent Zed ben beginnen gebruiken en zed is NIIIICE. Zeker met die potatoe laptops die we nu gebruiken en intelij niet kunnen draaien.
Dan even yolo tijd. Ben teveel aan het programmeren.

## Notes

## chapter 10 functions

### 06-07-2026
Ik denk dat het moeten uitschrijven van een flow chart erop wijst dat ik mij voor dit laatste hoofdstuk teveel op implementeren van de afzonderlijke sub hoofdstukken heb gefocust zonder dat ik het hele plaatje al in mijn hoof had. Beter om hier wat tijd voor te nemen. Anders ga ik weer tegen de bug van 2024 aanlopen. Ik ben hier nu wat tijd voor aan het nemen

Lox function flow chart:

#### Declaration part
In Lox a file gets executed from top to bottom and that's why we need the function declaration before function call. 
- encounter `Stmt::Function`
- create `LoxFunction`
- store the function name in the current environment
- later, when `Expr::Call` evaluates its `callee`, it looks up the name through `Expr::Variable`

#### call part
- `Expr::Call`
  - evaluate `callee`
    - if it is `Expr::Variable`, retrieve the `LoxFunction` from the environment. We now have a `LoxValue::Callable(Rc<dyn LoxCallable>)` and use the pointer to the function to check `function arity()` and invoke `function.call(self,args)`
    - check that it is callable
  - evaluate arguments
  - call `LoxFunction::call(args)`
    - create a new enclosed environment
    - bind parameters to arguments in the new environment
    - execute the function body with `execute_block()`
      - if a `return` is hit, it bubbles up as `ExecSignal::Return(value)`
      - otherwise it returns `ExecSignal::Normal`
    - restore the previous environment
    - return the final `LoxValue`
    - value can then be used to bind to a var, print,...

### 04-07-2026 chapter 10.5.1 returning from calls
Ben verbaasd dat er exceptions gebruikt worden voor controlflow.
wanneer we een return statement tegenkomen moeten we de stack unwinden tot het punt waar we call() roepen. Een return statement kunnen we tegenkomen in een IfStatement, WhileStatement en BlockStatement. voor all deze statements moeten we kunnen returnen naar call().
Rust heeft niet het concept van Exceptions en try catch. Er wijn 2 errors: recoverable Result and unrecoverable panic! Ik ga een custom result type moeten definieren Enum met Normal val en Return val (die de Loxvalue) die we returnen wrapped. Normal val staat gelijk aan nil. Kan dit dan aftoetsen in de call(function block) welk Result enum type we returnen -- normal of return

Flow diagram parser -> interpreter voor functions (blijf de flow vergeten, indien er teveel weken tussen zitten);

``` markdown

## Callable
fun add(a, b) {
  print a + b;
}

## Callee
add(1, 2);

        |
        v

# parser

## Callable
Stmt::Function
  name: "add"
  params: [a, b]
  body: [ print a + b; ]

## Callee
Stmt::ExpressionStmt
  expr: Expr::Call
    callee: Expr::Variable("add")
    arguments: [1, 2]

        |
        v

# INTERPRETER

interpreter executes function **declaration** (callable)

At runtime we create a LoxValue::Callable which we store in the environment so we can fetch it when we encounter the **callee**. This also mean that we have to declare our functions in chornological order Callable -> Callee. When we encounter the callee, we store the args in an enclosed environment so that they don't collide with the existing args in the parent environment.

environment
  "add" -> LoxValue::Callable(LoxFunction { declaration: Stmt::Function(...) })

        |
        v

interpreter evaluates call expression

Expr::Call
  callee -> Expr::Variable("add")
             -> environment.get("add")
             -> LoxFunction
  args   -> [1, 2]

        |
        v

LoxFunction::call(self, args)

  create **new enclosed environment**
        |
        +-- a = 1
        +-- b = 2

        |
        v

execute_block(body, new_env)

  runs:
    print a + b;
Short version:
Stmt::Function  -> stores callable in environment
Expr::Variable   -> looks it up by name
Expr::Call       -> invokes it
```

Ik denk dat na het afwerken van deze interpreter het leuk zou zijn om direct een VM voor deze interpreter te implementeren. Hiervoor moet ik wel het tweede deel van het boek lezen en de benlangrijkste stukken adopteren, maar dat zal wel moeten lukken. Lees boek uit en begin dan aan VM.
Als laatste kan ik dan types toevoegen. Ik denk dat als ik al dit gedaan heb wel de basis van interprters onder de knie zal hebben.

### 03-07-2026 REMOVE LOXVALUE AND changed way I test my interpreter via an integration test.

removed the `LoxValue` return path from the interpreter. Test behavior through 'print' and capturing and comparing print output instead. I think this is how most interpreters test and the previous added integration test and 'LoxValue' was adding unnecessary complexity. Plus the flow was deviating to much from the book and I was starting to get confused.

### rambling

Laatste maanden weinig tijd en ik merk dat ik veel zaken van dit hoofdstuk vergeten ben. Best om nog eens door de code te klikken alles te herlezen eens hoofdtuk 10 af is en dan eens samen te vallen. Het is ook op dit punt in 2024 dat ik de mist was ingegaan. Hopelijk vind ik eens een paar uur tijd om het bonvestaande in 1 keer te doen. Vrije tijd vinden als vader is niet altijd even evident. Ik overweeg om 's morgens 30 min vroeg op te staan en telkens even te coden zodat ik dit deel van het boek eindelijk kan afwerken.

## chapter 9 Control flow

### 24-05-2026

Heat wave makes the office to warm.
Was going over the code and found that I had arrived at [For Loops: 9.5](https://craftinginterpreters.com/control-flow.html#for-loops).

I am liking Zed but I think I still prefer Neovim, but I think I'll keep using Zed for the moment until multicursors have arrived at neovim. Plus my main goals this and the following year is getting down the basiscs of Rust and contributing to an opensource project which would be Zed (written in Rust). Some random ramblings unrelated to this repo :p

### 20-05-2026

Started back on crafting interpreters.
I thought i had more tests????
reverted removing RefCell<Environment> will refactor to Vec or allocator later

### 13-03-2026

Don't know why but I suddenly stopped passing by ref '&' and starting using .clone()? I caught myself doing it during the implementation of 'and' and 'or' logical operators and I'm wondering if I did it also the past weeks?

repalce vec![] in parser with &[] slices. vec allocates on the heap and the slice on the stack, so faster. No need for this to live on the heap. We only use it in 2 functions, and doesn't need to outlive those functions.

## Chapter 8 Statements

hen a local variable has the same name as a variable in an enclosing scope, it shadows the outer one. Code inside the block can\E2\80\99t see it any more\E2\80\94it is hidden in the \E2\80\9Cshadow\E2\80\9D cast by the inner one\E2\80\94but it\E2\80\99s still there.

## Chapter 7 Evaluating expressions

I started using typed errors for the interpreting fase and I am using Anyhow general errors for the parser fase. This has become a mess. Use one or the other but not both styles, Seeing that this is just practice. Use Anyhow and add context where needed.

```rust
.context("bla bla {}")?
```

Ik was de Literal van de ast aan het hergebruiken voor het wrappen van de waarde van de gevaleerde expressie. Dit zou vroeg of later problemen gegeven hebben door de tight coupling. Indien ik de Ast node, of de runtime value zou moeten extenden zou ik vroeg of laat problemen gehad hebben.

## parser?

Een **lexeme** is gewoon een groep karakters uit de source code die in de context van de taal iets betekenen, maar op zichzelf is het maar een stukje tekst.

Bijvoorbeeld:

var language = "lox";

De substring "language" op zich zegt niet veel; het is gewoon een naam. Pas in context van var language = ... krijgt het betekenis.

De **scanner** loopt de hele source code door en geeft de lexemes extra info zodat er tokens ontstaan. Een **token** is dus een lexeme plus metadata: het type, eventueel de waarde (voor literals), en waar het staat.

Daarna kan de **parser** die tokens gebruiken om de structuur van het programma te herkennen en zo een **AST** opbouwen.

Als je alles in één keer zou doen, moest de **parser** constant zelf gaan checken: oké, dit is een var, daarna een identifier, dan een =, dan een literal… Dat wordt heel snel complex. Door eerst te scannen en tokens te maken, kan de parser zich gewoon richten op de volgorde en structuur, zonder per karakter te hoeven nadenken.

## Chapter 5 Representing code

### 11-02-2026

Echt niet blij met de hoeveelheid data dat momenteel in de token en AST zit. Zeker the Literal die ik zowel in de token als AST heb. Ik denk dat ik het best uit de token haal en de Literal wel in de AST behoudt. parse gewoon de lexeme om de correcte value te bekomen.

xxxx

I am considering immediatley implementing the allocator to store the expresions instead of using Box<Expr>.
For example Binary would become: {left: exprId, operator:..., right: ExprId}. This way I don't have to mess with Box, the expr
won't be spread out in memory,... But I want to see the memory and performance gains, so I'll do it the naive way first.

**Recursive descent parsing** we go from lowest precendece to higherst precendece and each grammar rule (precendece) is implemented as a function. higher-precendece gets handled by functions farther down the stack.

Ik moet de AST structuur and how right-associative `a = b = c` en **left-associative** (5-3)-1 correct worden afgehandled door recursive descent parser nog wat laten inzinken.

**Ripped out** the **lifetimes** and references to the source code. Ik gebruikte dit voor de lexemes en waarom... Nu geef ik enkel de source code door als reference en de andere zaken clone ik voorlopig (to_string()). Waarom? Ik wil niet de hele tijd die lifetimes doorheen mijn code meeslepen als ik later tijdens de performance upgrade toch gebruik ga maken van een **arena** en de lifetimes dan toch overbodig gaan worden. Dus waarom nu adden en dan weer verwijdern gewoon om de performance impact te zien. Ik weet toch al dat er winst gaat zijn. Dit wil zeggen dat ik source code ook op de arena ga smijten en dan voor de substrings iets ga doen a la `struct Span {start u32, end u32}`.

## Chapter 4 Scanning

### 01-02-2026

forgot difference tussen static en const.

Recent ontdekt dat je ook **Arena Allocators** kan gebruiken in Rust. Ik kende het concept al door Ginger Bill, en Ryan Fleury, maar ik had er nog niet aan gedacht om dit ook in Rust te gebruiken. Wat zou het verschil zijn met **RC** en **Refcell** wij managen het geheugen, en deoalocaten wanneer we willen. Gebruikt RC een malloc under te hood? Onderzoek dit in detail. [bumpalo arena impl in Rust](https://docs.rs/bumpalo/latest/bumpalo/).
De tokens, en AST zullen allemaal een soorgelijke lifetime hebben, dus hier lijkt een arena Alocator perfect, want ik kan ze ook allemaal tegelijkertijd deoalocaten. RC en Refcell brengen volgens mij buiten de allocatie per object veel overhead, omdat ze de pointers moeten tracken, plus indien allocatie per object, waar belanden ze in memory? Liggen ze vlak naast elkaar of verspreid, met cache misses als mogelijks gevolg.

### 31-01-2026

Wat misschien ook interessent is is om de optimalisatie video van Jon Gjengset te bekijken [Impl rust: One Billion Row Challange](https://www.youtube.com/watch?v=tCY7p6dVAGE) en dan mogelijks soortgelijke optimalisaties toepassen op de interpreter indien de optimalisaties van toepassing zijn. Dat plus de optimalisaties hieronder besproken zal een interessant challange zijn.

### 30-01-2026

> **TODO** bekijk of we toch echt zowel de lexeme als de literal nodig? Hebben we ze uberhoupt nodig? Kan ik niet gewoon hun positie in de source code meegeven en dan ze interpreteren (op basis van token type) of subslicen indien ik het relevante source code gedeelete nodig heb? Ik denk dat ik eens een talk van Zig had gezien of Data driven development waar Andrew Kelly sprak over het versnellen van de compiler en het reduceren van de token code. source: Andrew Kelley: A Practical Guide to Applying Data Oriented Design (DoD) https://www.youtube.com/watch?v=IroPQ150F6c
> Bekijk na implementeren van de interpreter en zie of ik het sneller kan maken!

> We store both the lexeme and the literal value in a token.
> The lexeme is the exact source-code representation of the token, while the literal is the parsed value represented by that lexeme.
> For example, for a number token with source text "123", the lexeme is "123" (a string slice), and the literal value is 123 (a number).
> This is useful during parsing and interpreting: we keep the lexeme for error messages and source context, and the literal value so the interpreter doesn’t need to re-parse the token at runtime.

Zonet besloten om &str te gebruiken voor de lexeme. De source code zal toch blijven bestaan totaan het einde van ons interpreter en indien we de lexeme moeten bezitten en aanpassen bestaan hier methodes voor. Momenteel is een string slice voldoende en efficienter. Wel irritant om overal lifetimes aan toe te voegen. Zeker als het later blijkt dat ik de slices niet nodig had, of een verkeerde keuze was.

### xx-01-2026

I am not going to implement the [error reporting](https://craftinginterpreters.com/scanning.html#error-handling) in . I can just as easy use anyhow with_context.

Ideally, we would have an actual abstraction, some kind of “ErrorReporter” interface that gets passed to the scanner and parser so that we can swap out different reporting strategies. For our simple interpreter here, I didn’t do that, but I did at least move the code for error reporting into a different class.
`Might be interesting to see if I can define a trait for this?`
I had exactly that when I first implemented jlox. I ended up tearing it out because it felt over-engineered for the minimal interpreter in this book.
