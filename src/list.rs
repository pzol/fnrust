
#[deriving(PartialEq, Show, Clone)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

impl<'a, T:Clone> List<T> {
  fn empty() -> List<T> {
    Nil
  }

  fn append(self, x: T) -> List<T> {
    return Cons(x, box self)
  }

  fn len(&self) -> uint {
    match *self {
      Cons(_, ref xs) => 1 + xs.len(),
      Nil             => 0
    }
  }

  fn null(&self) -> bool {
    match *self {
      Cons(_, _) => false,
      Nil        => true
    }
  }

  fn first(&'a self) -> Option<&'a T> {
    match *self {
      Cons(ref x, _) => Some(x),
      Nil            => None
    }
  }

  fn last(&'a self) -> Option<&'a T> {
    match *self {
      Cons(ref x, ref xs) if xs.null() => Some(x),
      Cons(_, ref xs)                  => xs.last(),
      Nil                              => None
    }
  }

  /// return a reference to the tail of the list
  fn tail(&self) -> &List<T> {
    match *self {
      Nil                 => self,
      Cons(_, box ref xs) => xs
    }
  }

  /// return a **new** list, with the last element removed
  fn init(&self) -> List<T> {
    match *self {
      Cons(ref x, box Cons(_, box Nil)) => Cons(x.clone(), box Nil),
      Cons(ref x, ref xs)               => Cons(x.clone(), box xs.init()),
      Nil  => Nil
    }
  }

  fn map<U>(&self, f: |&T| -> U) -> List<U> {
    match *self {
      Cons(ref x, ref xs) => Cons(f(x), box xs.map(f)),
      Nil => Nil
    }
  }

  fn filter(&self, f: |&T| -> bool) -> List<T> {
    match *self {
      Cons(ref x, ref xs) if f(x) => Cons(x.clone(), box xs.filter(f)),
      Cons(_, ref xs)             => xs.filter(f),
      Nil => Nil
    }
  }

  fn find(&self, f: |&T| -> bool) -> Option<&T> {
    match *self {
      Nil                    => None,
      Cons(ref x, _) if f(x) => return Some(x),
      Cons(_, ref xs)        => xs.find(f)
    }
  }

  fn foldl(&self, z: T, f: |&T, &T| -> T) -> T {
    match *self {
      Nil => z,
      Cons(ref x, ref xs) => xs.foldl(f(&z, x), f)
    }
  }

  // def foldl1(&fn)
  //   match {
  //     Nil() { raise EmptyListError }
  //     Cons(h, t) { t.foldl(h, &fn)}
  //   }
  // end

  // def foldr(start, &fn)
  //   match {
  //     Nil() { start }
  //     # foldr f z (x:xs) = f x (foldr f z xs)
  //     Cons(h, t) { fn.(h, t.foldr(start, &fn)) }
  //   }
  // end

}

fn test_list() -> List<uint> {
  return Nil.append(21u).append(15).append(9)
}

#[test]
fn test_null() {
    let empty : List<uint> = List::empty();
    let full  = List::empty().append(1u);

    assert!(empty.null());
    assert!(!full.null());
}

#[test]
fn test_new() {
    let list : List<uint> = List::empty();
    match list {
      Cons(_, _) => fail!("Nil expected, got Cons"),
      Nil     => ()
    }
}

#[test]
fn test_first() {
  let list = Nil.append(1u);
  let    n = list.first().unwrap();
  assert_eq!(*n, 1);

  let empty : List<uint> = List::empty();
  assert_eq!(empty.first(), None);
}

#[test]
fn test_append() {
  let l = Nil.append(1u);
  println!("{}", l);
  assert_eq!(l.len(), 1);
}

#[test]
fn test_last() {
  let list = test_list();
  let last = list.last().unwrap();

  assert_eq!(*last, 21u);
}

#[test]
fn test_tail() {
  let list = test_list();
  let tail = list.tail();
  let expected = Nil.append(21u).append(15);
  assert_eq!(*tail, expected);

  assert_eq!(*tail.first().unwrap(), 15);
  assert_eq!(*tail.last().unwrap(), 21);
}

#[test]
fn test_init() {
  let list = test_list();
  let init = list.init();
  println!("{}", list);

  let expected = Nil.append(15u).append(9);
  assert_eq!(init, expected);
}

#[test]
fn test_map() {
  let list     = test_list();
  let expected = Nil.append(22u).append(16).append(10);
  let actual   = list.map(|n| n + 1);
  assert_eq!(actual, expected);
}

#[test]
fn test_filter() {
  let list   = test_list();
  let expected = Nil.append(21u).append(9);
  let actual = list.filter(|&n| n != 15);
  assert_eq!(actual, expected);

  assert!(list.filter(|&n| n == 1).null());
}

#[test]
fn test_find() {
  let list   = test_list();
  let actual = list.find(|&n| n == 15).unwrap();
  assert_eq!(*actual, 15);

  assert!(list.find(|&n| n == 1).is_none());
}

#[test]
fn test_foldl() {
  let list     = test_list();
  let actual   = list.foldl(0, |&b: &uint, &a: &uint| b + a);

  assert_eq!(actual, (((0u + 21) + 15) + 9));

  let int_list : List<int> = list.map(|&n| n as int);
  let actual = int_list.foldl(0i, |&b: &int, &a: &int| b - a);
  assert_eq!(actual, (((0i - 21) - 15) - 9));
}
