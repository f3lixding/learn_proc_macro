## Premise
I want to understand everything there is to know about macros. 

In general, in order for texts to be parsed into useful source code, there are the following components involve that make it happen (in the order of composition):
- **token**: They are the smallest unit of rust source code:
	1. **Keywords**: Reserved words like `fn`, `let`, `struct`, `enum`, etc.
	2. **Identifiers**: Names of variables, functions, structs, etc.
	3. **Literals**: Numeric, string, character, and boolean constants.
	4. **Operators**: Symbols like `+`, `-`, `*`,.
	5. **Punctuation**: Commas, semicolons, braces, brackets, etc.
 - **token stream**: As the name suggests, a stream of tokens
 - **TokenTree**: This is a `proc_macro2` specific construct. This is to group tokens that are delimited by either {} or (). 
 - **AST**: Abstract Syntax Tree. These are made up of tokens. They are usually delimited by {} or ().

During a compilation, here is roughly what happens in order for a proc macro to produce useable code:
1. **Input as Token Stream**: When you write a procedural macro, Rust provides the macro with a stream of tokens (a `TokenStream`) that represents the code to which the macro is applied. This could be the contents of a function, struct, or any other code snippet depending on the type of macro.
2. **Parsing Token Stream**: The macro uses the `syn` crate to parse these tokens into a more structured format, often an AST. This structured format lets you navigate and understand the code's syntax easily.
3. **Manipulating the AST**: Once you have the AST, your macro code can analyze, modify, or augment it. You might change function bodies, add or alter annotations, generate new code, etc. This is where the real power of procedural macros lies - they can dynamically alter or generate Rust code based on complex logic.
4. **Generating Code**: After manipulating the AST, the macro uses the `quote` crate to turn the altered or newly generated AST back into a `TokenStream`. This `TokenStream` is then handed back to the Rust compiler.
5. **Compilation**: The Rust compiler takes this generated token stream and compiles it as if it were regular Rust code written by a developer.
## Resources
[Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop#attribute-macro-sorted).
[Seq implemented (i.e. function like proc macro)](https://github.com/dtolnay/seq-macro)

## Some important crates that you will need
### Syn
https://crates.io/crates/syn
This crate makes parsing streams of rust tokens into syntax tree.

#### parse_macro_input
This is a dec macro used to make parsing a little easier: https://rcos.io/static/internal_docs/syn/macro.parse_macro_input.html

#### ParseStream
https://docs.rs/syn/2.0.39/syn/parse/index.html
This is the input to `parse` in `Parse` trait. 
What's important about this is that the `parse` method besides producing the token, also _advances_ the pointer to the next token. 

### Quote
https://crates.io/crates/quote
This crate turns `AST` back into `tokenstream` and hands it off back to the compiler to make sense of it.

### Proc-macro2
https://crates.io/crates/proc-macro2
A wrapper around the procedural macro API of the _compiler's_ `proc_macro` crate. 
Its main purposes are:
- For some reason types in `proc_macro` crates are accessible only to within the crate and compiler and cannot exist in code outside of a proc macro. This crate allows access to these types outside in a normal `main.rs` or `lib.rs`.
- This also makes proc macros written with this crate unit testable.
## Declarative Macros
**Fragment Types**:
- These are "types" of items that get passed into macros in general (both proc and dec macros). These are also referred to as _metavariables_.
- A full list of fragment types can be found here: https://doc.rust-lang.org/reference/macros-by-example.html.


## Proc Macro
**First thing first**:
Procedural macros (often abbreviated as proc macros) and declarative macros (defined with `macro_rules!`) serve different purposes in Rust, each with its strengths and limitations. Generally, one might prefer procedural macros when:
1. **Complex Input Manipulation**: Procedural macros allow for more sophisticated parsing, manipulation, and validation of the input tokens compared to declarative macros.
2. **Code Generation from External Data**: If you're generating Rust code based on external data, such as a configuration file or a database schema, procedural macros are more suited.
3. **Custom Derive**: One of the primary uses of procedural macros is creating custom derive implementations. For instance, the `serde` crate allows for custom serialization and deserialization of Rust structs and enums using `#[derive(Serialize, Deserialize)]`, implemented as procedural macros.
4. **Attribute-Based Macros**: Procedural macros allow for custom attributes (e.g., `#[your_attribute_here]`). This is useful for aspect-oriented programming-like tasks where you might want to annotate functions or types with special behavior.
5. **Generating Implementation Based on Type Introspection**: Procedural macros can introspect the types they're applied to, making decisions based on this introspection. This is not possible with `macro_rules!`.
6. **DSL Implementation**: If you're implementing domain-specific languages (DSLs) within Rust, procedural macros provide the power and flexibility you might need.
