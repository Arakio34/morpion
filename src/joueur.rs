use crate::symbole::Symbole;
use crate::morpion::Morpion;

pub struct Joueur
{
	pub pseudo : String,
	pub score : i32,
	pub sym : Symbole,
	pub joue : bool,
}

impl Joueur
{
	/*pub fn afficher_joueur(&self)
	{
		println!("Pseudo : {} , Score : {}",self.pseudo,self.score);
	}*/
	pub fn jouer_un_coup(&self, colone : usize, ligne : usize, mrp : &mut Morpion)
	{
		println!("{} joue {} sur la case {} {}", self.pseudo,self.sym.retourne_symbole()
			,ligne,colone);
		mrp.changer_case(colone,ligne,self);
		mrp.afficher_morpion();
	}
}
pub fn new(pseudo : &str, sym : Symbole) -> Joueur
{
	let jr = Joueur {
		pseudo : pseudo.to_string(),
		sym : sym.clone(),
		score : 0,
		joue : false,
	};
	return jr;
}
