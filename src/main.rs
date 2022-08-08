use easyinput::input;
use std::process::abort;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
//main funkce
fn fake_main() -> i32 {
    0
}

fn main() {
    let exit_code = fake_main();
    println!("Čus vítej u nějaké divné hry");
    println!("Jsi v poušti je u tebe nějakej divnej chlapík");
    println!("Vyber jednu z možností(napiš a nebo b)");
    println!("a) Dáš mu vodu");
    println!("b) Kopneš ho");
    println!("");
    let user_input = input("");
    if user_input == "a" {
        println!("Umřel jsi nemáš co pít");
        sleep(Duration::from_millis(10000));
        exit(exit_code);
    } else if user_input == "b" {
        println!("Udělal jsi dobře neboť to byl kanibal");
        println!("Teď se válí na zemi co s ním uděláš? (napiš jedno z písmen)");
        println!("a)Zakopeš ho do písku");
        println!("b)Utečeš");
        println!("");
        let user_input = input("");
        if user_input == "a" {
            println!("Zatkla tě policie za vraždu");
            sleep(Duration::from_millis(10000));
            exit(exit_code);
        } else if user_input == "b" {
            println!(
                "Udělals dobře jelikož pak by to nebyla vražda, zůstals na svobodě. \
            Ale hledá tě banda kanibalů, chtějí pomstu.\
            Co uděláš?"
            );
            println!("a) Nahlásíš to policii");
            println!("b) Utkáš se s nimi v boji");
            let user_input = input("");
            if user_input == "a" {
                println!("Vyhraál jsi");
                sleep(Duration::from_millis(10000));
                exit(exit_code)
            } else if user_input == "b" {
                println!("Zemřel jsi");
                sleep(Duration::from_millis(10000));
                exit(exit_code);
            }
        }
    }
}
