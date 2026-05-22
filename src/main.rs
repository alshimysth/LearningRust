fn main() {
    let name = "Dimitri";
    let done: i32 = 75;
    let total: i32 = 96;

    // println! : affiche avec saut de ligne (stdout)
    println!("Bonjour {}", name);
    println!("{} tâches sur {}", done, total);

    // print! : affiche SANS saut de ligne
    print!("Entrez un nom : ");

    // eprintln! : affiche sur stderr (pour les erreurs et logs)
    eprintln!("Erreur : fichier introuvable");

    // format! : crée une String sans afficher
    let message = format!("Bonjour {} !", name);

    // dbg! : affiche la valeur + le nom de la variable (pour déboguer)
    let x = 42;
    dbg!(x); // affiche: [src/main.rs:5] x = 42

    // assert_eq! : utilisé dans les tests
    assert_eq!(2 + 2, 4);

    let task_id = 42;
    let title = "Finir le sprint";
    let ratio = 0.75;

    println!("{}", task_id); // 42                  — Display (lisible humain)
    println!("{:?}", task_id); // 42                  — Debug (dev)
    println!("{:#?}", title); // "Finir le sprint"   — Debug pretty-print
    println!("{:>10}", title); // aligné à droite sur 10 chars
    println!("{:.1}", ratio); // 0.8                — 1 décimales
    println!("{:05}", task_id); // 00042               — padding avec zéros
    println!("{title}"); // Rust 1.58+ : interpolation par nom de variable

    let x = 1;
    let x = 2;
    println!("{}", x);

    // Créer un Vec vide
    let mut v: Vec<i32> = Vec::new();

    // Créer avec des valeurs initiales — macro vec!
    let v2 = vec![1, 2, 3, 4, 5];
    let noms = vec!["Alice", "Bob", "Charlie"]; // Vec<&str>

    // Ajouter des éléments
    v.push(10);
    v.push(20);
    v.push(30);

    // Taille
    println!("Taille : {}", v.len()); // 3
    println!("Vide ? {}", v.is_empty()); // false

    // Accès par index
    println!("Premier : {}", v[0]); // peut paniquer
    println!("Premier : {:?}", v.get(0)); // Some(10) — sécurisé

    // Supprimer le dernier → retourne Option<T>
    if let Some(dernier) = v.pop() {
        println!("Retiré : {}", dernier); // 30
    }

    // Itérer
    for element in &v {
        // &v emprunte le Vec, v reste utilisable après
        println!("{}", element);
    }

    // Itérer avec index
    for (i, element) in v.iter().enumerate() {
        println!("[{}] = {}", i, element);
    }

    // Filtrer et collecter en nouveau Vec
    let grands: Vec<i32> = v
        .iter()
        .filter(|&&x| x > 15)
        .copied() // convertit &&i32 en i32
        .collect();

    // Shadowing : nouvelle variable avec le même nom
    let score = 100;
    println!("Score brut : {}", score);

    let score = score / 10; // Nouvelle variable, pas une mutation
    println!("Score /10 : {}", score);

    let score = format!("{}/10", score); // On change même de type : i32 → String
    println!("Score formaté : {}", score);

    // Format de println!
    let nombre = 42;
    let texte = "TaskFlow";
    let ratio = 0.6666_f64;

    println!("{}", nombre); // 42          → Display (lisible)
    println!("{:?}", nombre); // 42          → Debug
    println!("{:#?}", texte); // "TaskFlow"  → Debug pretty

    println!("{:>10}", texte); // "  TaskFlow" → aligné à droite, largeur 10
    println!("{:<10}", texte); // "TaskFlow  " → aligné à gauche
    println!("{:^10}", texte); // " TaskFlow " → centré

    println!("{:.2}", ratio); // 0.67        → 2 décimales
    println!("{:08.2}", ratio); // 00000.67    → padded avec zéros
    println!("{:+}", nombre); // +42         → signe explicite

    // Interpolation par nom (Rust 1.58+)
    println!("{nombre} et {texte}"); // 42 et TaskFlow

    // Appel de fonctions
    let message = saluer("Dimitri");
    println!("{}", message);

    let resultat = additionner(10, 32);
    println!("10 + 32 = {}", resultat);

    // if comme expression (pas besoin de ternaire)
    let age = 25;
    let categorie = if age < 18 { "mineur" } else { "majeur" };
    println!("Catégorie : {}", categorie);
}

// Fonction qui retourne une String
// &str en paramètre : emprunte le texte sans en prendre possession
fn saluer(nom: &str) -> String {
    format!("Bonjour {} depuis TaskFlow !", nom)
    // Pas de ; → cette expression est la valeur retournée
    // Équiv Java : return "Bonjour " + nom + " depuis TaskFlow !";
}

// Fonction qui retourne un i32
fn additionner(a: i32, b: i32) -> i32 {
    a + b  // expression finale = valeur de retour
}