extern crate rusty;

use rusty::*;
use Term::*;


fn church(k: u8) -> Box<Term> {
  let b = (0..k).fold(Box::new(Var(0)), |a, _| Box::new(App(Box::new(Var(1)), a)));
  Box::new(Lam(Box::new(Lam(b))))
}

fn main() {
  let id = Lam(Box::new(Var(0)));
  println!("{}", to_string(&id));
  let c2 = church(2);
  let c3 = church(3);
  let c8 = eval(&Box::new(App(c2, c3)));
  println!("{}", to_string(&c8));
}
