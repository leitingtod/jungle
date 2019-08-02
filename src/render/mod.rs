use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{File, read_to_string};
use std::path::{Path, PathBuf};

use handlebars::{
    Context, Handlebars, Helper, JsonRender, Output, RenderError, to_json,
};
use pulldown_cmark::{CowStr, Event, html, Options, Parser, Tag};
use serde_json::to_string;
use serde_json::value::{Map, Value as Json};

use crate::book::*;
use crate::errors::*;
use crate::utils::{load_file_contents, write_file};
use crate::theme::{INDEX, BOOK};

pub fn render_summary(data: &str, dest: &PathBuf) -> Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("index",
                                        String::from_utf8(INDEX.to_owned())?)?;

    let data = make_summary_data(data);
    info!("json to render: {:#?}", data);

    let rendered = handlebars.render("index", &data)?;
    info!("rendered: {}", rendered);

    info!("write {:?}\n", dest.join("index.html"));
    write_file(dest.as_path(), "index.html", rendered.as_bytes())?;

    Ok(())
}

pub fn make_summary_data(md: &str) -> Map<String, Json> {
    let mut data = Map::new();

    data.insert("summary".to_owned(), json!(render_markdown(md)));

    data
}

pub fn render_book(ctx: &RenderContext) -> Result<()> {
    let mut handlebars = Handlebars::new();
    info!("{:#?}", ctx);

    handlebars.register_template_string("book", String::from_utf8(BOOK.to_owned())?)?;

    let data = make_book_data(&ctx.book);
    debug!("json to render: {:#?}", data);

    let rendered = handlebars.render("book", &data)?;
    info!("rendered: {}", rendered);

    let stripped = ctx.book.root.strip_prefix(get_books_dir(&ctx.root))
        .expect("Chapters are always inside a book");

    info!("write {:?}\n", ctx.destination.join(stripped.join("README.html")));
    write_file(ctx.destination.as_path(),
               stripped.join("README.html"), rendered.as_bytes())?;

    for ch in ctx.book.iter() {
        let md = read_to_string(ch.path.as_path())?;
        let data = render_markdown(md.as_str());

        let stripped = ch.path.parent().unwrap().strip_prefix(get_books_dir(&ctx.root))
            .expect("Chapters are always inside a book");

        let filename = ch.path.file_stem().unwrap();
        let stripped = stripped.join(
            PathBuf::from(filename).with_extension("html"));

        info!("write: {:?}\n", ctx.destination.join(&stripped));
        write_file(ctx.destination.as_path(), stripped.as_path(), data.as_bytes())?;
    }

    Ok(())
}

pub fn make_book_data(book: &Book) -> Map<String, Json> {
    let mut data = Map::new();
    let readme = book.root.join("README.md");
    if readme.exists() {
        let md = read_to_string(readme.as_path())
            .unwrap_or(String::new());
        let content = render_markdown(md.as_str());
        data.insert("content".to_owned(), json!(content));
    } else {
        data.insert("name".to_owned(), json!(book.name.clone()));
    }

    let mut chapters = vec![];

    for item in book.iter() {
        let mut chapter = BTreeMap::new();
        chapter.insert("name".to_owned(), json!(item.name));

        let href = item.path.to_str().unwrap()
            .replace(".md", ".html")
            .replace("src", "build");
        chapter.insert("path".to_owned(), json!(href));

        chapters.push(chapter);
    }

    data.insert("chapters".to_owned(), json!(chapters));

    data
}

pub fn render_markdown(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(text, options);

    // 将md文件中的文件链接地址后缀转换为html
    let parser = parser.map(|event| match event {
        Event::End(Tag::Link(a, b, c)) => {
            let html_link = b.to_owned().to_string().replace(".md", ".html");
            Event::End(Tag::Link(a, CowStr::from(html_link), c))
        }
        Event::Start(Tag::Link(a, b, c)) => {
            let html_link = b.to_owned().to_string().replace(".md", ".html");
            Event::Start(Tag::Link(a, CowStr::from(html_link), c))
        }
        _ => event,
    });

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(text.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    html_output
}

/// The context provided to all renderers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderContext {
    /// Which version of `jungle` did this come from (as written in `jungle`'s
    /// `Cargo.toml`). Useful if you know the renderer is only compatible with
    /// certain versions of `jungle`.
    pub version: String,
    /// The book's root directory.
    pub root: PathBuf,
    /// A loaded representation of the book itself.
    pub book: Book,
    /// Where the renderer *must* put any build artefacts generated. To allow
    /// renderers to cache intermediate results, this directory is not
    /// guaranteed to be empty or even exist.
    pub destination: PathBuf,
    #[serde(skip)]
    __non_exhaustive: (),
}

impl RenderContext {
    /// Create a new `RenderContext`.
    pub fn new<P, Q>(root: P, book: Book, destination: Q) -> RenderContext
        where
            P: Into<PathBuf>,
            Q: Into<PathBuf>,
    {
        RenderContext {
            book,
            version: crate::VERSION.to_string(),
            root: root.into(),
            destination: destination.into(),
            __non_exhaustive: (),
        }
    }

    /// Get the source directory's (absolute) path on disk.
    pub fn source_dir(&self) -> PathBuf {
        self.root.join("")
    }
}


