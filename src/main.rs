use easy input::input;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
//main funkce
fn fake_main() -> i32 {
    0
}

fn friendship_ofer(exit_code: &i32) -> &i32 {
    println!("Chlapík zůstal na místě a začal ti nabízet přátelství.");
    println!("a) Odmíneš.");
    println!("b) Uděláš krok k němu.");

    let user_input = input("");
    if user_input == "a" {
        println!("Vyhrál jsi, nebo to byl kanibal");
        sleep(Duration::from_millis(10000));
        exit(*exit_code);
    } else if user_input == "b" {
        println!("Prohraál jsi");
        sleep(Duration::from_millis(10000));
        exit(*exit_code);
    }

    return exit_code
}

fn prohra(exit_code: &i32) {
    println!("Prohraál jsi");
    sleep(Duration::from_millis(10000));
    exit(*exit_code);
}

fn main() {
    let exit_code = fake_main();
    println!("Čus vítej u nějaké divné hry");
    println!("Jsi v poušti je u tebe nějakej divnej chlapík");
    println!("Vyber jednu z možností(napiš a nebo b)");
    println!("a) Dáš mu vodu");
    println!("b) Necháš ho");
    println!("c) Poodstoupíš krok vzad");
    println!("");
    let user_input = input("");
    if user_input == "a" {
        println!("Umřel jsi nemáš co pít");
        prohra(&exit_code)
    } else if user_input == "b" {
        println!("Udělal jsi dobře neboť to byl kanibal");
        println!("Začne tě pronásledovat. Co uděláš? (napiš jedno z písmen)");
        println!("a)Utkáš se s ním v souboji");
        println!("b)Utečeš");
        println!("");
        let user_input = input("");
        if user_input == "a" {
            prohra(&exit_code)
        } else if user_input == "b" {
            println!(
                "Udělals dobře jelikož by jsi prohrál, zůstals na svobodě. \
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
    } else if user_input == "c" {
        friendship_ofer(&exit_code);
    }
}
