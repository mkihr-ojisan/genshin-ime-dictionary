use std::path::Path;

use anyhow::Context;
use quick_xml::events::Event;
use romaji::romaji_to_hiragana;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    ext::NodeExt,
    mediawiki::{parse_mediawiki, Node, TemplateArgument},
};

mod ext;
mod mediawiki;
mod romaji;

const DATABASE_DUMP_URL: &str =
    "https://s3.amazonaws.com/wikia_xml_dumps/g/ge/gensinimpact_pages_current.xml.7z";
const DATABASE_DUMP_FILENAME: &str = "gensinimpact_pages_current.xml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_dump = if let Some(filename) = std::env::args().nth(1) {
        std::fs::read(filename).context("Failed to read file")?
    } else {
        let temp_dir = tempfile::tempdir().context("Failed to create tempdir")?;

        eprintln!("Downloading database dump...");

        let compressed_database_dump_filename =
            temp_dir.path().join("gensinimpact_pages_current.xml.7z");
        download_compressed_database_dump(&compressed_database_dump_filename).await?;

        eprintln!("Decompressing database dump...");

        decompress_file(&compressed_database_dump_filename, DATABASE_DUMP_FILENAME)
            .await
            .context("Failed to decompress file")?
    };

    eprintln!("Generating dictionary...");

    let pages_iter = parse_database_dump(&database_dump);

    for page in pages_iter {
        let page = page.context("Failed to parse database dump")?;
        let entries = to_ime_dictionary_entry(&page);
        for entry in entries {
            println!("{}\t{}\t固有名詞", entry.yomi, entry.word);
        }
    }

    Ok(())
}

/// Genshin Impact Wikiからデータベースをダウンロードし指定したファイルに保存する。
async fn download_compressed_database_dump(filename: &Path) -> anyhow::Result<()> {
    let mut response = reqwest::get(DATABASE_DUMP_URL).await.unwrap();
    let mut file = File::create(filename)
        .await
        .context("Failed to create temp file")?;
    while let Some(chunk) = response.chunk().await.unwrap() {
        file.write_all(&chunk)
            .await
            .context("Failed to write chunk to tempfile")?;
    }
    Ok(())
}

/// 指定したアーカイブファイルから指定したファイルを取り出す。
/// 返り値はファイルのバイト列
async fn decompress_file(archive_file: &Path, filename: &str) -> anyhow::Result<Vec<u8>> {
    let command = tokio::process::Command::new("7z")
        .arg("x")
        .arg("-so")
        .arg(archive_file)
        .arg(filename)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .context("Failed to spawn 7z")?;

    let output = command
        .wait_with_output()
        .await
        .context("Failed to wait for 7z")?;

    if !output.status.success() {
        anyhow::bail!("7z exited with non-zero status code");
    }

    Ok(output.stdout)
}

#[derive(Debug, Default)]
struct Page {
    revision: Revision,
}

#[derive(Debug, Default)]
struct Revision {
    text: PageText,
}

#[derive(Debug, Default)]
struct PageText {
    other_languages: Vec<OtherLanguages>,
}

#[derive(Debug, Default)]
struct OtherLanguages {
    ja: Option<Vec<Node<'static>>>,
    ja_rm: Option<Vec<Node<'static>>>,
}

fn parse_database_dump(input: &[u8]) -> impl Iterator<Item = anyhow::Result<Page>> + '_ {
    let reader = quick_xml::Reader::from_reader(input);

    struct PagesIter<'a> {
        reader: quick_xml::Reader<&'a [u8]>,
    }

    impl<'a> Iterator for PagesIter<'a> {
        type Item = anyhow::Result<Page>;
        fn next(&mut self) -> Option<Self::Item> {
            let mut in_page_tag = false;
            let mut in_revision_tag = false;
            let mut in_text_tag = false;

            let mut page = Page::default();

            loop {
                match self.reader.read_event() {
                    Ok(Event::Start(ref e)) if e.name().0 == b"page" => {
                        in_page_tag = true;
                    }
                    Ok(Event::Start(ref e)) if in_page_tag && e.name().0 == b"revision" => {
                        in_revision_tag = true;
                    }
                    Ok(Event::Start(ref e))
                        if in_page_tag && in_revision_tag && e.name().0 == b"text" =>
                    {
                        in_text_tag = true;
                    }
                    Ok(Event::Text(ref e)) if in_text_tag => {
                        let text = match e.unescape() {
                            Ok(text) => text,
                            Err(e) => return Some(Err(e.into())),
                        };

                        if !text.contains("Other Languages") {
                            continue;
                        }

                        let document = parse_mediawiki(&text);

                        document.traverse(&mut |elem| {
                            if let Node::Template { name, arguments } = elem {
                                if *name != "Other Languages" {
                                    return;
                                }

                                let mut other_languages = OtherLanguages::default();

                                if let Some(arg) =
                                    arguments.iter().find(|arg| arg.name == Some("ja".into()))
                                {
                                    other_languages.ja = Some(arg.value.to_static());
                                }

                                if let Some(arg) = arguments
                                    .iter()
                                    .find(|arg| arg.name == Some("ja_rm".into()))
                                {
                                    other_languages.ja_rm = Some(arg.value.to_static());
                                }

                                page.revision.text.other_languages.push(other_languages);
                            }
                        });
                    }
                    Ok(Event::End(ref e))
                        if in_page_tag
                            && in_revision_tag
                            && in_text_tag
                            && e.name().0 == b"text" =>
                    {
                        in_text_tag = false;
                    }
                    Ok(Event::End(ref e))
                        if in_page_tag && in_revision_tag && e.name().0 == b"revision" =>
                    {
                        in_text_tag = false;
                    }
                    Ok(Event::End(ref e)) if in_page_tag && e.name().0 == b"page" => {
                        return Some(Ok(page));
                    }
                    Ok(Event::Eof) => return None,
                    Ok(_) => {}
                    Err(e) => return Some(Err(e.into())),
                }
            }
        }
    }

    PagesIter { reader }
}

#[derive(Debug)]
struct IMEDictionaryEntry {
    word: String,
    yomi: String,
}

fn to_ime_dictionary_entry(page: &Page) -> impl Iterator<Item = IMEDictionaryEntry> + '_ {
    page.revision.text.other_languages.iter().flat_map(
        |other_languages| -> Option<IMEDictionaryEntry> {
            fn template(name: &str, arguments: &[TemplateArgument], output: &mut String) {
                if name == "Rubi" {
                    for i in (0..arguments.len()).step_by(2) {
                        output.push_str(&arguments[i].value.to_string(template));
                    }
                }
            }

            let ja = other_languages.ja.as_ref()?.to_string(template);
            let ja_rm = other_languages.ja_rm.as_ref()?.to_string(template);

            let word = remove_tags(&ja).trim().to_string();
            let romaji = remove_tags(&ja_rm).trim();

            if word.is_empty() || romaji.is_empty() {
                return None;
            }

            let yomi =
                romaji_to_hiragana(romaji, word.contains('ー')).replace("りいうぇ", "りーゆえ");

            Some(IMEDictionaryEntry {
                word: word.to_string(),
                yomi,
            })
        },
    )
}

fn remove_tags(s: &str) -> &str {
    let mut s = s;
    if let Some(i) = s.find("<ref") {
        s = &s[..i];
    }
    if let Some(i) = s.find("<!--") {
        s = &s[..i];
    }
    s
}
