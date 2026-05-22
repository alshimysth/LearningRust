use std::env;
fn main() {
    // env::args() retourne un itérateur sur les arguments
    // collect() le matérialise en Vec<String>
    let args: Vec<String> = env::args().collect();

    // Affiche tous les arguments pour comprendre la structure
    println!("Nombre d'args : {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("  args[{}] = {}", i, arg);
    }

    // DANGEREUX : panic si args.len() < 2
    // let premier = args[1];

    // SÛR : retourne Option<&String>
    match args.get(1) {
        Some(arg) => println!("Premier arg : {}", arg),
        None      => println!("Aucun argument fourni"),
    }

    // cargo run -- hello monde 42
    // Premier arg : hello

    // cargo run
    // Aucun argument fourni
}