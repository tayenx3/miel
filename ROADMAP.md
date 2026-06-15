# The Miel Roadmap

## Where we are now

We are likely at 0.0.1 Pre-Alpha. Which is... **very** early.

## 0.1.0 Alpha

- [ ] Basic Compiler Core
  - [x] Lexer
  - [x] Parser
  - [x] Semantic Analysis
  - [ ] IR
    - [ ] Code Generation
    - [ ] Optimization Passes
  - [ ] Native Machine Code Generation
- [ ] Core Language Features
  - [x] Comments (`;; ...`, `;[ ... ]`)
  - [ ] Type System Core
    - [ ] Basic Primitive Types
      - [x] Integers (`int`, `uint`, `i8`-`i64`, `u8`-`u64`)
      - [x] Floats (`float`, `f32`, `f64`)
      - [x] Booleans
      - [ ] String Slices
      - [ ] Pointer (`*T`)
      - [ ] Structs and Enums
    - [ ] Type Inference
  - [ ] Mutation
  - [ ] Memory Management
    - [ ] Stack Allocation (Variables)
    - [ ] `box T` Heap Allocation
    - [ ] Dereference (`*ptr`)
  - [ ] Functions
    - [x] `callable()` Type Syntax
    - [ ] Parameters and Return Values
    - [x] Function Calls
    - [ ] Multiple Functions
  - [ ] Control Flow
    - [x] If/else Conditionals
    - [x] While Loops
    - [ ] Return
    - [ ] Break/Continue

## 0.2.0 Alpha

- [ ] Box (`box T`)
- [ ] Iterators
  - [ ] Iterator Definition API (not decided on yet)
  - [ ] Ranges
  - [ ] For Loops

## 0.3.0 Alpha

- [ ] Constants with Compile-time Evaluation

## 0.4.0 Alpha

- [ ] Result (`T ! E`)
  - [ ] Error Propagation
  - [ ] Expect/Unwrap
  - [ ] Map Ok
  - [ ] Map Err
- [ ] Permission System
  - [ ] `Root` Permission
  - [ ] Custom Permission Definitions
  - [ ] `@Permission` Syntax
  - [ ] `acquire`/`release` Syntax

## 0.5.0 Alpha

- [ ] Affine Types
  - [ ] Affine Protection
  - [ ] `#affine` Directive
  - [ ] Permission Checking Pass

## 0.6.0 Alpha

- [ ] Generics/Polymorphism

## 0.7.0 Alpha

- [ ] Cross-Compilation
  - [ ] Target Triples (Windows/Linux/macOS)
  - [ ] `--target` Flag

## 0.8.0 Alpha

- [ ] Module System
  - [ ] Basic Imports
  - [ ] Circular Imports

## 0.9.0 Alpha

- [ ] Concurrency Basics
  - [ ] `spawn` keyword (goroutine/fiber style)
  - [ ] Channels (`chan T` type)

## 0.10.0 Alpha

- [ ] Project System (combs) and Build System
- [ ] Basic Standard Library
  - [ ] I/O (`std/io`)
    - [ ] `Print`/`PrintLn` Functions
    - [ ] `EPrint`/`EPrintLn` Functions
    - [ ] `ReadLine` Function
    - [ ] `IoWrite` Permission
    - [ ] `IoRead` Permission
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
    - [ ] `TimeGet` Permission
  - [ ] File System (`std/fs`)
    - [ ] `File` Type
    - [ ] `ReadString`/`ReadBytes` Functions
    - [ ] `WriteString`/`WriteBytes` Functions
    - [ ] `Exists` Function
    - [ ] `Delete` Function
    - [ ] `FsRead` Permission
    - [ ] `FsWrite` Permission
    - [ ] `FsOpen` Permission
    - [ ] `FsDelete` Permission
  - [ ] Random (`std/rand`)
    - [ ] `RandInt`/`RandRange`
    - [ ] `RandomGen` Permission
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

## 0.11.0-0.x.0 Alpha

- Stabilizations and polish
