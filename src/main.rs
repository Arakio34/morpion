mod symbole;
mod joueur;
mod partie;
mod morpion;
fn main()
{
	println!("Morpion.");
	let tom = joueur::new("Tom",symbole::Symbole::Croix);
	let killian = joueur::new("Killian",symbole::Symbole::Rond);
	let mrp : morpion::Morpion = morpion::new();
	let mut partie : partie::Partie = partie::new(tom,killian,mrp);
	partie.debut_partie();
}
