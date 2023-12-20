pub enum Symbole
{
	Croix,
	Rond,
	Vide,
}

impl Symbole
{
	pub fn clone(&self) -> Symbole 
	{
		match self {
			Symbole::Croix => return Symbole::Croix,
			Symbole::Rond => return Symbole::Rond,
			Symbole::Vide => return Symbole::Vide,
		}
	}
	/*pub fn afficher_symbole(&self)
	{
		match self{
			Symbole::Croix => println!("Croix"),
			Symbole::Rond => println!("Rond"),
			Symbole::Vide => println!("Vide"),
		}
	}*/
	pub fn retourne_symbole(&self) -> char
	{
		match self{
			Symbole::Croix => return 'X', 
			Symbole::Rond => return 'O', 
			Symbole::Vide => return ' ',
		}
	}
}

