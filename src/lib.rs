// Copyright (c) 2024  Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

pub mod verb {

    use std::collections::HashMap;
    use std::io;
    use std::process::exit;

    fn guess_verb(key: &str, guess: &str, verbs: &HashMap<&str, &str>) -> bool {
        match verbs.get(key) {
            Some(verb) => verb == &guess,
            None => false,
        }
    }

    pub fn print_verb() {
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

            let guess = guess.trim();

            if guess == "q" {
                exit(0)
            }

            if guess_verb(key, guess, &verbs) {
                println!("Giusto {} {}", key, value);
            } else {
                println!("Sbagliato {} {}", key, value);
            }
        }
    }
}
