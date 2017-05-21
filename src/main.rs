
#[macro_use]
extern crate lazy_static;

extern crate base64;

mod enigma_machine;
use enigma_machine::{ Enigma, Router, Reflector, Plugboard, SubstitutionTable };
use enigma_machine::{ SUBSTITUTION_TABLE1, SUBSTITUTION_TABLE2, SUBSTITUTION_TABLE3, REFLECTOR, PLUGBOARD };

mod utility;

fn main() {
    enigma("A quick brown fox jumps over the lazy dog.", "ABC");
    enigma("いろはにほへとちりぬるを わかよたれそつねならむ うゐのおくやまけふこゑて あさきゆめみしえひもせすん", "DEF");
}

fn enigma(string: &str, positions: &str) {

    let mut enigma = Enigma::new(
        vec![
            Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE1.to_vec())),
            Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE2.to_vec())),
            Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE3.to_vec())),
        ],
        Plugboard::new(SubstitutionTable::new(PLUGBOARD.to_vec())),
        Reflector::new(SubstitutionTable::new(REFLECTOR.to_vec()))
    );

    enigma.set_positions(positions);
    println!(" ");
    println!("string: {}", string);

    let encrypted = enigma.encrypt(&string);
    println!("encrypted: {}", encrypted);

    enigma.set_positions(positions);
    let decrypted = enigma.decrypt(&encrypted);
    println!("decrypted: {}", decrypted);
}
