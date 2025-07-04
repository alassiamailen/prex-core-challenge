use crate::dto::new_client_dto::NewClient;
use crate::model::client_model::Client;
use rust_decimal::Decimal;

/// Maps an [Client] from [NewClient] and [i32] client_id
pub fn client_map(new_client: NewClient, client_id: i32) -> Client {
    Client {
        client_id,
        client_name: new_client.client_name,
        birth_date: new_client.birth_date,
        document_number: new_client.document_number,
        country: new_client.country,
        balance: Decimal::new(0, 0),
    }
}
