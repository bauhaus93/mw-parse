#[macro_use]
extern crate log;

extern crate logger;
extern crate esm_parser;


use esm_parser::GameData;

pub fn main() {
    match logger::init() {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to init log: {}", e);
            return;
        }
    }

    let game_data = match esm_parser::parse_game_data("mwstuff/Morrowind.esm") {
        Ok(gd) => gd,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("{}", game_data);
    
}
