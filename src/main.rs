use crate::client::create_client;
use crate::structs::{BurningTextForm, CoolTextResponse};
use rand::distr::{Alphanumeric, SampleString};

mod client;
mod structs;

fn main() {
    // let urls = vec![
    //     "https://r73.cooltext.com/rendered/cooltext491629871094159.gif",
    //     "https://r74.cooltext.com/rendered/cooltext491635339442129.gif",
    //     "https://r76.cooltext.com/rendered/cooltext491632146919683.gif",
    // ];

    let client = create_client().unwrap();
    let mut rng = rand::rng();

    for _ in 0..5 {
        let string = Alphanumeric.sample_string(&mut rng, 16);
        let form = BurningTextForm::new(string);

        let response = client
            .post("https://cooltext.com/PostChange")
            .form(&form)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let response: CoolTextResponse = serde_json::from_str(&response).unwrap();

        let gif_data = client
            .get(response.render_location)
            .send()
            .unwrap()
            .bytes()
            .unwrap();

        eprintln!("got {} bytes of gif data", gif_data.len())
    }
}
