Here is an example of a working preprocessor for mdbook v0.5.1 that I wrote that
mdbook-content-loader made for, for reference.

1. Cargo.toml

```toml
[package]
name = "mdbook-content-collections"
version = "0.1.3"
edition = "2024"
authors = ["T Sawyer saylesss88@github.com"]
license = "Apache-2.0"
description = "An mdBook preprocessor that provides Astro-like content collections with typed frontmatter, validation, and structured content indexing"
repository = "https://github.com/saylesss88/mdbook-content-collections"
readme = "README.md"
keywords = [
	"mdbook",
	"content-collections",
	"frontmatter",
	"preprocessor",
	"schema",
]
categories = ["command-line-utilities", "web-programming", "development-tools"]

[lib]
name = "mdbook_content_collections"
path = "src/lib.rs"

[[bin]]
name = "mdbook-content-collections"
path = "src/bin/mdbook-content-collections.rs"

[dependencies]
clap = {version = "4", features = ["derive"]}
anyhow = "1"
walkdir = "2"
serde = {version = "1", features = ["derive"]}
serde_yaml = "0.9"
chrono = {version = "0.4", features = ["serde"]}
pulldown-cmark = "0.13"
serde_json = "1"
globset = "0.4"
toml = "0.9.8"

# Optional: keep for future RSS generation from the content index
# rss = {version = "2", features = ["builders"], optional = true}

# [features]
# rss = ["dep:rss"]
```

2. lib.rs

