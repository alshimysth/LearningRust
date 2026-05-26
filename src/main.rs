use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    // Pattern 1 : match explicite — le plus lisible
    let nom = match args.get(1) {
        Some(n) => n.as_str(),   // extrait &str depuis &String
        None    => "monde",      // valeur par défaut
    };

    // Pattern 2 : chaîne de méthodes — le plus compact (idiomatique)
    let priorite = args.get(2)
        .map(|s| s.as_str())    // Option<&String> → Option<&str>
        .unwrap_or("medium");   // Option<&str> → &str

    println!("Bonjour {} ! Priorité : {}", nom, priorite);
}