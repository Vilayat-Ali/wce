pub trait DBModel
where
    Self: Clone + Send + Sync,
{
    type ServiceError;
    type CreatePayload;
    type CreateResponse;

    // create a new record in the database table/collection
    fn create(
        &self,
        data: Self::CreatePayload,
    ) -> impl std::future::Future<Output = Result<Self::CreateResponse, Self::ServiceError>> + Send
    where
        Self::CreatePayload: std::marker::Send;
}
