use std::io;

enum Symbole
{
	Croix,
	Rond,
	Vide,
}

struct Joueur
{
	pseudo : String,
	score : i32,
	sym : Symbole,
	joue : bool,
}


struct Morpion
{
	v : Vec<[Symbole;3]>,
	t : usize,
}

struct Partie 
{
	mrp : Morpion,
	j1 : Joueur,
	j2 : Joueur,
	etats : bool,
}

impl Symbole
{
	fn clone(&self) -> Symbole 
	{
		match self {
			Symbole::Croix => return Symbole::Croix,
			Symbole::Rond => return Symbole::Rond,
			Symbole::Vide => return Symbole::Vide,
		}
	}
	fn afficher_sybole(&self)
	{
		match self{
			Symbole::Croix => println!("Croix"),
			Symbole::Rond => println!("Rond"),
			Symbole::Vide => println!("Vide"),
		}
	}
	fn retourne_symbole(&self) -> char
	{
		match self{
			Symbole::Croix => return 'X', 
			Symbole::Rond => return 'O', 
			Symbole::Vide => return ' ',
		}
	}
}


impl Joueur
{
	fn afficher_joueur(&self)
	{
		println!("Pseudo : {} , Score : {}",self.pseudo,self.score);
	}
	fn jouer_un_coup(&self, colone : usize, ligne : usize, mrp : &mut Morpion)
	{
		println!("{} joue {} sur la case {} {}", self.pseudo,self.sym.retourne_symbole()
			,ligne,colone);
		mrp.changer_case(colone,ligne,self);
		mrp.afficher_morpion();
	}
}
		
impl Morpion
{
	fn changer_case(&mut self,colone : usize, ligne : usize, jr : &Joueur)
	{
		self.v[ligne][colone] = jr.sym.clone();
	}
	fn afficher_morpion(&self)
	{
		for vec in &self.v
		{
			let mut c : [char;3] = [' ', ' ', ' '];
			let mut i = 0;
			for sym in vec 
			{
				c[i]=sym.retourne_symbole();
				i = i + 1;
			}
			println!("{} | {} | {}", c[0],c[1],c[2]);
		}
	}
	fn verifie_ligne(&self) -> bool
	{
		for vec in &self.v
		{
			for sym in vec 
			{
				let mut conteur_l_x : usize = 0;
				let mut conteur_l_o : usize = 0;
				match sym {
					Symbole::Croix => conteur_l_x +=1,
					Symbole::Rond => conteur_l_o +=1,
					Symbole::Vide => continue,
				}
				if conteur_l_x == 3 || conteur_l_o == 3
				{
					return true;
				}
			}
		}
		return false;
	}
	fn verifie_colone(&self) -> bool
	{
		for c in 0..3
		{
			let mut conteur_c_x : usize = 0;
			let mut conteur_c_o : usize = 0;
			for l in 0..3
			{
				match &self.v[l][c] {
					Symbole::Croix => conteur_c_x +=1,
					Symbole::Rond => conteur_c_o +=1,
					Symbole::Vide => continue,
				}
				if conteur_c_x == 3 || conteur_c_o == 3
				{
					return true;
				}
			}
		}
		return false;
	}
	fn verifie_diagonal(&self) -> bool
	{
		if self.v[0][0].retourne_symbole() == self.v[1][1].retourne_symbole() &&
		self.v[1][1].retourne_symbole() == self.v[2][2].retourne_symbole()
		{
			return true;
		}
		return false;
	}
	fn verifie(&self) -> bool
	{
		let vl : bool = self.verifie_ligne();	
		let vc : bool = self.verifie_colone();	
		let vd : bool = self.verifie_diagonal();	

		if vl == true || vc == true || vd == true
		{
			return true;
		}
		return false;
	}
}

impl Partie {
	fn debut_partie(&mut self)
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
	fn milieu_partie(&mut self)
	{
		println!("La partie peu commencer !");
		self.j1.joue = true;
		let mut tour : u8 = 1;
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
	}
	fn fin_partie(&self)
	{	
		println!("Fin de la partie !");
	}
}
fn creer_partie(j1 : Joueur , j2 : Joueur, mrp : Morpion) -> Partie 
{
	let prt = Partie {
		mrp : mrp,
		j1 : j1,
		j2 : j2,
		etats : false,
	};
	return prt;
}

fn creer_morpion() -> Morpion
{
	let mut mrp = Morpion {
		v : Vec::new(),
		t : 9,
	};
	for _i in 0..3
	{
		mrp.v.push([Symbole::Vide,Symbole::Vide,Symbole::Vide]);
	}
	return mrp;
}

fn creer_joueur(pseudo : &str, sym : Symbole) -> Joueur
{
	let jr = Joueur {
		pseudo : pseudo.to_string(),
		sym : sym.clone(),
		score : 0,
		joue : false,
	};
	return jr;
}
					
fn main()
{
	println!("Morpion.");
	let tom = creer_joueur("Tom",Symbole::Croix);
	let killian = creer_joueur("Killian",Symbole::Rond);
	let mut mrp : Morpion = creer_morpion();
	let mut partie : Partie = creer_partie(tom,killian,mrp);
	partie.debut_partie();
}
