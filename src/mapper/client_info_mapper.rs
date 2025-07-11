use crate::dto::client_info_dto::ClientInfo;
use crate::model::client_model::Client;

/// Maps an [ClientInfo] from [Client] and [i32] client_id
pub fn map_client_info(client: Client) -> ClientInfo {
    ClientInfo {
        client_id: client.client_id,
        client_name: client.client_name,
        birth_date: client.birth_date,
        document_number: client.document_number,
        country: client.country,
        balance: client.balance,
    }
}
/// Unit tests cases
#[cfg(test)]
mod tests {
    use crate::mapper::client_info_mapper::map_client_info;
    use crate::model::client_model::Client;
    use crate::stub::client_info_stub::stub::create_client_info_stub;

    /// Scenario:
    /// Executes map_client_info with valid parameters
    /// Expectation:
    /// A [ClientInfo] should be returned
    #[tokio::test]
    async fn when_map_client_info_should_return_client_info_struct() {
        let expected_client_info = create_client_info_stub();

        let client_request = Client {
            client_id: expected_client_info.client_id,
            client_name: expected_client_info.client_name.clone(),
            birth_date: expected_client_info.birth_date,
            document_number: expected_client_info.document_number.clone(),
            country: expected_client_info.country.clone(),
            balance: expected_client_info.balance,
        };

        let result = map_client_info(client_request);

        assert_eq!(expected_client_info.client_id, result.client_id);
        assert_eq!(expected_client_info.client_name, result.client_name);
        assert_eq!(expected_client_info.birth_date, result.birth_date);
        assert_eq!(expected_client_info.document_number, result.document_number);
        assert_eq!(expected_client_info.country, result.country);
        assert_eq!(expected_client_info.balance, result.balance);
    }
}
