use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait FromRow {
    fn from_row(row: &tokio_postgres::row::Row) -> Self;
    fn from_rows(rows: &Vec<tokio_postgres::row::Row>) -> Vec<Self>
    where
        Self: Sized,
    {
        rows.into_iter().map(|row| Self::from_row(row)).collect()
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Uuid<T> {
    inner: uuid::Uuid,
    phantom: PhantomData<T>,
}

impl<T> Uuid<T> {
    pub fn inner(&self) -> uuid::Uuid {
        self.inner
    }
    pub fn new(uuid: uuid::Uuid) -> Self {
        Uuid {
            inner: uuid,
            phantom: PhantomData,
        }
    }
}

impl<T> PartialEq for Uuid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for Uuid<T> {}
