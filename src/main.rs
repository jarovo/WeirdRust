use easyinput::input;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
//main funkce
fn fake_main() -> i32 {
    0
}

fn friendship_ofer(exit_code: &i32){
    println!("Chlapík zůstal na místě a začal ti nabízet přátelství.");
    println!("a) Odmíneš.");
    println!("b) Uděláš krok k němu.");

    let user_input = input("");
    if user_input == "a" {
        println!("Vyhrál jsi, neboť to byl kanibal");
        vyhra(&exit_code);
    } else if user_input == "b" {
        println!("Prohrál jsi");
        prohra(&exit_code);
    }
}

fn vyhra(exit_code: &i32) {
    println!("Vyhrál jsi. Můžeš pokračovat.");
    sleep(Duration::from_millis(10000));
    main()
}

fn prohra(exit_code: &i32) {
    println!("Prohraál jsi");
    sleep(Duration::from_millis(10000));
    exit(*exit_code);
}

fn boj(exit_code: &i32) {
    println!("Došlo k nějakému souboji. Vše trvalo věčně a spoustu ");
    println!("lidí dělalo zbytečnou práci při zajišťování důkazů při soudním procesu.");
    println!("Stálo to spousty peněz. Zemřel jsi ty a/nebo tvůj protivník vyčerpáním.");
}

fn policie(exit_code: &i32) {
    println!("Divný člověk u policie řekne, že je vše tvoje vina. Žene to k soudu. Neznáš zákony. Co uděláš?");
    println!("a) Utkáš se s ním ve smírčím řízení.");
    println!("b) Vzdáš to bez boje.");
    println!("c) Vezmeš si právníka a jdeš k soudu.");
    boj(&exit_code)
}

fn main() {
    let exit_code = fake_main();
    println!("Čus vítej u dobré hry");
    println!("Jsi v poušti je u tebe nějakej divnej chlapík. Chce vodu");
    println!("Vyber jednu z možností(napiš a nebo b)");
    println!("a) Dáš mu všechnu svoji vodu.");
    println!("b) Necháš ho.");
    println!("c) Poodstoupíš krok vzad.");
    println!("");
    let user_input = input("");
    match user_input.as_str() {
        "a" => {
            println!("Umřel jsi nemáš co pít");
            prohra(&exit_code);
        }
        "b" => {
            println!("Udělal jsi dobře neboť to byl kanibal");
            println!("Začne tě pronásledovat. Co uděláš? (napiš jedno z písmen)");
            println!("a)Utkáš se s ním v souboji");
            println!("b)Utečeš");
            println!("");
            let user_input = input("");
            if user_input == "a" {
                boj(&exit_code);
                prohra(&exit_code);
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
                    policie(&exit_code);
                    vyhra(&exit_code);
                } else if user_input == "b" {
                    boj(&exit_code);
                    prohra(&exit_code);
                }
            }
        }
        "c" => {
            friendship_ofer(&exit_code);
        }
        &_ => {
            println!("Now you are a God.");
            vyhra(&exit_code);
        }
    }
}