# The Miel Roadmap

## Where we are now

We are likely at before v0.1.0. Which is... **very** early.

## 0.1.0

- [ ] Basic Compiler Core
  - [x] Lexer
  - [x] Parser
  - [ ] MIR (codenamed Honey)
    - [ ] Code Generation
    - [ ] Optimization Passes
    - [ ] Semantic Analysis
  - [ ] Native Machine Code Generation
- [ ] Core Language Features
  - [x] Comments (`;; ...`, `;[ ... ]`)
  - [ ] Type System Core
    - [ ] Basic Primitive Types
      - [ ] Integers (`int`, `uint`, `i8`-`i64`, `u8`-`u64`)
        - [ ] Bitwise ops (`|`, `&`, `^`, `!`)
      - [ ] Floats (`float`, `f32`, `f64`)
      - [ ] Booleans
        - [ ] Logical ops (`or`, `and`, `xor`, `not`)
    - [ ] Basic Type Inference
  - [ ] Basic Compile-Time Evaluation for Simple Expressions
  - [ ] Mutation
    - [ ] `=` Reassignment
    - [ ] Augmented Operators
  - [ ] Functions
    - [x] `callable()` Type Syntax
    - [ ] Parameters and Return Values
    - [ ] Function Calls
    - [ ] Multiple Functions
  - [ ] Control Flow
    - [ ] If/else Conditionals
    - [ ] While Loops
    - [ ] Return
    - [ ] Break/Continue

## 0.2.0

- [ ] Box (`box T`)
- [ ] Pointer (`*T`)
  - [ ] Dereference (`*ptr`)
  - [ ] String Slices
- [ ] Iterators
  - [ ] Iterator Definition API (not decided on yet)
  - [ ] Ranges
  - [ ] For Loops

## 0.3.0

- [ ] Constants with Compile-time Evaluation

## 0.4.0

- [ ] Result (`T ? E`)
  - [ ] Error Propagation
  - [ ] Expect/Unwrap
  - [ ] Map Ok
  - [ ] Map Err

## 0.5.0
- [ ] Capability System
  - [ ] `Root` Capability
  - [ ] Custom Capability Definitions
  - [ ] `@Capability` Syntax
  - [ ] `acquire`/`release` Syntax

## 0.6.0

- [ ] Generics/Polymorphism

## 0.7.0

- [ ] Cross-Compilation
  - [ ] Target Triples (Windows/Linux/macOS)
  - [ ] `--target` Flag
- [ ] Structs and Enums
- [ ] Affine Types
  - [ ] Affine Protection
  - [ ] `#Affine` Directive
  - [ ] `@Capability` Syntax
  - [ ] Capability Checking Pass
  - [ ] References and `ro` References (read-only)

## 0.8.0

- [ ] Module System
  - [ ] Basic Imports
  - [ ] Circular Imports

## 0.9.0

- [ ] Concurrency Basics
  - [ ] `spawn` keyword (goroutine/fiber style)
  - [ ] Channels (`chan T` type)

## 0.10.0

- [ ] Project System (combs) and Build System
- [ ] Basic Standard Library
  - [ ] I/O (`std/io`)
    - [ ] `Print`/`PrintLn` Functions
    - [ ] `EPrint`/`EPrintLn` Functions
    - [ ] `ReadLine` Function
    - [ ] `IoWrite` Capability
    - [ ] `IoRead` Capability
  - [ ] Collections (`std/collections`)
    - [ ] `Vec` Type
    - [ ] `Map` Type
    - [ ] `Set` Type
  - [ ] Strings (`std/str`)
    - [ ] `String` Type (for growable strings)
    - [ ] `CString` Type (for growable, null-terminated strings)
  - [ ] Time (`std/time`)
    - [ ] `TimePoint` Type
    - [ ] `TimeFrame` Type
    - [ ] `TimeGet` Capability
  - [ ] File System (`std/fs`)
    - [ ] `File` Type
    - [ ] `ReadString`/`ReadBytes` Functions
    - [ ] `WriteString`/`WriteBytes` Functions
    - [ ] `Exists` Function
    - [ ] `Delete` Function
    - [ ] `FsRead` Capability
    - [ ] `FsWrite` Capability
    - [ ] `FsOpen` Capability
    - [ ] `FsDelete` Capability
  - [ ] Random (`std/rand`)
    - [ ] `RandInt`/`RandRange`
    - [ ] `RandomGen` Capability
  - [ ] Math (`std/math`)
    - [ ] Float Operations
    - [ ] Float Constants 
      - [ ] Euler's Number
      - [ ] Pi
      - [ ] Phi
      - [ ] Euler-Mascheroni Constant
      - [ ] Apéry's Constant
      - [ ] Pythagoras' Constant
      - [ ] Catalan's Constant

## 0.11.0-0.x.0

- Stabilizations and polish
