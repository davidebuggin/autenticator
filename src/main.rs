use csv;
use std::error::Error; 
use std::fs::File; 
use serde::Deserialize; 
use promptly::prompt; 

pub trait Autenticator {
    fn login_check(&self) -> Result<bool, Box<dyn Error>>;
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    user_id: String,
    #[serde(rename = "PasswordId")]
    password_id: String,
}

impl Autenticator for User{
    fn login_check(&self) -> Result<bool, Box<dyn Error>>{
        let file = File::open("UtentiRegistrati.csv")?;
        let mut reader = csv::Reader::from_reader(file);

    for result in reader.deserialize() {
        let user: User = result?;
  
        if user.user_id == self.user_id && user.password_id == self.password_id {
            return Ok(true); 
        }
    }

    Ok(false) 
    }
}

fn main()-> Result<(), Box<dyn Error>>{
    
    let user = User { user_id : prompt("Inserisci l'ID utente")? , password_id : prompt("Inserisci la password")?};
    match user.login_check() {
        Ok(true) => println!("Accesso riuscito!"),
        Ok(false) => println!("Accesso fallito!"),
        Err(e) => eprintln!("Errore: {}", e),
    }

    Ok(())
}






