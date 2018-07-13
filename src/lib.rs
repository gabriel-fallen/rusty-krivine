// The engine

use std::rc::Rc;

pub enum Term {
  Var(u32),
  Lam(Box<Term>),
  App(Box<Term>, Box<Term>),
  Free(String), // FIXME: `&str`?
}

use Term::*;

pub fn to_string(t: &Term) -> String {
  match t {
    Var(i) => i.to_string(),
    Lam(t1) => String::from("λ.") + &to_string(t1),
    App(u, v) => String::from("(") + &to_string(u) + &String::from(") ") + &to_string(v),
    Free(name) => name.clone()
  }
}

// Let's fix certain names for particular lifetimes.
// Let `'a` denote a lifetime of original lambda-term (the longest lifetime)

enum Env<'a> {
  Nil,
  Env(Closure<'a>, Rc<Env<'a>>),
  Lift(Rc<Env<'a>>)
}

use Env::*;

fn level<'a>(e: &Rc<Env<'a>>) -> u32 {
  match **e {
    Nil => 0,
    Env(_, ref p) => level(p),
    Lift(ref p) => level(p)
  } 
}


struct Closure<'a> {
  term: &'a Box<Term>,
  env: Rc<Env<'a>>
}

impl<'a> Clone for Closure<'a> {
  fn clone(&self) -> Closure<'a> {
    Closure {
      term: self.term,
      env: Rc::clone(&self.env) // FIXME: for this machine we can just transfer ownership
    }
  }
}


type Stack<'a> = Vec<Closure<'a>>;


fn eval_aux<'a, 'b>(t: &'a Box<Term>, e: Rc<Env<'a>>, s: &'b mut Stack<'a>) -> Box<Term> {
  match **t {
    Var(i) => {
      match fetch(i, &e) {
        Some(closure) => eval_aux(closure.term, Rc::clone(&closure.env), s),
        None => {
          if s.len() == 0 {
            Box::new(Var(i + level(&e)))
          } else {
            let mut v = Vec::new();
            s.iter().fold(Box::new(Var(i)), |a, c| Box::new(App(a, eval_aux(c.term, Rc::clone(&c.env), &mut v))))
          }
        }
      }
    },
    Lam(ref t1) => {
      match s.pop() {
        Some(c) => eval_aux(t1, Rc::new(Env(c, e)), s),
        None => Box::new(Lam(eval_aux(t1, Rc::new(Lift(e)), s)))
      }
    },
    App(ref u, ref v) => {
      s.push(Closure{term: v, env: Rc::clone(&e)});
      eval_aux(&u, e, s)
    },
    Free(ref name) => {
      if s.len() == 0 {
        Box::new(Free(name.clone())) // FIXME: can we reuse `t` here?
      } else {
        // foldl (|a, c| App(a, eval_aux(c.term, c.env, Vec::new()))) (Free(name)) s
        let mut v = Vec::new();
        s.iter().fold(Box::new(Free(name.clone())), |a, c| Box::new(App(a, eval_aux(c.term, Rc::clone(&c.env), &mut v))))
      }
    }
  }
}

fn fetch<'a, 'b>(i: u32, e: &'b Rc<Env<'a>>) -> Option<Closure<'a>> {
  match **e {
    Nil => None,
    Env(ref c, ref p) => {
      if i == 0 {
        Some(c.clone())
      } else {
        fetch(i-1, p)
      }
    },
    Lift(ref p) => {
      if i == 0 {
        None
      } else {
        match fetch(i, p) {
          Some(c) => Some(Closure {term: c.term, env: c.env}),
          None => None
        }
      }
    }
  }
}

pub fn eval(t: &Box<Term>) -> Box<Term> {
  let mut s = Vec::new();
  eval_aux(t, Rc::new(Nil), &mut s)
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
