use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let base_url = "https://www.chenwanglaw.com";
    let paths = vec!["/", "services", "articles", "members", "contact_us"];

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    for path in paths {
        let full_url = format!("{}/{}", base_url, path);
        xml.push_str(&format!(
            "<url><loc>{}</loc><changefreq>weekly</changefreq></url>",
            full_url.trim_end_matches('/')
        ));
    }

    xml.push_str("</urlset>");

    let mut file = File::create("sitemap.xml")?;
    file.write_all(xml.as_bytes())?;

    println!("sitemap.xml generated ✅");
    Ok(())
}
