use regex::Regex;

pub fn fetch_title(url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let html = ureq::get(url).call()?.body_mut().read_to_string()?;
    let re = Regex::new(r"(?is)<title[^>]*>(.*?)</title>")?;
    let title = re
        .captures(&html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "(no title)".to_string());

    Ok(title)
}
