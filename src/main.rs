extern crate rusty;

use std::thread;

use rusty::*;
use Term::*;


fn church(k: u8) -> Box<Term> {
  let b = (0..k).fold(var(0), |a, _| app(var(1), a));
  lam(lam(b))
}

fn nmpair(n: u8, m: u8) -> Box<Term> {
  // t = \x.\y.((a)(x)y)(b)(y)x
  let b = app( app(Box::new(Free("a".to_string())), app(var(1), var(0))), app(Box::new(Free("b".to_string())), app(var(0), var(1))) );
  let t = lam(lam(b));
  app(app(t, church(n)), church(m))
}

fn main() {
  // let id = Lam(Box::new(Var(0)));
  // println!("{}", to_string(&id));
  // let c2 = church(2);
  // println!("{}", to_string(&c2));
  // let c3 = church(3);
  // let c8 = eval(&app(c3, c2));
  // println!("{}", to_string(&c8));

  let child = thread::Builder::new().stack_size(1024 * 1024 * 1024).spawn(move || {
        return eval(&nmpair(7, 6));
    }).unwrap();
  let _nm = child.join().unwrap();
  // println!("{}", to_string(&nm)); // no need to print it to evaluate it, it's an eager language
  println!("Done!");
}
