use crate::error::Error;
use crate::info::Information;

use regex::Regex;
use url::Url;

use minicalc::compute;
use percent_encoding::percent_decode_str;
use scraper::{Html, Selector};

pub struct File {
    origin: String,
}

impl File {
    pub fn new(origin: &str) -> File {
        File {
            origin: origin.into(),
        }
    }

    pub fn get_information(&self) -> Result<Information, Box<dyn std::error::Error>> {
        let script_content = {
            let page_content = fetch_content(&self.origin)?;
            let document = Html::parse_document(&page_content);
            let selector =
                Selector::parse("#lrbox .right script").map_err(|_| Error::InvalidSelector)?;

            Ok::<String, Error>(
                document
                    .select(&selector)
                    .nth(0)
                    .ok_or(Error::CannotFindScriptTag)?
                    .inner_html(),
            )
        }?;

        let re1 = Regex::new(
            r#"document.getElementById\('dlbutton'\)\.href = "/d/(\w+)/"\s?\+\s?\(a \* b \+ c \+ d\)\s?\+\s?"/([/\w%.-]+)";"#,
        )?;
        let re2 = Regex::new(
            r#"document.getElementById\('dlbutton'\)\.href = "/d/(\w+)/"\s?\+\s?\(([\d+% ]+)\)\s?\+\s?"/([/\w%.-]+)";"#,
        )?;

        if re1.is_match(&script_content) {
            let re_var = Regex::new(r#"var\s?a\s?=\s?(\d+)\s?%\s?900\s?;"#)?;
            let var = {
                let groups = re_var
                    .captures(&script_content)
                    .ok_or(Error::InvalidScriptContent)?;
                groups[1].parse::<i64>()
            }?;

            let groups = re1
                .captures(&script_content)
                .ok_or(Error::InvalidScriptContent)?;
            Ok(Information {
                domain: String::from(
                    Url::parse(&self.origin)?
                        .host_str()
                        .ok_or(Error::InvalidDomain)?,
                ),
                id: String::from(&groups[1]),
                key: (var % 900) * (var % 53) + 8 + (var % 13),
                name: String::from(percent_decode_str(&groups[2]).decode_utf8()?),
                encoded_name: String::from(&groups[2]),
            })
        } else if re2.is_match(&script_content) {
            let groups = re2
                .captures(&script_content)
                .ok_or(Error::InvalidScriptContent)?;
            Ok(Information {
                domain: String::from(
                    Url::parse(&self.origin)?
                        .host_str()
                        .ok_or(Error::InvalidDomain)?,
                ),
                id: String::from(&groups[1]),
                key: compute(&groups[2])?,
                name: String::from(percent_decode_str(&groups[3]).decode_utf8()?),
                encoded_name: String::from(&groups[3]),
            })
        } else {
            Err(Error::ScriptContentNotMatching.into())
        }
    }
}

fn fetch_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = ureq::get(url).call();
    if resp.ok() {
        Ok(resp.into_string()?)
    } else {
        Err("invalid HTTP response".into())
    }
}
