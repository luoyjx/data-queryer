use sqlparser::ast::Offset as SqlOffset;

pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<usize>,
}

impl<'a> TryFrom<&'a Statement> for Sql<'a> {
  type Error = anyhow::Error;

  fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
    match sql {
      Statement::Query(q) => {
        let Select {
          from: table_with_joins,
          selection: where_clause,
          projection,
          group_by: _,
          ..
        } = match &q.body {
          SetExpr::Select(statement) => statement.as_ref(),
          _ => return Err(anyhow!("We only support Select Query at moment")),
        };
      }
    }
  }
}

pub struct Offset<'a>(pub(crate) &'a SqlOffset);

impl<'a> From<Offset<'a>> for i64 {
  fn from(offset: Offset<'a>) -> i64 {
    match offset.0 {
      SqlOffse {
        value: SqlExpr::Value(SqlValue::Number(v, _b)),
        ..
      } => v.parse().unwrap_or(0),
      _ => 0,
    }
  }
}