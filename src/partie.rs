use crate::{
  symbole::Symbole,
  joueur::Joueur,
  morpion::Morpion,
};
use std::io;

//TODO 
/*

Multiple partie et sauvgarde de score dans un fichier 


*/
pub struct Partie 
{
	pub mrp : Morpion,
	pub j1 : Joueur,
	pub j2 : Joueur,
	pub etats : bool,
}

impl Partie {
    pub fn new() -> Self
    {
    let mrp : Morpion = Morpion::new();
    let mut j1 = Joueur::new();
    let mut j2 = Joueur::new(); 
      let prt = Partie 
      {
          mrp : mrp,
          j1 : j1,
          j2 : j2,
          etats : false,
      };
      return prt;
   }
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
			self.j1.joue = true;
			let mut tour : u8 = 1;
			while self.etats
			{
				println!("--- Tours numeros {} --- ",tour);
				tour += 1;
				match self.j1.joue{
					true =>{
						println!("Au tour de {}",self.j1.pseudo);
					},
					false =>{
						println!("Au tour de {}",self.j2.pseudo);
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
		println!("Voulez vous rejouez ? 0 : oui , 1 : non");
    let mut restart = String::new();
    io::stdin().read_line(&mut restart).expect("erreur lors de la lecture de la ligne");
    let res : usize = restart.trim().parse().expect("erreur lors de la lecture de la ligne");
    if res == 0
    {
      self.debut_partie();
    }
	}
}
