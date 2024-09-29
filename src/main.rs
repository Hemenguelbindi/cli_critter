mod commands;
mod critter;
mod critter_manager;

use clap::Parser;
use commands::CritterCommand;
use critter_manager::CritterManager;

#[cfg(unix)]
use daemonize::Daemonize;
use notify_rust::Notification;

#[]

fn main() {
    let cli = CritterCommand::parse();


    let mut critter_manager =  CritterManager::new();


    #[cfg(unix)]
    {
        let daemonize = Daemonize::new().
        pid_file("critter.pid").working_directory("/tmp").usrname("critter")
        .group_name("critter").umask(0o777).stdout("/tmp/tamagotchi.out").stderr("/tmp/tamagotchi.err");
        match daemonize.start() {
            Ok() => {println!("Успешно запущен в фоне")}
            Err(e) => {println!("Ошибка: {:?}", e)}
        }
    }

    critter_manager.execute(cli);
    #[cfg(unix)]
    Notification::new()
        .summary("Tamagotchi")
        .body("Tamagotchi запущен в фоновом режиме и готов к игре!")
        .show()
        .unwrap();    
}