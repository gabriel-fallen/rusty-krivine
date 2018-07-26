// The engine

use std::rc::Rc;

#[derive(Clone)]
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


enum Env {
  Nil,
  Env(Closure, Rc<Env>),
  Lift(Rc<Env>)
}

use Env::*;

fn level(e: &Rc<Env>) -> u32 {
  match **e {
    Nil => 0,
    Env(_, ref p) => level(p),
    Lift(ref p) => 1 + level(p)
  } 
}


enum Closure {
  NilClosure(Rc<Env>),
  Closure(Box<Term>, Rc<Env>)
}

use Closure::*;

impl Clone for Closure {
  fn clone(&self) -> Closure {
    match self {
      NilClosure(ref e) => NilClosure(Rc::clone(e)),// FIXME: for this machine we can just transfer ownership
      Closure(term, env) => Closure(
        term.clone(),
        Rc::clone(env) // FIXME: for this machine we can just transfer ownership
      )
    }
  }
}

type Stack = Vec<Closure>;


fn eval_aux<'a, 'b>(mut t: Box<Term>, mut e: Rc<Env>, s: &'b mut Stack) -> Box<Term> {
  loop {
    match {*t} {
      Var(i) => {
        match fetch(i, &e) {
          Some(closure) => match closure {
            Closure(term, env) => {
              // eval_aux(term, env, s)
              t = term;
              e = env;
              continue;
            },
            NilClosure(env) => {
              // here we unroll `eval_aux(Var(0), Nil, s)`
              if s.len() == 0 {
                return var(level(&env))
              } else {
                return s.drain(..).rev().fold(var(level(&env)), |a, c| {
                  if let Closure(term, env) = c {
                    // app(a, eval_aux(term, Rc::clone(env), &mut Vec::new())) // FIXME: do we still need Rc::clone here?
                    app(a, eval_aux(term, env, &mut Vec::new()))
                  } else {
                    panic!("NilClosure!")
                  }
                })
              }
            }
          },
          None => {
            if s.len() == 0 {
              return var(i + level(&e))
            } else {
              return s.drain(..).rev().fold(var(i + level(&e)), |a, c| {
                if let Closure(term, env) = c {
                  // app(a, eval_aux(term, Rc::clone(env), &mut Vec::new())) // FIXME: do we still need Rc::clone here?
                  app(a, eval_aux(term, env, &mut Vec::new()))
                } else {
                  panic!("NilClosure!")
                }
              })
            }
          }
        }
      },
      Lam(t1) => {
        match s.pop() {
          Some(c) => {
            // eval_aux(t1, Rc::new(Env(c, e)), s)
            t = t1;
            e = Rc::new(Env(c, e));
            continue;
          },
          None => return lam(eval_aux(t1, Rc::new(Env(NilClosure(Rc::new(Nil)), Rc::new(Lift(e)))), s))
        }
      },
      App(u, v) => {
        s.push(Closure(v, Rc::clone(&e)));
        // eval_aux(&u, e, s)
        t = u;
        continue;
      },
      Free(name) => {
        if s.len() == 0 {
          return Box::new(Free(name)) // FIXME: can we reuse `t` here?
        } else {
          // foldl (|a, c| App(a, eval_aux(c.term, c.env, Vec::new()))) (Free(name)) s
          return s.drain(..).rev().fold(Box::new(Free(name)), |a, c| {
            if let Closure(term, env) = c {
              // app(a, eval_aux(term, Rc::clone(env), &mut Vec::new())) // FIXME: do we still need Rc::clone here?
              app(a, eval_aux(term, env, &mut Vec::new()))
            } else {
              panic!("NilClosure!")
            }
          })
        }
      }
    }
  }
}

fn fetch<'b>(i: u32, e: &'b Rc<Env>) -> Option<Closure> {
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

pub fn eval(t: Box<Term>) -> Box<Term> {
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
