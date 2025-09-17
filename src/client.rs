use anyhow::anyhow;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT};
use reqwest::Certificate;
use std::time::Duration;

const SAFARI_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15";

/// taken from https://letsencrypt.org/certificates/

const LETS_ENCRYPT_ROOT_AND_R10_THROUGH_R14: &[u8] = include_bytes!("cooltext.pem");

/// Returns a [HeaderMap] preloaded with the Safari user agent.
fn construct_headers() -> HeaderMap {
	let mut headers = HeaderMap::new();
	headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
	headers.insert(ORIGIN, "https://cooltext.com".parse().unwrap());
	headers.insert(REFERER, "https://cooltext.com/Logo-Design-Burning".parse().unwrap());
	headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
	headers.insert(USER_AGENT, HeaderValue::from_static(SAFARI_UA));
	headers
}

pub(crate) fn create_client() -> Result<reqwest::blocking::Client> {
	let mut builder = reqwest::blocking::Client::builder()
		.use_rustls_tls()
		.default_headers(construct_headers())
		.timeout(Duration::from_secs(10));

	let certs = Certificate::from_pem_bundle(LETS_ENCRYPT_ROOT_AND_R10_THROUGH_R14)?;
	for cert in certs {
		builder = builder.add_root_certificate(cert);
	}

	// for cert in LETS_ENCRYPT_CERTS {
	// 	builder = builder.add_root_certificate(Certificate::from_pem(cert)?);
	// }

	builder.build()
		.map_err(|e| {
			eprintln!("Couldn't initialize client: {e:?}");
			anyhow!("Couldn't initiate request!")
		})
}