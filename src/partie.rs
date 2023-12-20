use crate::symbole::Symbole;
use crate::joueur::Joueur;
use crate::morpion::Morpion;
use std::io;

pub struct Partie 
{
	pub mrp : Morpion,
	pub j1 : Joueur,
	pub j2 : Joueur,
	pub etats : bool,
}

impl Partie {
	pub fn debut_partie(&mut self)
	{
		if self.j1.sym.retourne_symbole() == self.j2.sym.retourne_symbole()
		{
			match self.j1.sym {
				Symbole::Croix => self.j2.sym = Symbole::Rond,
				Symbole::Rond => self.j2.sym = Symbole::Croix,
				Symbole::Vide => {
					self.j1.sym = Symbole::Rond;
					self.j2.sym = Symbole::Croix;
				},
			}
		}
		self.j1.score = 0;
		self.j2.score = 0;
		self.etats = true;
		self.milieu_partie();
	}
	pub fn milieu_partie(&mut self)
	{
		println!("La partie peu commencer !");
		self.j1.joue = true;
		let tour : u8 = 1;
		while self.etats
		{
			println!("Tours numeros {}",tour);
			self.j1.joue = true;
			let mut tour : u8 = 1;
			while self.etats
			{
				println!("Tours numeros {}",tour);
				tour += 1;
				match self.j1.joue{
					true =>{
						println!("{} joue avec les {}",self.j1.pseudo,
							self.j1.sym.retourne_symbole());
					},
					false =>{
						println!("{} joue avec les {}",self.j2.pseudo,
							self.j2.sym.retourne_symbole());
					},
				}
				let mut ligne = String::new();
				let mut colone = String::new();
				println!("Annoncer la ligne");
				io::stdin().read_line(&mut ligne)
					.expect("Failure");
				println!("Annoncer la colone");
				io::stdin().read_line(&mut colone)
					.expect("Failure");
				let ligne : usize = ligne.trim().parse()
					.expect("Donner une valeur valide");
				let colone : usize = colone.trim().parse()
					.expect("Donner une valeur valide");
				match self.j1.joue{
					true => {
						self.j1.jouer_un_coup(colone,ligne,&mut self.mrp);
						self.j1.joue = false;
						self.j2.joue = true;
					},
					false => {
						self.j2.jouer_un_coup(colone,ligne,&mut self.mrp);
						self.j1.joue = true;
						self.j2.joue = false;
					},
				}
				if tour >= 5 
				{
					if self.mrp.verifie()
					{
						self.etats = false;
					}
				}
			}
		}
		self.fin_partie();
	}
	pub fn fin_partie(&mut self)
	{	
		match self.j1.joue{
			true => {
				println!(" {} a gagner !!!",self.j2.pseudo); 
				self.j2.score +=1;
			}
			false => {
				println!(" {} a gagner !!!",self.j1.pseudo); 
				self.j1.score +=1;
			}
		}
		println!("Fin de la partie !");
	}
}
pub fn new(j1 : Joueur , j2 : Joueur, mrp : Morpion) -> Partie 
{
	let prt = Partie {
		mrp : mrp,
		j1 : j1,
		j2 : j2,
		etats : false,
	};
	return prt;
}
