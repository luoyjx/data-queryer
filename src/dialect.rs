use sqlparser::dialect::Dialect;


#[derive(Debug, Default)]
pub struct JxDialect;

impl Dialect for JxDialect {
  fn is_identifier_start(&self, ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
  }

  // identifier 允许包含 ':', '/', '?', '&', '=', '-', '.'
  fn is_identifier_part(&self, ch: char) -> bool {
    ('a'..='z').contains(&ch)
      || ('A'..='Z').contains(&ch)
      || ('0'..='9').contains(&ch)
      || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
  }
}

fn example_sql() -> String {
  let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

  let sql = format!(
    "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
    FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
    url
  );

  sql
}

#[cfg(test)]
mod tests {
  use super::*;
  use sqlparser::parser::Parser;

  #[test]
  fn it_works() {
    let sql = &example_sql();
    println!("{}", sql);
    assert!(Parser::parse_sql(&JxDialect::default(), sql).is_ok());
  }
}