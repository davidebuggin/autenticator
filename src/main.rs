use csv;
use std::error::Error;
use std::fs::File;
// use std::intrinsics::mir::Return;
use promptly::prompt;
use serde::Deserialize;
use std::string::String;
trait Autenticator {
    fn login_check(&self, user_id: &str, password: &str) -> Result<bool, Box<dyn Error>>;
}

#[derive(Debug, Deserialize)]
struct User {
    #[serde(rename = "UserId")]
    user_id: String,
    #[serde(rename = "PasswordId")]
    password: String,
}

struct Autentication {
    file: File,
}

struct ToyAutentication {
    safe_password: [String; 3],
}

impl Autenticator for Autentication {
    fn login_check(&self, user_id: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        let mut reader = csv::Reader::from_reader(&self.file);

        for result in reader.deserialize() {
            let user: User = result?;

            if user.user_id == user_id && user.password == password {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Autenticator for ToyAutentication {
    fn login_check(&self, user_id: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        for p in &self.safe_password {
            if p.trim() == (password) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let user = User {
        user_id: prompt("Inserisci l'ID utente")?,
        password: prompt("Inserisci la password")?,
    };
    let autenticator = Autentication {
        file: File::open("UtentiRegistrati.csv")?,
    };
    let safe_password = ToyAutentication {
        safe_password: ["1".to_string(), "2".to_string(), "3".to_string()],
    };
    match safe_password.login_check(&user.user_id, &user.password) {
        Ok(true) => println!("Accesso riuscito!"),
        Ok(false) => println!("Accesso fallito!"),
        Err(e) => eprintln!("Errore: {}", e),
    }

    Ok(())
}
