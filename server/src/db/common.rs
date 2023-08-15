pub trait FromRow {
    fn from_row(row: tokio_postgres::row::Row) -> Self;
    fn from_rows(rows: Vec<tokio_postgres::row::Row>) -> Vec<Self>
    where
        Self: Sized,
    {
        rows.into_iter().map(|row| Self::from_row(row)).collect()
    }
}
