use crate::servers::Config;
use std::fs;
use std::path::Path;

fn build_part(path: String) -> reqwest::multipart::Part {
    let filename = Path::new(&path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    reqwest::multipart::Part::bytes(fs::read(path).unwrap()).file_name(filename)
}

pub async fn send_post(config: &Config, local_path: String, debug: bool) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut form = reqwest::multipart::Form::new();
    let mut headers = reqwest::header::HeaderMap::new();
    for (k, v) in config.form.iter() {
        form = form.text(k.clone(), v.clone());
    }
    form = form.part(config.form_file.clone(), build_part(local_path));
    for (k, v) in config.headers.iter() {
        headers.insert(
            reqwest::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
            v.as_str().parse().unwrap(),
        );
    }

    let res = client
        .post(config.url.clone())
        .headers(headers)
        .multipart(form)
        .send()
        .await
        .unwrap();

    if debug {
        println!("{:#?}\n", res);
    }

    let json = res.text().await.unwrap();

    if debug {
        println!("{:#?}\n", json);
    }

    if let Ok(resp) = json::parse(&json) {
        if config.name == "uploadcc" {
            match resp["success_image"][0]["url"].as_str() {
                Some(val) => {
                    return Ok(format!("https://upload.cc/{}", val));
                }
                None => {
                    return Err("Unable to upload file to upload.cc".to_string());
                }
            }
        } // special check for upload.cc

        let mut out = resp.clone();
        for next in config.response.iter() {
            out = out[next].clone().to_owned();
        }
        match out.as_str() {
            Some(o) => return Ok(o.to_string()), // the server response a success info json
            None => {
                // cannot get success info from responsed json
                let mut out = resp;
                for next in config.message.iter() {
                    out = out[next].clone().to_owned();
                }
                // try to get error message from json
                match out.as_str() {
                    Some(msg) => return Err(msg.to_string()),
                    None => Err(String::from(
                        "Fail to get response, please check the config file",
                    )),
                }
            }
        }
    } else {
        if config.response.len() == 0 && json.starts_with("http") {
            Ok(json)
        } else {
            Err(String::from(format!(
                "Fail to parse json response\n Got: {}",
                json
            )))
        }
    }
}
