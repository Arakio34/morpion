use crate::symbole::Symbole;
use crate::morpion::Morpion;
use std::io;

pub struct Joueur
{
	pub pseudo : String,
	pub score : i32,
	pub sym : Symbole,
	pub joue : bool,
}

impl Joueur
{
	pub fn afficher_joueur(&self)
	{
		println!("---\nPseudo : {} \n Score : {}\n---",self.pseudo,self.score);
	}
	pub fn jouer_un_coup(&self, colone : usize, ligne : usize, mrp : &mut Morpion)
	{
		mrp.changer_case(colone,ligne,self);
		mrp.afficher_morpion();
	}
}
pub fn new() -> Joueur
{
	println!("Saisire un pseudo : ");		
	let mut psd = String::new();
	io::stdin().read_line(&mut psd)
		.expect("Erreur");
	println!("Quel symbole voulez vous (1 : X | 2 : O) : ");		
	let mut s = String::new();
	io::stdin().read_line(&mut s)
		.expect("Erreur");
	let sym : Symbole;
	match &s[..]{
		"1" => sym = Symbole::Croix,
		"2" => sym = Symbole::Rond,
		 _ => sym = Symbole::Vide,
	};	

	let jr = Joueur {
		pseudo : psd,
		sym : sym.clone(),
		score : 0,
		joue : false,
	};
	jr.afficher_joueur();
	return jr;
}
