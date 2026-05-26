use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Récupérer le nom (args[1]) ou "monde" par défaut
    let nom: &str = args.get(1)
        .map(|s| s.as_str())
        .unwrap_or("monde");

    // Récupérer la priorité (args[2]) ou "normale" par défaut
    let priorite: &str = args.get(2)
        .map(|s| s.as_str())
        .unwrap_or("normale");

    let message = construire_message(nom, priorite);
    println!("{}", message);

    // Bonus : afficher infos debug
    afficher_infos_debug(&args);
}

/// Construit le message de bienvenue.
/// `nom` et `priorite` sont des &str (références vers du texte).
fn construire_message(nom: &str, priorite: &str) -> String {
    // format! crée une nouvelle String
    format!("Bonjour {} ! Priorité : {}", nom, priorite)
}

/// Affiche des infos de débogage sur les arguments.
/// `args` est emprunté en lecture (&Vec<String>).
fn afficher_infos_debug(args: &Vec<String>) {
    eprintln!("[DEBUG] {} argument(s) reçu(s)", args.len());
    for (i, arg) in args.iter().enumerate() {
        eprintln!("[DEBUG]   args[{}] = {}", i, arg);
    }
}
