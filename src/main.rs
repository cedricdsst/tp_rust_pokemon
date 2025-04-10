use std::fmt;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

// Définir les Pokemon
#[derive(Debug, Clone, PartialEq)]
pub enum PokemonType {
    Feu,
    Eau,
    Plante,
    Electrik,
    Normal,
}

impl fmt::Display for PokemonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PokemonType::Feu => write!(f, "Feu"),
            PokemonType::Eau => write!(f, "Eau"),
            PokemonType::Plante => write!(f, "Plante"),
            PokemonType::Electrik => write!(f, "Electrik"),
            PokemonType::Normal => write!(f, "Normal"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Genre {
    Male,
    Femelle,
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Genre::Male => write!(f, "Mâle"),
            Genre::Femelle => write!(f, "Femelle"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    nom: String,
    niveau: u32,
    pokemon_type: PokemonType,
    experience: u32,
    genre: Genre,
}

// Fonctions et comportements
impl Pokemon {
    pub fn new(nom: String, pokemon_type: PokemonType, genre: Genre) -> Self {
        Pokemon {
            nom,
            niveau: 1,
            pokemon_type,
            experience: 0,
            genre,
        }
    }

    pub fn gagner_experience(&mut self, xp: u32) {
        self.experience += xp;
        
        // Vérifier si le Pokémon peut monter de niveau
        let niveaux_a_ajouter = self.experience / 100;
        if niveaux_a_ajouter > 0 {
            self.niveau += niveaux_a_ajouter;
            self.experience %= 100;
            println!("{} est passé au niveau {}!", self.nom, self.niveau);
        }
    }

    pub fn display(&self) {
        println!("Nom: {}", self.nom);
        println!("Niveau: {}", self.niveau);
        println!("Type: {}", self.pokemon_type);
        println!("XP: {}", self.experience);
        println!("Genre: {}", self.genre);
    }

    pub fn peuvent_se_reproduire(&self, autre: &Pokemon) -> bool {
        // Vérifier si les deux Pokémon sont du même type
        if self.pokemon_type != autre.pokemon_type {
            return false;
        }
        
        // Vérifier si les genres sont opposés
        if self.genre == autre.genre {
            return false;
        }
        
        
        if self.niveau < 5 || autre.niveau < 5 {
            return false;
        }
        
        true
    }

    //Reproduction
    pub fn reproduire(&self, autre: &Pokemon) -> Option<Pokemon> {
        if self.peuvent_se_reproduire(autre) {
            let mut rng = rand::thread_rng();
            
            // Générer aléatoirement le genre du nouveau Pokémon
            let genre = if rng.gen_bool(0.5) {
                Genre::Male
            } else {
                Genre::Femelle
            };
            
            // Générer aléatoirement le nom du nouveau Pokémon
            let noms = vec!["Babybulbe", "Poussin", "Flambi", "Aquali", "Sparkle", "Mystère"];
            let nom = noms[rng.gen_range(0..noms.len())].to_string();
            
            // Créer un nouveau Pokémon
            let bebe = Pokemon {
                nom,
                niveau: 1,
                pokemon_type: self.pokemon_type.clone(),
                experience: 0,
                genre,
            };
            
            Some(bebe)
        } else {
            None
        }
    }
}

// Gestion de l'élevage
#[derive(Debug)]
pub struct Elevage {
    pokemon: Vec<Pokemon>,
}

impl Elevage {
    pub fn new() -> Self {
        Elevage { pokemon: Vec::new() }
    }
    
    pub fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        self.pokemon.push(pokemon);
    }
    
    pub fn afficher_tous_les_pokemon(&self) {
        println!("=== Liste des Pokémon dans l'élevage ===");
        for (index, pokemon) in self.pokemon.iter().enumerate() {
            println!("Pokémon #{}", index + 1);
            pokemon.display();
            println!("--------------------------");
        }
    }
    
    pub fn entrainer_tous_les_pokemon(&mut self, xp: u32) {
        for pokemon in &mut self.pokemon {
            pokemon.gagner_experience(xp);
        }
        println!("Tous les Pokémon ont gagné {} points d'expérience!", xp);
    }
    
    pub fn tenter_reproduction(&mut self, index1: usize, index2: usize) -> bool {
        // Vérifier si les indices sont valides
        if index1 >= self.pokemon.len() || index2 >= self.pokemon.len() {
            println!("Indices invalides!");
            return false;
        }
        
        // Cloner les Pokémon pour les passer à la méthode reproduire
        let pokemon1 = self.pokemon[index1].clone();
        let pokemon2 = self.pokemon[index2].clone();
        
        // Tenter la reproduction
        if let Some(bebe) = pokemon1.reproduire(&pokemon2) {
            println!("Reproduction réussie! Un nouveau Pokémon est né!");
            println!("Informations sur le nouveau Pokémon:");
            bebe.display();
            
            // Ajouter le bébé à l'élevage
            self.ajouter_pokemon(bebe);
            true
        } else {
            println!("La reproduction a échoué. Les Pokémon ne sont pas compatibles.");
            false
        }
    }
    
  
    pub fn trier_par_niveau(&mut self) {
        self.pokemon.sort_by(|a, b| a.niveau.cmp(&b.niveau));
    }
    
    pub fn trier_par_type(&mut self) {
        self.pokemon.sort_by(|a, b| {
            let type_a = format!("{}", a.pokemon_type);
            let type_b = format!("{}", b.pokemon_type);
            type_a.cmp(&type_b)
        });
    }
    
   
    pub fn sauvegarder_dans_fichier(&self, chemin: &str) -> std::io::Result<()> {
        let mut fichier = File::create(chemin)?;
        
        for pokemon in &self.pokemon {
            writeln!(
                fichier,
                "{};{};{};{};{}",
                pokemon.nom,
                pokemon.niveau,
                format!("{}", pokemon.pokemon_type),
                pokemon.experience,
                format!("{}", pokemon.genre)
            )?;
        }
        
        println!("Données sauvegardées avec succès dans {}", chemin);
        Ok(())
    }
}

fn main() {
    let mut elevage = Elevage::new();
    
    // Add some initial Pokemon
    let pikachu = Pokemon::new(
        String::from("Pikachu"),
        PokemonType::Electrik,
        Genre::Male,
    );
    
    let salameche = Pokemon::new(
        String::from("Salamèche"),
        PokemonType::Feu,
        Genre::Male,
    );
    
    let carapuce = Pokemon::new(
        String::from("Carapuce"),
        PokemonType::Eau,
        Genre::Femelle,
    );
    
    elevage.ajouter_pokemon(pikachu);
    elevage.ajouter_pokemon(salameche);
    elevage.ajouter_pokemon(carapuce);
    
    loop {
        println!("\n=== Menu Principal ===");
        println!("1. Afficher tous les Pokémon");
        println!("2. Ajouter un nouveau Pokémon");
        println!("3. Entraîner les Pokémon");
        println!("4. Tenter une reproduction");
        println!("5. Trier les Pokémon");
        println!("6. Sauvegarder dans un fichier");
        println!("7. Quitter");
        
        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        
        match choix.trim() {
            "1" => {
                elevage.afficher_tous_les_pokemon();
            },
            "2" => {
                println!("Entrez le nom du Pokémon:");
                let mut nom = String::new();
                std::io::stdin().read_line(&mut nom).expect("Erreur de lecture");
                
                println!("Choisissez le type (1: Feu, 2: Eau, 3: Plante, 4: Electrik, 5: Normal):");
                let mut type_choix = String::new();
                std::io::stdin().read_line(&mut type_choix).expect("Erreur de lecture");
                
                let pokemon_type = match type_choix.trim() {
                    "1" => PokemonType::Feu,
                    "2" => PokemonType::Eau,
                    "3" => PokemonType::Plante,
                    "4" => PokemonType::Electrik,
                    "5" => PokemonType::Normal,
                    _ => {
                        println!("Choix invalide, type par défaut: Normal");
                        PokemonType::Normal
                    }
                };
                
                println!("Choisissez le genre (1: Mâle, 2: Femelle):");
                let mut genre_choix = String::new();
                std::io::stdin().read_line(&mut genre_choix).expect("Erreur de lecture");
                
                let genre = match genre_choix.trim() {
                    "1" => Genre::Male,
                    "2" => Genre::Femelle,
                    _ => {
                        println!("Choix invalide, genre par défaut: Mâle");
                        Genre::Male
                    }
                };
                
                let nouveau_pokemon = Pokemon::new(nom.trim().to_string(), pokemon_type, genre);
                elevage.ajouter_pokemon(nouveau_pokemon);
                println!("Pokémon ajouté avec succès!");
            },
            "3" => {
                println!("Entrez le nombre de points d'expérience à donner:");
                let mut xp = String::new();
                std::io::stdin().read_line(&mut xp).expect("Erreur de lecture");
                if let Ok(xp_value) = xp.trim().parse::<u32>() {
                    elevage.entrainer_tous_les_pokemon(xp_value);
                } else {
                    println!("Valeur invalide!");
                }
            },
            "4" => {
                elevage.afficher_tous_les_pokemon();
                println!("Entrez l'index du premier Pokémon:");
                let mut index1 = String::new();
                std::io::stdin().read_line(&mut index1).expect("Erreur de lecture");
                
                println!("Entrez l'index du deuxième Pokémon:");
                let mut index2 = String::new();
                std::io::stdin().read_line(&mut index2).expect("Erreur de lecture");
                
                if let (Ok(i1), Ok(i2)) = (index1.trim().parse::<usize>(), index2.trim().parse::<usize>()) {
                    elevage.tenter_reproduction(i1 - 1, i2 - 1);
                } else {
                    println!("Indices invalides!");
                }
            },
            "5" => {
                println!("Choisissez le critère de tri (1: Niveau, 2: Type):");
                let mut tri_choix = String::new();
                std::io::stdin().read_line(&mut tri_choix).expect("Erreur de lecture");
                
                match tri_choix.trim() {
                    "1" => {
                        elevage.trier_par_niveau();
                        println!("Pokémon triés par niveau:");
                    },
                    "2" => {
                        elevage.trier_par_type();
                        println!("Pokémon triés par type:");
                    },
                    _ => println!("Choix invalide!"),
                }
                elevage.afficher_tous_les_pokemon();
            },
            "6" => {
                println!("Entrez le nom du fichier pour la sauvegarde:");
                let mut filename = String::new();
                std::io::stdin().read_line(&mut filename).expect("Erreur de lecture");
                
                if let Err(e) = elevage.sauvegarder_dans_fichier(filename.trim()) {
                    println!("Erreur lors de la sauvegarde: {}", e);
                }
            },
            "7" => {
                println!("Au revoir!");
                break;
            },
            _ => println!("Choix invalide!"),
        }
    }
}