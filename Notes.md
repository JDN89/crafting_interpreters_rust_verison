# TODO

- [ ] Token bevat Object literal; geen idee waarvoor dit zal dienen, maar bij mij is dit voorlopig een literal: string

# Nice to have

- [ ] decend error reporter. See chapter 4.1.1

# Notes

Een **lexeme** is gewoon een groep karakters uit de source code die in de context van de taal iets betekenen, maar op zichzelf is het maar een stukje tekst.

Bijvoorbeeld:

var language = "lox";

De substring "language" op zich zegt niet veel; het is gewoon een naam. Pas in context van var language = ... krijgt het betekenis.

De **scanner** loopt de hele source code door en geeft de lexemes extra info zodat er tokens ontstaan. Een **token** is dus een lexeme plus metadata: het type, eventueel de waarde (voor literals), en waar het staat.

Daarna kan de **parser** die tokens gebruiken om de structuur van het programma te herkennen en zo een **AST** opbouwen.

Als je alles in één keer zou doen, moest de **parser** constant zelf gaan checken: oké, dit is een var, daarna een identifier, dan een =, dan een literal… Dat wordt heel snel complex. Door eerst te scannen en tokens te maken, kan de parser zich gewoon richten op de volgorde en structuur, zonder per karakter te hoeven nadenken.

## Chapter 4 Scanning

### 30-01-2026

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