```rs
use anyhow::Result;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::{fs, path::Path, time::SystemTime};
use walkdir::WalkDir;

// Minimum body length (in chars) before we prefer it over description
const MIN_BODY_PREVIEW_CHARS: usize = 80;

// Convert file modification time → UTC
fn systemtime_to_utc(st: SystemTime) -> DateTime<Utc> {
    DateTime::<Utc>::from(st)
}

// Parse front-matter date formats
fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(date_str) = s {
        if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
            return Ok(Some(dt.with_timezone(&Utc)));
        }

        if let Ok(nd) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            return Ok(Some(
                Utc.from_utc_datetime(&nd.and_hms_opt(0, 0, 0).unwrap()),
            ));
        }
    }
    Ok(None)
}

#[derive(Debug, Deserialize, Clone)]
pub struct FrontMatter {
    pub title: String,

    #[serde(deserialize_with = "deserialize_date")]
    pub date: Option<DateTime<Utc>>,

    pub author: Option<String>,
    pub description: Option<String>, // User-supplied summary (optional)

    // New: optional collection name
    pub collection: Option<String>,

    // New: simple tags array for indexing
    pub tags: Option<Vec<String>>,

    // New: draft flag
    pub draft: Option<bool>,
}

#[derive(Debug)]
pub struct Article {
    pub fm: FrontMatter,
    pub content: String,
    pub path: String,
}

pub fn parse_markdown_file(root: &Path, path: &Path) -> Result<Article> {
    let text = fs::read_to_string(path)?;

    let mut lines = text.lines();
    let mut yaml = String::new();
    let mut in_yaml = false;

    // Extract YAML front matter
    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if !in_yaml {
                in_yaml = true;
                continue;
            } else {
                break;
            }
        }
        if in_yaml {
            yaml.push_str(line);
            yaml.push('\n');
        }
    }

    // Markdown content after front matter
    let content = lines.collect::<Vec<_>>().join("\n") + "\n";

    let fallback_date = path
        .metadata()
        .ok()
        .and_then(|m| m.modified().ok())
        .map(systemtime_to_utc);

    // Parse front matter
    let fm = if !yaml.trim().is_empty() {
        serde_yaml::from_str(&yaml).unwrap_or_else(|_| FrontMatter {
            title: path.file_stem().unwrap().to_string_lossy().into_owned(),
            date: fallback_date,
            author: None,
            description: Some(content.clone()),
            collection: None,
            tags: None,
            draft: None,
        })
    } else {
        FrontMatter {
            title: path.file_stem().unwrap().to_string_lossy().into_owned(),
            date: fallback_date,
            author: None,
            description: Some(content.clone()),
            collection: None,
            tags: None,
            draft: None,
        }
    };

    let rel_path = path.strip_prefix(root).unwrap_or(path);

    Ok(Article {
        fm,
        content,
        path: rel_path.to_string_lossy().into_owned(),
    })
}

pub fn collect_articles(src_dir: &Path) -> Result<Vec<Article>> {
    let mut articles = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase());

        if !matches!(ext.as_deref(), Some("md" | "markdown")) {
            continue;
        }

        if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .eq_ignore_ascii_case("SUMMARY.md")
        {
            continue;
        }

        if let Ok(article) = parse_markdown_file(src_dir, path) {
            articles.push(article);
        }
    }

    // Sort newest → oldest
    articles.sort_by_key(|a| a.fm.date);
    articles.reverse();

    Ok(articles)
}

fn markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let parser = Parser::new_ext(md, Options::all());
    html::push_html(&mut html, parser);
    html
}

/// Strip obvious leading boilerplate (TOCs, details, long definition blocks)
/// so previews tend to start at the main intro text instead of metadata.
fn strip_leading_boilerplate(md: &str) -> &str {
    let mut seen_heading = false;
    let mut byte_idx = 0;
    let mut acc_bytes = 0;

    for (i, line) in md.lines().enumerate() {
        let line_len_with_nl = line.len() + 1; // assume '\n' separated

        // Skip initial blank lines entirely
        if i == 0 && line.trim().is_empty() {
            acc_bytes += line_len_with_nl;
            continue;
        }

        if line.trim_start().starts_with('#') {
            seen_heading = true;
        }

        if seen_heading && line.trim().is_empty() {
            // First blank line after heading: start preview after this
            acc_bytes += line_len_with_nl;
            byte_idx = acc_bytes;
            break;
        }

        acc_bytes += line_len_with_nl;
    }

    if byte_idx == 0 {
        md
    } else {
        &md[byte_idx.min(md.len())..]
    }
}

/// Take at most `max_chars` worth of UTF‑8 text from `s`.
fn utf8_prefix(s: &str, max_chars: usize) -> &str {
    if max_chars == 0 {
        return "";
    }

    let mut last_byte = 0;

    for (ch_idx, (byte_idx, _)) in s.char_indices().enumerate() {
        if ch_idx == max_chars {
            last_byte = byte_idx;
            break;
        }
        last_byte = byte_idx + 1;
    }

    if last_byte == 0 || last_byte >= s.len() {
        s
    } else {
        &s[..last_byte]
    }
}

/// Take up to `max_paragraphs` <p> blocks from HTML, and cap at `max_chars` (UTF-8 safe).
fn html_first_paragraphs(html: &str, max_paragraphs: usize, max_chars: usize) -> String {
    let mut out = String::new();
    let mut start = 0;
    let mut count = 0;

    while count < max_paragraphs {
        // Find next <p ...>
        let rel = match html[start..].find("<p") {
            Some(i) => i,
            None => break,
        };
        let p_start = start + rel;

        // Find the end of this paragraph
        let rel_close = match html[p_start..].find("</p>") {
            Some(i) => i,
            None => break,
        };
        let close = p_start + rel_close + "</p>".len();

        let para = &html[p_start..close];
        out.push_str(para);
        count += 1;
        start = close;
    }

    // If no <p> found, fall back to original HTML
    if out.is_empty() {
        out = html.to_string();
    }

    // UTF‑8 safe trim by character count
    if out.chars().count() > max_chars {
        out.chars().take(max_chars).collect()
    } else {
        out
    }
}

#[derive(Debug, Serialize)]
pub struct ContentEntry {
    pub path: String, // relative path in src
    pub title: String,
    pub date: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub preview_html: String,
}

/// Build a simple content index from src_dir and write it to `output_path`.
pub fn build_content_index(src_dir: &Path, output_path: &Path) -> Result<()> {
    let articles = collect_articles(src_dir)?;

    let entries: Vec<ContentEntry> = articles
        .into_iter()
        .map(|article| {
            let content_trimmed = article.content.trim();
            let body_len = content_trimmed.chars().count();

            let mut source_md =
                if body_len >= MIN_BODY_PREVIEW_CHARS || article.fm.description.is_none() {
                    content_trimmed
                } else {
                    article.fm.description.as_deref().unwrap_or(content_trimmed)
                };

            source_md = strip_leading_boilerplate(source_md);

            const PREVIEW_MD_SLICE_CHARS: usize = 4000;
            let source_md_slice = utf8_prefix(source_md, PREVIEW_MD_SLICE_CHARS);

            let raw_html = markdown_to_html(source_md_slice);
            let preview_html = html_first_paragraphs(&raw_html, 3, 800);

            ContentEntry {
                path: article.path,
                title: article.fm.title,
                date: article.fm.date.map(|d| d.to_rfc3339()),
                author: article.fm.author,
                description: article.fm.description,
                collection: article.fm.collection,
                tags: article.fm.tags.unwrap_or_default(),
                draft: article.fm.draft.unwrap_or(false),
                preview_html,
            }
        })
        .collect();

    // Very simple index shape for now
    let index = json!({
        "entries": entries,
    });

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(output_path, serde_json::to_vec_pretty(&index)?)?;

    Ok(())
}
```

3. main.rs

```rs
use mdbook_content_collections::build_content_index;
use serde_json::Value;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("--version")
        || args.get(1).map(|s| s.as_str()) == Some("-V")
    {
        println!("mdbook-content-collections {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        println!("true");
        return;
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let input_array: Vec<Value> = serde_json::from_str(&input).expect("Invalid JSON from mdBook");
    let context = &input_array[0];
    let book = &input_array[1];

    let root = context
        .pointer("/root")
        .and_then(|v| v.as_str())
        .unwrap_or(".");

    let src_dir = PathBuf::from(root).join("src");

    // Write directly into src/ → gets copied to book/ automatically
    let index_path = src_dir.join("content-collections.json");

    eprintln!("Writing content index to: {}", index_path.display());

    if let Err(e) = build_content_index(&src_dir, &index_path) {
        eprintln!("ERROR: Failed to build content index: {e}");
        std::process::exit(1);
    }

    // Success confirmation (optional, nice for debugging)
    if index_path.exists() {
        let size = index_path.metadata().map(|m| m.len()).unwrap_or(0);
        eprintln!("Content index written successfully ({} bytes)", size);
    }

    // Echo back the unchanged book
    println!("{}", serde_json::to_string(book).unwrap());
}
```

