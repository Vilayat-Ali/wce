use serde::{de::DeserializeOwned, Serialize};

pub trait DBModel<T>
where
    T: Serialize + DeserializeOwned + std::fmt::Debug + Send + 'static,
{
    // create a new record in the database table/collection
    fn create<R>(
        &self,
        data: T,
    ) -> impl std::future::Future<Output = Result<R, Box<dyn std::error::Error>>> + Send;

    // create many records in the database table/collection
    fn create_many<R>(
        &self,
        data: Vec<T>,
    ) -> impl std::future::Future<Output = Result<R, Box<dyn std::error::Error>>> + Send;

    // read a record in the database table/collection by Id
    fn read_by_id<I, R>(
        &self,
        id: I,
    ) -> impl std::future::Future<Output = Result<R, Box<dyn std::error::Error>>> + Send;

    // update a record in the database table/collection by Id
    fn update_by_id<I, R>(
        &self,
        id: I,
    ) -> impl std::future::Future<Output = Result<R, Box<dyn std::error::Error>>> + Send;

    // soft delete a record in the database table/collection by Id
    fn soft_delete_by_id<I, R>(
        &self,
        id: I,
    ) -> impl std::future::Future<Output = Result<R, Box<dyn std::error::Error>>> + Send;
}
