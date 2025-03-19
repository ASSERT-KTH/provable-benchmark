Project for keeping track of benchamrking and testing ZKPoX in the RiscZERO zkVM.

google sheets document with in pogress benchmarks https://docs.google.com/spreadsheets/d/17S0oak3UW_kJlua0hc1BWxX_xTfjMtx3ETi8M5xgVVA/edit?usp=sharing 

Work in progress..


## Directory Structure

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                    <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```

