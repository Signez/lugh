#[cfg(test)]
mod tests {
    use super::super::common::*;

    use actix_web::http::StatusCode;
    use serde_json;

    #[derive(Deserialize)]
    struct Configuration {
        locales: Vec<String>,
    }

    #[actix_rt::test]
    async fn test_get_configuration_without_token() {
        let (response, content) = get("/api/v1/configuration", None).await;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
        assert_eq!("", content)
    }

    #[actix_rt::test]
    async fn test_get_configuration_with_success() {
        let (response, content) = get("/api/v1/configuration", valid_token()).await;

        assert_eq!(StatusCode::OK, response.status());

        let configuration = parse_configuration(&content);

        assert_eq!(5, configuration.locales.len());
    }

    fn parse_configuration(content: &str) -> Configuration {
        serde_json::from_str(content).expect("Can’t parse configuration from content")
    }
}
