use anyhow::anyhow;
use crate::client::create_client;

mod client;

fn main() {
    let urls = vec![
        "https://r73.cooltext.com/rendered/cooltext491629871094159.gif",
        "https://r74.cooltext.com/rendered/cooltext491635339442129.gif",
        "https://r76.cooltext.com/rendered/cooltext491632146919683.gif",
    ];

    let client = create_client().unwrap();

    for url in urls {
        let gif_data = client
            .get(url)
            .send()
            .unwrap()
            .bytes()
            .map_err(|e| {
                eprintln!("error downloading cooltext GIF: {e:#?}");
                anyhow!("Couldn't retrieve the generated image.")
            })
            .unwrap();

        eprintln!("got {} bytes of gif data from {url}", gif_data.len())
    }


}