Based off of that, help me fix mdbook-content-loader:

1. Cargo.toml

```toml
[package]
name = "mdbook-content-loader"
version = "0.1.0"
edition = "2021"
description = "Injects content-collections.json into mdBook pages as a global variable"
license = "Apache-2.0"

[dependencies]
mdbook-preprocessor = "0.5.1"
serde = {version = "1.0", features = ["derive"]}
serde_json = "1.0"
log = "0.4"
env_logger = "0.10"
chrono = {version = "0.4", features = ["clock"]}
anyhow = "1.0"
semver = "1.0"
```

2. lib.rs

```rs
use anyhow::{bail, Context};
use chrono::Utc;
use log;
use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::Error,
    Preprocessor, PreprocessorContext,
};
use serde_json::{json, Map, Value};
use std::cmp::Reverse;
use std::fs;
use std::path::Path;

pub struct ContentLoader;

impl ContentLoader {
    pub fn new() -> ContentLoader {
        ContentLoader
    }
}

impl Preprocessor for ContentLoader {
    fn name(&self) -> &str {
        "content-loader"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // mdBook 0.5.1: Config has a typed book.src (PathBuf)
        let src = ctx.config.book.src.to_str().unwrap_or("src");
        let src_dir = ctx.root.join(src);
        let index_path = src_dir.join("content-collections.json");

        let payload: Value = match load_collections(&index_path) {
            Ok(data) => data,
            Err(e) => {
                log::warn!("content-loader: {}", e);
                return Ok(book);
            }
        };

        let script = format!(
            r#"<script>window.CONTENT_COLLECTIONS = {};</script>"#,
            serde_json::to_string(&payload)?
        );

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = format!("{}\n{}", script, chapter.content);
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}

fn load_collections(path: &Path) -> anyhow::Result<Value> {
    if !path.exists() {
        bail!("content-collections.json not found at {:?}", path);
    }

    let content = fs::read_to_string(path).context("Failed to read content-collections.json")?;
    let json_val: Value = serde_json::from_str(&content).context("Failed to parse JSON")?;

    let entries: Vec<Value> = json_val
        .get("entries")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().cloned().collect())
        .unwrap_or_default();

    let published: Vec<_> = entries
        .into_iter()
        .filter(|e| !e.get("draft").and_then(|v| v.as_bool()).unwrap_or(false))
        .collect();

    let mut collections: Map<String, Value> = Map::new();
    let mut default_collection = vec![];

    for entry in &published {
        let coll = entry
            .get("collection")
            .and_then(|v| v.as_str())
            .unwrap_or("posts")
            .to_string();
        if coll == "posts" {
            default_collection.push(entry.clone());
        } else {
            let entry_arr = collections
                .entry(coll)
                .or_insert_with(|| json!([]))
                .as_array_mut()
                .expect("Failed to convert to array");
            entry_arr.push(entry.clone());
        }
    }

    if !default_collection.is_empty() {
        sort_by_date_desc(&mut default_collection);
        collections.insert("posts".to_string(), json!(default_collection));
    }

    for coll in collections.values_mut() {
        if let Value::Array(arr) = coll {
            sort_by_date_desc(arr);
        }
    }

    Ok(json!({
        "entries": published,
        "collections": collections,
        "generated_at": Utc::now().to_rfc3339(),
    }))
}

fn sort_by_date_desc(arr: &mut Vec<Value>) {
    arr.sort_by_key(|e| {
        let date = e.get("date").and_then(|v| v.as_str()).unwrap_or("");
        Reverse(date.to_string())
    });
}
```

3. main.rs

```rs
use mdbook_content_loader::ContentLoader;
use mdbook_preprocessor::{errors::Error, CmdPreprocessor, Preprocessor, MDBOOK_VERSION};
use semver::{Version, VersionReq};
use std::io;
use std::process;

fn main() {
    env_logger::init();
    let preprocessor = ContentLoader::new();

    if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        log::warn!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
```

4. Compiler error:

```text
Checking mdbook-content-loader v0.1.0 (/var/home/jr/mdbook-content-loader)
error[E0432]: unresolved import `mdbook_preprocessor::CmdPreprocessor`
 --> src/main.rs:2:42
  |
2 | use mdbook_preprocessor::{errors::Error, CmdPreprocessor, Preprocessor, MDBOOK_VERSION};
  |                                          ^^^^^^^^^^^^^^^
  |                                          |
  |                                          no `CmdPreprocessor` in the root
  |                                          help: a similar name exists in the module: `Preprocessor`

For more information about this error, try `rustc --explain E0432`.
error: could not compile `mdbook-content-loader` (bin "mdbook-content-loader") due to 1 previou
```
