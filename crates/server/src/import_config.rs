use crate::protocol::ImportPayload;
use thiserror::Error;
use url::Url;

const IMPORT_SCHEME_URL: &str = "vibecodingremote://import";
const IMPORT_PAYLOAD_VERSION: u8 = 1;

#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub import_url: String,
}

#[derive(Debug, Error)]
pub enum ImportConfigError {
    #[error("failed to build import URL: {0}")]
    BuildUrl(#[source] url::ParseError),
}

impl ImportConfig {
    pub fn new(endpoint: String, token: String) -> Result<Self, ImportConfigError> {
        let payload = ImportPayload {
            version: IMPORT_PAYLOAD_VERSION,
            endpoint,
            token,
        };
        let import_url = build_import_url(&payload)?;

        Ok(Self { import_url })
    }
}

pub fn build_import_url(payload: &ImportPayload) -> Result<String, ImportConfigError> {
    let mut url = Url::parse(IMPORT_SCHEME_URL).map_err(ImportConfigError::BuildUrl)?;
    url.query_pairs_mut()
        .append_pair("v", &payload.version.to_string())
        .append_pair("endpoint", &payload.endpoint)
        .append_pair("token", &payload.token);

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_url_matches_the_mobile_scanner_contract() {
        let config = ImportConfig::new(
            "http://192.168.1.23:8765".to_owned(),
            "test-token".to_owned(),
        )
        .expect("import URL should build");
        let url = Url::parse(&config.import_url).expect("import URL should parse");

        assert_eq!(url.scheme(), "vibecodingremote");
        assert_eq!(url.host_str(), Some("import"));
        assert_eq!(
            url.query_pairs().find(|(key, _)| key == "v").unwrap().1,
            "1"
        );
        assert_eq!(
            url.query_pairs()
                .find(|(key, _)| key == "endpoint")
                .unwrap()
                .1,
            "http://192.168.1.23:8765"
        );
        assert_eq!(
            url.query_pairs().find(|(key, _)| key == "token").unwrap().1,
            "test-token"
        );
    }
}
