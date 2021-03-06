use rational::Rational;

pub struct Source {
  s: Vec<char>,
  pos: usize,
}

impl Source {
  pub fn new(s: String) -> Source {
    Source {
      s: s.chars().collect(),
      pos: 0,
    }
  }

  fn peek(&self) -> Option<char> {
    self.s.get(self.pos).cloned()
  }

  fn next(&mut self) -> Option<char> {
    let val = self.peek();
    self.pos += 1;
    val
  }

  fn char(&mut self, c: char) -> Option<char> {
    self.expect(|x| x == c)
  }

  fn expect<F>(&mut self, f: F) -> Option<char>
  where
    F: FnOnce(char) -> bool,
  {
    match self.peek() {
      Some(x) if f(x) => {
        self.next();
        Some(x)
      }
      _ => None,
    }
  }

  fn number(&mut self) -> Option<Rational> {
    let mut s = String::new();
    let g = if let Some(_) = self.char('-') { -1 } else { 1 };

    while let Some(c) = self.expect(|c| c.is_digit(10)) {
      s.push(c);
    }

    if s.len() != 1 && s.clone().chars().nth(0) == Some('0') {
      None
    } else {
      s.parse::<i64>().ok().map(|x| Rational::from(x * g))
    }
  }

  fn eof(&self) -> Option<()> {
    if self.s.len() == self.pos {
      Some(())
    } else {
      None
    }
  }

  pub fn parse(&mut self) -> Option<Rational> {
    let x = self.expr()?;
    self.eof()?;
    Some(x)
  }

  fn expr(&mut self) -> Option<Rational> {
    let mut x = self.term()?;
    while let Some(c) = self.expect(|c| c == '+' || c == '-') {
      match c {
        '+' => {
          x += self.term()?;
        }
        '-' => {
          x -= self.term()?;
        }
        _ => {
          unreachable!();
        }
      }
    }
    Some(x)
  }

  fn term(&mut self) -> Option<Rational> {
    let mut x = self.factor()?;
    while let Some(c) = self.expect(|c| c == '*' || c == '/') {
      match c {
        '*' => {
          x *= self.factor()?;
        }
        '/' => {
          x /= self.factor()?;
        }
        _ => {
          unreachable!();
        }
      }
    }
    Some(x)
  }

  fn factor(&mut self) -> Option<Rational> {
    if let Some(_) = self.char('(') {
      let ret = self.expr();
      self.char(')')?;
      ret
    } else {
      self.number()
    }
  }
}
