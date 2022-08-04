use json::{array, JsonValue};
use std::{collections::HashMap, fs, process::exit};

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub form: HashMap<String, String>,
    pub form_file: String,
    pub response: Vec<String>,
    pub message: Vec<String>,
}

fn parse_config(parsed: JsonValue) -> Vec<Config> {
    let mut servers = Vec::new();
    for item in parsed.members().into_iter() {
        let name: String;
        let url: String;
        let mut headers = HashMap::new();
        let mut form = HashMap::new();
        let form_file: String;
        let mut response = Vec::new();
        let mut message = Vec::new();

        if let Some(s) = item["name"].as_str() {
            name = s.to_string();
        } else {
            break;
        }
        if let Some(u) = item["url"].as_str() {
            url = u.to_string();
        } else {
            break;
        }
        for (k, v) in item["headers"].entries().into_iter() {
            if let Some(val) = v.as_str() {
                headers.insert(k.to_string(), val.to_string());
            } else {
                break;
            }
        }
        for (k, v) in item["form"].entries().into_iter() {
            if let Some(val) = v.as_str() {
                form.insert(k.to_string(), val.to_string());
            } else {
                break;
            }
        }
        if let Some(u) = item["form_file"].as_str() {
            form_file = u.to_string();
        } else {
            form_file = String::from("");
        }
        for val in item["response"].members().into_iter() {
            if let Some(s) = val.as_str() {
                response.push(s.to_string());
            }
        }
        for val in item["message"].members().into_iter() {
            if let Some(s) = val.as_str() {
                message.push(s.to_string());
            }
        }

        let conf = Config {
            name,
            url,
            headers,
            form,
            form_file,
            response,
            message,
        };

        servers.push(conf);
    }

    servers
}

pub fn use_file_config(path: String) -> Vec<Config> {
    let file_content = match fs::read_to_string(&path) {
        Ok(fc) => fc,
        Err(e) => {
            eprintln!("Fail to read config file at `{}`", path);
            eprintln!("Reason: {}", e);
            exit(1);
        }
    };

    let parsed = match json::parse(&file_content) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Fail to parse config file at `{}`", path);
            eprintln!("Reason: {}", e);
            exit(1);
        }
    };
    parse_config(parsed)
}

pub fn use_build_in_config() -> Vec<Config> {
    let arr = array![
        {
            "name": "smms",
            "url": "https://sm.ms/api/v2/upload",
            "headers": {
                "Authorization": "qk0WHwBx9NOcUWyMgqMo7o6YoMRAGHTX"
            },
            "form": {},
            "form_file": "smfile",
            "response": ["data", "url"],
            "message": ["message"]
        },
        {
            "name": "imgkr",
            "url": "https://imgkr.com/api/v2/files/upload",
            "headers": {
                "Referer": "https://imgkr.com/",
                "Cookie": "antn=53af7d0925dc0b65f16e34618352750b"
            },
            "form": {},
            "form_file": "file",
            "response": ["data"],
            "message": ["message"]
        },
        {
            "name": "imgtp",
            "url": "https://imgtp.com/api/upload",
            "headers": {
                "TOKEN": "2a9613644272b0bf2b6eacaeed68e838"
            },
            "form": {},
            "form_file": "image",
            "response": ["data", "url"],
            "message": ["msg"]
        },
        {
            "name": "moebox",
            "url": "https://catbox.moe/user/api.php",
            "headers": {
                "Referer": "https://catbox.moe/"
            },
            "form": { "reqtype": "fileupload" },
            "form_file": "fileToUpload",
            "response": [],
            "message": []
        },
        {
            "name": "vgy",
            "url": "https://vgy.me/upload",
            "headers": {},
            "form": { "userkey": "a5dT4k2QVgWxaT7UCM1TgI16OpBv37mk" },
            "form_file": "file",
            "response": ["image"],
            "message": []
        },
        {
            "name": "yujian",
            "url": "https://www.hualigs.cn/api/upload?apiType=bilibili&token=562cbd50f59dcc06751de6863ce7adc5",
            "headers": {},
            "form": {},
            "form_file": "image",
            "response": ["data", "url", "bilibili"],
            "message": ["msg"]
        },
        {
            "name": "uploadcc",
            "url": "https://upload.cc/image_upload",
            "headers": {
                "Referer": "https://catbox.moe/"
            },
            "form": {},
            "form_file": "uploaded_file[]",
            "response": ["success_image", 0, "url"],
            "message": ["code"]
        }
    ];

    parse_config(arr)
}
