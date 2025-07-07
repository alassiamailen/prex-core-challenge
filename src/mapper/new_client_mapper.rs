use crate::dto::new_client_dto::NewClient;
use crate::model::client_model::Client;
use rust_decimal::Decimal;

/// Maps an [Client] from [NewClient] and [i32] client_id
pub fn map_client(new_client: NewClient, client_id: i32) -> Client {
    Client {
        client_id,
        client_name: new_client.client_name,
        birth_date: new_client.birth_date,
        document_number: new_client.document_number,
        country: new_client.country,
        balance: Decimal::new(0, 0),
    }
}

/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::dto::new_client_dto::NewClient;
    use crate::mapper::new_client_mapper::map_client;
    use crate::stub::client_info_stub::stub::create_client_info_stub;
    use crate::stub::new_client_stub::stub::create_new_client_stub;

    /// Scenario:
    /// Executes map_client with valid parameters
    /// Expectation:
    /// A [Client] should be returned
    #[tokio::test]
    async fn when_map_client_should_return_client_struct() {
        let expected_client = create_new_client_stub();
        let session_id = create_client_info_stub().client_id;
        let new_client_request = NewClient {
            client_name: expected_client.client_name.clone(),
            birth_date: expected_client.birth_date,
            document_number: expected_client.document_number.clone(),
            country: expected_client.country.clone(),
        };
        let result = map_client(new_client_request, session_id);

        assert_eq!(session_id, result.client_id);
        assert_eq!(expected_client.client_name, result.client_name);
        assert_eq!(expected_client.birth_date, result.birth_date);
        assert_eq!(expected_client.document_number, result.document_number);
        assert_eq!(expected_client.country, result.country);
    }
}
