mod symbole;
mod joueur;
mod partie;
mod morpion;

fn main()
{
	println!("Morpion.");
	let mut tom = joueur::new();
	let mut killian = joueur::new(); 
	let mrp : morpion::Morpion = morpion::new();
	let mut partie : partie::Partie = partie::new(tom,killian,mrp);
	partie.debut_partie();
}
