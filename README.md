# Enigma Machine

## Dependencies
Insert to Cargo.toml of your project.
```toml
[dependencies]
enigma_machine = "*"
```
or
 ```console
❯ cargo add encrypter
```

## Usage
**import**
```rust
extern crate enigma_machine;

use enigma_machine::enigma::{ Enigma, Router, Reflector, Plugboard, SubstitutionTable };
use enigma_machine::enigma::{ SUBSTITUTION_TABLE1, SUBSTITUTION_TABLE2, SUBSTITUTION_TABLE3, REFLECTOR, PLUGBOARD };
```
**setup enigma**
```rust
let mut enimga = Enigma::new(
    vec![
        Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE1.to_vec())),
        Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE2.to_vec())),
        Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE3.to_vec())),
    ],
    Plugboard::new(SubstitutionTable::new(PLUGBOARD.to_vec())),
    Reflector::new(SubstitutionTable::new(REFLECTOR.to_vec()))
);
```
**set routers position.**
```rust
let positions = "ABC";
enigma.set_positions(positions);
```
**encrypt and decrypt.**
```rust
let encrypted = enigma.encrypt(&string);
let decrypted = enigma.decrypt(&encrypted);
```

## License
MIT