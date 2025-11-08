# jex

## Overview

> jex is a Java-like Language processor that operaters without the JVM. 
> It directly analyzes and executes Java source code, running on the Rust time. :) 

## Features
* `JVM-independent` : Standalone execution environment without JVM dependency
* `Rust-based` : Implemented in safe and fast Rust
Direct interpretation: Directly analyzes and executes source code
* `Lightweight` : Operates efficiently with minimal dependencies

## install && build

```shell
git clone https://github.com/SteelCrab/jex.git

cd jex

cargo build --release 
```

```mermaid
flowchart TD
    A[Source Code<br/>e.g. int a = 1;] --> B[Lexer<br/>Tokenize]
    B --> C[Parser<br/>Build AST]
    C --> D[Semantic Analyzer<br/>Type Checking]
    D --> E[Code Generator<br/>Generate IR or Bytecode]
    E --> F[Compiler Backend<br/>LLVM or Rust Codegen]
    F --> G[Executable / Binary]
    G --> H[Runtime Environment<br/>Execution + Memory + I/O]

```
