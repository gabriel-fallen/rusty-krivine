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

fn klmn(k: u8, l: u8, m: u8, n: u8) -> Box<Term> {
  // t = \x1.\x2.\x3.\x4.((((a)(x1)x2) (b)(x2)x1) (c)(x3)x4) (d)(x4)x3
  let v = vec![app(var(3), var(2)),
               app(Box::new(Free("b".to_string())), app(var(2), var(3))),
               app(Box::new(Free("c".to_string())), app(var(1), var(0))),
               app(Box::new(Free("d".to_string())), app(var(0), var(1)))];
  let b = v.into_iter().fold(Box::new(Free("a".to_string())), |a, c| app(a, c));
  let t = lam(lam(lam(lam( b ))));
  app(app(app(app(t, church(k)), church(l)), church(m)), church(n))
}


fn run<T, F>(f: F) -> T
  where T: 'static + Send,
        F: 'static + Fn() -> T + Send {
  let child = thread::Builder::new().stack_size(2048 * 1024 * 1024).spawn(f).unwrap();
  return child.join().unwrap();
}

fn main() {
  // let id = Lam(Box::new(Var(0)));
  // println!("{}", to_string(&id));
  // let c2 = church(2);
  // println!("{}", to_string(&c2));
  // let c3 = church(3);
  // let c8 = eval(&app(c3, c2));
  // println!("{}", to_string(&c8));

  // let _nm = run(move || {
  //       return eval(&nmpair(7, 6));
  //   });
  // println!("{}", to_string(&nm)); // no need to print it to evaluate it, it's an eager language
  let _klmn = run(move || {
        return eval(klmn(7, 6, 6, 7));
    });
  println!("Done!");
}
