# TODO

- [ ] fix stackoverflow error

```BASH
Running prompt
>   1 > 2
Parser error: : Expect ';' after value..
>  1 > 2 ;

thread 'main' (13773) has overflowed its stack
fatal runtime error: stack overflow, aborting
[1]    13773 IOT instruction (core dumped)  cargo run
```

- [ ] run test er push in github?

## Nice to have

- [ ] finish statement chapter and fix integration tests
- [ ] decend error reporter. See chapter 4.1.1
- [ ] Arana alloc
  - [ ] push Expr on the arena and store the index of the location in the arena in the AST
  - [ ] for subslices just store the start and end of the ss, and when you actually need the slice, index into the soruce code...
- [ ] reduce memory footprint Tokens
  - [ ] research memory profiling tools in Rust. What is there?
- [ ] Struct of Arrays instead of Array of struct

## Notes

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
