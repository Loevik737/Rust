//to compile use:
//rustc starter.rs -A warnings
use std::process::Command;
use std::io;
fn main(){
    const programs: &'static [ &'static str ] = &[
    "C:/Program Files (x86)/Steam/steam.exe",
    "C:/Program Files (x86)/Skype/Phone/skype.exe"
    ];
    for prog in programs.into_iter(){
        Command::new(prog).spawn();
    }
}
