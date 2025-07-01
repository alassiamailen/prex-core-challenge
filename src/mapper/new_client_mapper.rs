use crate::dto::new_client_dto::NewClientDto;
use crate::model::client_model::Client;

/// Map [NewClientDto] to [Client] from [NewClientDto] new_client and [i32] client_id
pub fn map_new_client_to_client(new_client: NewClientDto, client_id:i32) -> Client{
    Client{
        client_id,
        client_name: new_client.client_name,
        birth_date: new_client.birth_date,
        document_number: new_client.document_number,
        country: new_client.country,
        credit_amount: None,
    }
}