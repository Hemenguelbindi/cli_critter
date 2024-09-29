use crate::commands::{CritterCommand, Commands};
use crate::critter::Critter; // если у тебя есть структура Critter

pub struct CritterManager;

impl CritterManager {
    pub fn new() -> CritterManager {
        CritterManager
    }


    pub fn execute(&mut self, command: CritterCommand) {
        match command.commands {
            Commands::StartCritter { name } => {
                println!("Запускаем Тамагочи с именем: {}", name);
                let tamagotchi = Critter::new(name);
            }
            Commands::Status => {
                println!("Запрашиваем статус Тамагочи...");
            }
            Commands::StopCritter => {
                println!("Останавливаем Тамагочи...");
            }
        }
    }
}
