use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::option::Option;

pub trait FromRow {
    fn from_row(row: &tokio_postgres::row::Row) -> Self;
    fn from_rows(rows: &[tokio_postgres::row::Row]) -> Vec<Self>
    where
        Self: Sized,
    {
        rows.iter().map(|row| Self::from_row(row)).collect()
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

impl<T> PartialOrd for Uuid<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> Ord for Uuid<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}
