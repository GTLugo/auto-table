#[derive(Debug, Clone)]
pub struct Table {}

impl Table {
  pub fn builder() -> TableBuilder {
    TableBuilder::new()
  }
}

pub struct TableBuilder {}

impl Default for TableBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl TableBuilder {
  pub fn new() -> Self {
    Self {}
  }

  pub fn build(self) -> Table {
    Table {

    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn table() {
    let table = Table::builder().build();
  }
}
