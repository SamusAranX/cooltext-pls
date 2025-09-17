use anyhow::anyhow;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT};
use reqwest::{Certificate, IntoUrl, Response};
use serde::Serialize;
use std::fs::File;
use std::io::copy;
use std::path::Path;
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

#[allow(dead_code)]
pub trait ClientExtras {
	async fn get_req<U: IntoUrl>(&self, url: U) -> Result<Response>;

	async fn post_form<U: IntoUrl, T: Serialize + ?Sized>(&self, url: U, form: &T) -> Result<Response>;

	/// Downloads a file to a specified [File].
	async fn download_to_file<U: IntoUrl>(&self, url: U, dest: &mut File) -> Result<()>;

	/// Downloads a file to a specified path.
	#[allow(dead_code)]
	async fn download<U: IntoUrl, P: AsRef<Path>>(&self, url: U, dest: P) -> Result<()>;
}

impl ClientExtras for reqwest::Client {
	async fn get_req<U: IntoUrl>(&self, url: U) -> Result<Response> {
		reqwest::Client::new()
			.get(url)
			.send().await
			.map_err(|e| {
				if e.is_timeout() {
					eprintln!("timeout sending GET request: {e:#?}");
					anyhow!("The server took too long to answer.")
				} else {
					eprintln!("error sending GET request: {e:#?}");
					anyhow!("Couldn't send the request.")
				}
			})?
			.error_for_status()
			.map_err(|e| {
				eprintln!("GET response error: {e:#?}");
				anyhow!("The server returned an error.")
			})
	}

	async fn post_form<U: IntoUrl, T: Serialize + ?Sized>(&self, url: U, form: &T) -> Result<Response> {
		reqwest::Client::new()
			.post(url)
			.form(form)
			.send().await
			.map_err(|e| {
				if e.is_timeout() {
					eprintln!("timeout sending POST form request: {e:#?}");
					anyhow!("The server took too long to answer.")
				} else {
					eprintln!("error sending POST form request: {e:#?}");
					anyhow!("Couldn't send the request.")
				}
			})?
			.error_for_status()
			.map_err(|e| {
				eprintln!("POST form response error: {e:#?}");
				anyhow!("The server returned an error.")
			})
	}

	async fn download_to_file<U: IntoUrl>(&self, url: U, dest: &mut File) -> Result<()> {
		let resp = self.get_req(url).await?;
		let bytes = resp.bytes().await
			.map_err(|e| {
				eprintln!("download_to_file bytes error: {e}");
				anyhow!("Couldn't retrieve response data.")
			})?;

		copy(&mut bytes.iter().as_slice(), dest)
			.map_err(|e| {
				eprintln!("download_to_file file copy error: {e}");
				anyhow!("Couldn't store response data.")
			})?;

		Ok(())
	}

	async fn download<U: IntoUrl, P: AsRef<Path>>(&self, url: U, dest: P) -> Result<()> {
		let mut dest = File::create(dest)?;
		self.download_to_file(url, &mut dest).await
	}
}