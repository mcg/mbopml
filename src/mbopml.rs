use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;
use std::env;
use std::process;
use xml::writer::{EmitterConfig, XmlEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Please provide the Micro.blog API key, username, and format using the --api-key, --username, and --format arguments.");
        process::exit(1);
    }

    let api_key = &args[1];
    let username = &args[2];
    let format = &args[3];

    let url = format!("https://micro.blog/users/following/{}", username);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key))?);

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;

    if response.status().is_success() {
        let following_list: Value = response.json().await?;
        let opml = generate_opml(following_list, format);
        println!("{}", opml);
    } else {
        println!("Failed to download following list. Status code: {}", response.status());
    }

    Ok(())
}

fn generate_opml(following_list: Value, format: &str) -> String {
    let mut buffer = Vec::new();
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut buffer);

    writer.write(XmlEvent::start_element("opml")).unwrap();
    writer.write(XmlEvent::start_element("head")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // end head
    writer.write(XmlEvent::start_element("body")).unwrap();

    for user in following_list.as_array().unwrap() {
        let name = user["name"].as_str().unwrap();
        let username = user["username"].as_str().unwrap();
        let xml_url = match format {
            "xml" => format!("https://{}.micro.blog/feed.xml", username),
            "json" => format!("https://{}.micro.blog/feed.json", username),
            _ => String::new(),
        };

        writer.write(XmlEvent::start_element("outline")).unwrap();
        writer.write(XmlEvent::attribute("text", name)).unwrap();
        writer.write(XmlEvent::attribute("type", "rss")).unwrap();
        writer.write(XmlEvent::attribute("title", name)).unwrap();
        writer.write(XmlEvent::attribute("xmlUrl", &xml_url)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap(); // end outline
    }

    writer.write(XmlEvent::end_element()).unwrap(); // end body
    writer.write(XmlEvent::end_element()).unwrap(); // end opml

    String::from_utf8(buffer).unwrap()
}
