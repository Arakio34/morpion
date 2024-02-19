use crate::partie;
use crate::joueur;

struct Score
{
  player : String,
  point : usize,
}


impl Score 
{
  pub fn new(winner : Joueur) -> Self
  {
    Score
    {
      player : winner.pseudo,
      point : winner.point,
    }
  }
}
