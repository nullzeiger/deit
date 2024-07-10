use std::collections::HashMap;
use std::io;
use std::process::exit;

fn main() {
    let verbs = HashMap::from([
        ("werden", "diventare"),
        ("haben", "avere"),
        ("sein", "essere"),
        ("können", "potere"),
        ("müssen", "dovere"),
        ("sollen", "dovere"),
        ("sagen", "dire"),
        ("geben", "dare"),
        ("kommen", "venire"),
        ("wollen", "volere"),
        ("machen", "fare"),
        ("gehen", "andare"),
        ("heißen", "chiamare"),
        ("wisen", "sapere"),
        ("sehen", "vedere"),
        ("finden", "trovare"),
        ("bleiben", "rimanere"),
        ("mögen", "piacere"),
        ("fahren", "andare"),
    ]);

    for (key, value) in &verbs {
        let mut guess = String::new();
        println!("Traduci {} verbo in Italiano (q per uscire): ", key);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();

        if guess.as_str() == "q" {
            exit(0)
        }

        match verbs.get(key) {
            Some(verb) => {
                if verb == &guess {
                    println!("Giusto {} {}", key, value)
                } else {
                    println!("Sbagliato {} {}", key, value)
                }
            }
            None => println!("Non esiste il verbo che hai tradotto"),
        }
    }
}
