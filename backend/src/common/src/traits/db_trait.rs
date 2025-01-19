use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ModelFieldValue {
    IntVal(i32),
    FloatVal(f32),
    TextVal(String),
    BoolVal(bool),
}

pub trait DBModel
where
    Self: Clone + Send + Sync,
{
    type ServiceError;
    type Id;
    type Schema;
    type Config;

    type CreatePayload;
    type CreateResponse;

    // create a new record in the database table/collection
    fn create(
        &self,
        data: Self::CreatePayload,
        config: Option<&Self::Config>,
    ) -> impl std::future::Future<Output = Result<Self::CreateResponse, Self::ServiceError>> + Send
    where
        Self::CreatePayload: std::marker::Send;

    // get a record by id
    fn read_by_id(
        &self,
        id: Self::Id,
    ) -> impl std::future::Future<Output = Result<Self::Schema, Self::ServiceError>> + Send
    where
        Self::Id: std::marker::Send;

    // update a record by id
    fn update_by_id(
        &self,
        id: Self::Id,
        update_data: HashMap<String, ModelFieldValue>,
    ) -> impl std::future::Future<Output = Result<(), Self::ServiceError>> + Send
    where
        Self::Id: std::marker::Send;

    // delete a record by id
    fn delete_by_id(
        &self,
        id: Self::Id,
    ) -> impl std::future::Future<Output = Result<(), Self::ServiceError>> + Send
    where
        Self::Id: std::marker::Send;
}
