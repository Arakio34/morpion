mod symbole;
mod joueur;
mod partie;
mod morpion;

use crate::partie::Partie;

fn main()
{
	let mut partie : partie::Partie = Partie::new();
	partie.debut_partie();
}
