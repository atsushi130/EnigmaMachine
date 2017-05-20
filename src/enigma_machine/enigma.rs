
/// enigma module
use super::{ Router, Reflector, RouterManager, Plugboard };

/// rust_serialize module
use rustc_serialize::base64::{ ToBase64, FromBase64,STANDARD };

pub struct Enigma {
    router_manager: RouterManager,
    plugboard: Plugboard,
    reflector: Reflector
}

impl Enigma {

    pub fn new(routers: Vec<Router>, plugboard: Plugboard, reflector: Reflector) -> Self {
        Enigma {
            router_manager: RouterManager {
                routers: routers
            },
            plugboard,
            reflector
        }
    }

    pub fn encrypt(&mut self, string: &str) -> String {

        let encrypted: Vec<String> = self.encode_base64(string).chars().map( |character| {

            let intput = self.plugboard.input(&character);

            let in_encrypted = self.router_manager.encrypt_to_reflector(&intput);
            let reflected = self.reflector.reflect(&in_encrypted);
            let out_encrypted = self.router_manager.encrypt_from_reflector(&reflected);

            self.router_manager.increment();
            self.plugboard.output(&out_encrypted).to_string()
        }).collect();

        encrypted.join("")
    }

    fn encode_base64(&self, string: &str) -> String {
        let bytes = string.as_bytes();
        bytes.to_base64(STANDARD)
    }

    pub fn decrypt(&mut self, string: &str) -> String {

        let decrypted: Vec<String> = string.chars().map( |character| {

            let intput = self.plugboard.input(&character);

            let in_encrypted = self.router_manager.encrypt_to_reflector(&intput);
            let reflected = self.reflector.reflect(&in_encrypted);
            let out_encrypted = self.router_manager.encrypt_from_reflector(&reflected);

            self.router_manager.increment();
            self.plugboard.output(&out_encrypted).to_string()
        }).collect();

        self.decode_base64(&decrypted.join(""))
    }

    fn decode_base64(&self, string: &str) -> String {
        let bytes = string.from_base64().unwrap();
        String::from_utf8(bytes).unwrap()
    }

    pub fn set_positions(&mut self, positions: &str) -> &Self {

        if positions.chars().count() != self.router_manager.routers.len() {
            panic!("The number of positions does not match the number of routers.");
        }

        for (router, position) in self.router_manager.routers.iter_mut().zip(positions.chars()) {
            router.set_position(&position);
        }

        return self
    }
}
