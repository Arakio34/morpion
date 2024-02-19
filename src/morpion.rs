use crate::symbole::Symbole;
use crate::joueur::Joueur;

pub struct Morpion
{
	v : Vec<[Symbole;3]>,
}

impl Morpion
{
   pub fn new() -> Self
   {
     let mut mrp = Morpion {
       v : Vec::new(),
     };
     for _i in 0..3
     {
       mrp.v.push([Symbole::Vide,Symbole::Vide,Symbole::Vide]);
     }
     return mrp;
   }
	pub fn changer_case(&mut self,colone : usize, ligne : usize, jr : &Joueur)
    {
		self.v[ligne][colone] = jr.sym.clone();
	}
	pub fn afficher_morpion(&self)
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
	pub fn verifie_ligne(&self) -> bool
	{
		for vec in &self.v
		{
			let mut conteur_l_x : usize = 0;
			let mut conteur_l_o : usize = 0;
			for sym in vec 
			{
				match sym {
					Symbole::Croix => conteur_l_x +=1,
					Symbole::Rond => conteur_l_o +=1,
					Symbole::Vide => continue,
				}
			}
			if conteur_l_x == 3 || conteur_l_o == 3
			{
				return true;
			}
		}
		return false;
	}
	pub fn verifie_colone(&self) -> bool
	{
		for c in 0..3
		{
			let mut conteur_c_x : usize = 0;
			let mut conteur_c_o : usize = 0;
			for l in 0..3
			{
				match &self.v[l][c] 
			        {
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
	pub fn verifie_diagonal(&self) -> bool
	{
		if self.v[0][0].retourne_symbole() == self.v[1][1].retourne_symbole() &&
		self.v[1][1].retourne_symbole() == self.v[2][2].retourne_symbole()
		{
			return true;
		}
		return false;
	}
	pub fn verifie(&self) -> bool
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

