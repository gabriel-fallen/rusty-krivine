// The engine

use std::rc::Rc;

pub enum Term {
  Var(u32),
  Lam(Box<Term>),
  App(Box<Term>, Box<Term>),
  Free(String), // FIXME: `&str`?
}

use Term::*;

// Smart constructors

pub fn var(n: u32) -> Box<Term> {
  Box::new(Var(n))
}

pub fn lam(t: Box<Term>) -> Box<Term> {
  Box::new(Lam(t))
}

pub fn app(u: Box<Term>, v: Box<Term>) -> Box<Term> {
  Box::new(App(u, v))
}

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
    Lift(ref p) => 1 + level(p)
  } 
}


enum Closure<'a> {
  NilClosure(Rc<Env<'a>>),
  Closure(&'a Box<Term>, Rc<Env<'a>>)
}

use Closure::*;

impl<'a> Clone for Closure<'a> {
  fn clone(&self) -> Closure<'a> {
    match self {
      NilClosure(ref e) => NilClosure(Rc::clone(e)),// FIXME: for this machine we can just transfer ownership
      Closure(ref term, ref env) => Closure(
        term,
        Rc::clone(env) // FIXME: for this machine we can just transfer ownership
      )
    }
  }
}

impl<'a> Closure<'a> {
  fn term(&self) -> &'a Box<Term> {
    match self {
      Closure(ref t, _) => t,
      NilClosure(_) => panic!("NilClosure.term()")
    }
  }
  fn env(&self) -> &Rc<Env<'a>> {
    match self {
      Closure(_, env) => env,
      NilClosure(_) => panic!("NilClosure.term()")
    }
  }
}


type Stack<'a> = Vec<Closure<'a>>;


fn eval_aux<'a, 'b>(t: &'a Box<Term>, e: Rc<Env<'a>>, s: &'b mut Stack<'a>) -> Box<Term> {
  match **t {
    Var(i) => {
      match fetch(i, &e) {
        Some(closure) => match closure {
          Closure(term, env) => eval_aux(term, env, s),
          NilClosure(env) => {
            // here we unroll `eval_aux(Var(0), Nil, s)`
            if s.len() == 0 {
              var(level(&env))
            } else {
              s.iter().rev().fold(var(level(&env)), |a, c| app(a, eval_aux(c.term(), Rc::clone(c.env()), &mut Vec::new())))
            }
          }
        },
        None => {
          if s.len() == 0 {
            var(i + level(&e))
          } else {
            s.iter().rev().fold(var(i + level(&e)), |a, c| app(a, eval_aux(c.term(), Rc::clone(c.env()), &mut Vec::new())))
          }
        }
      }
    },
    Lam(ref t1) => {
      match s.pop() {
        Some(c) => eval_aux(t1, Rc::new(Env(c, e)), s),
        None => lam(eval_aux(t1, Rc::new(Env(NilClosure(Rc::new(Nil)), Rc::new(Lift(e)))), s))
      }
    },
    App(ref u, ref v) => {
      s.push(Closure(v, Rc::clone(&e)));
      eval_aux(&u, e, s)
    },
    Free(ref name) => {
      if s.len() == 0 {
        Box::new(Free(name.clone())) // FIXME: can we reuse `t` here?
      } else {
        // foldl (|a, c| App(a, eval_aux(c.term, c.env, Vec::new()))) (Free(name)) s
        s.iter().rev().fold(Box::new(Free(name.clone())), |a, c| app(a, eval_aux(c.term(), Rc::clone(c.env()), &mut Vec::new())))
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
      match fetch(i, p) {
        None => None,
        Some(c) => match c {
          Closure(term, env) => Some(Closure(term, Rc::new(Lift(env)))),
          NilClosure(env) => Some(NilClosure(Rc::new(Lift(env))))
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
