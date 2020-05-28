use std::io::prelude::*;
use std::fs::File;
use quick_xml::Reader;
use quick_xml::events::Event;
use json::{object,array};

pub static RSS_DEFAULT_NODE_TAG:&'static str = "item";
pub static RSS_DEFAULT_TITLE_TAG:&'static str = "title";
pub static RSS_DEFAULT_LINK_TAG:&'static str = "link";
pub static RSS_DEFAULT_AUTHOR_TAG:&'static str = "author";
pub static RSS_DEFAULT_DESC_TAG:&'static str = "description";
pub static RSS_DEFAULT_PUBLISH_TAG:&'static str = "pubDate";


#[derive(Debug)]
#[allow(dead_code)]
pub struct RssItem{
    pub title: String,
    pub link: String,
    pub author: String,
    pub description: String,
    pub publish: String,
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct RssParser{
    xml:String,
    pub node_tag:String,
    pub title_tag:String,
    pub link_tag:String,
    pub author_tag:String,
    pub description_tag:String,
    pub publish_tag:String,
}


impl Default for RssItem{
    fn default() -> Self {
        Self{
            title:String::new(),
            link:String::new(),
            author:String::new(),
            description:String::new(),
            publish:String::new()
        }
    }
}


impl RssParser{
    pub async fn from_url(url:&str,charset:&str)->Result<Self,Box<dyn std::error::Error>>{
        let body = reqwest::get(url)
            .await?
            .text_with_charset(charset)
            .await?;

        Ok(Self{
            xml:body,
            node_tag:String::from(RSS_DEFAULT_NODE_TAG),
            title_tag:String::from(RSS_DEFAULT_TITLE_TAG),
            link_tag:String::from(RSS_DEFAULT_LINK_TAG),
            author_tag:String::from(RSS_DEFAULT_AUTHOR_TAG),
            description_tag:String::from(RSS_DEFAULT_DESC_TAG),
            publish_tag:String::from(RSS_DEFAULT_PUBLISH_TAG)
        })
    }

    pub async fn from_file(filename:&str)->Result<Self,Box<dyn std::error::Error>>{
        let mut f = File::open(filename)?;
        let mut body = String::new();
        f.read_to_string(&mut body)?;

        Ok(Self{
            xml:body,
            node_tag:String::from(RSS_DEFAULT_NODE_TAG),
            title_tag:String::from(RSS_DEFAULT_TITLE_TAG),
            link_tag:String::from(RSS_DEFAULT_LINK_TAG),
            author_tag:String::from(RSS_DEFAULT_AUTHOR_TAG),
            description_tag:String::from(RSS_DEFAULT_DESC_TAG),
            publish_tag:String::from(RSS_DEFAULT_PUBLISH_TAG)
        })
    }

    pub async fn parse_vec(&mut self)->Result<Vec<RssItem>,Box<dyn std::error::Error>>{
        let mut reader = Reader::from_str(self.xml.as_str());

        reader.trim_text(true);
        reader.check_end_names(true);
        reader.check_comments(false);
        reader.expand_empty_elements(true);


        let mut buff = Vec::new();
        let mut nodes = Vec::new();
        let mut active = String::new();

        loop{
            match reader.read_event(&mut buff) {
                // Fetch = <Item></Item>
                Ok(Event::Start(ref e)) => {
                    active = std::str::from_utf8(e.name())?.to_string();
                    if self.node_tag.eq_ignore_ascii_case(&active) {
                        nodes.push(RssItem::default());
                    }
                }

                // Fetch = <Item><Node><CDATA></Node><Item>
                Ok(Event::CData(ref e)) => {
                    let node_text = std::str::from_utf8(e.escaped())?;
                    if let Some(last) = nodes.last_mut() {
                        match active {
                            _ if self.title_tag.eq_ignore_ascii_case(&active) => { last.title = node_text.to_string() },
                            _ if self.link_tag.eq_ignore_ascii_case(&active) => { last.link = node_text.to_string() },
                            _ if self.author_tag.eq_ignore_ascii_case(&active) => { last.author = node_text.to_string() },
                            _ if self.description_tag.eq_ignore_ascii_case(&active) => { last.description = node_text.to_string() },
                            _ if self.publish_tag.eq_ignore_ascii_case(&active) => { last.publish = node_text.to_string() },
                            _ => (),
                        }
                    }
                }

                // Fetch = <Item><Node></Node><Item>
                Ok(Event::Text(ref e)) => {
                    let node_text = e.unescape_and_decode(&reader)?;
                    if let Some(last) = nodes.last_mut() {
                        match active {
                            _ if self.title_tag.eq_ignore_ascii_case(&active) => { last.title = node_text.to_string() },
                            _ if self.link_tag.eq_ignore_ascii_case(&active) => { last.link = node_text.to_string() },
                            _ if self.author_tag.eq_ignore_ascii_case(&active) => { last.author = node_text.to_string() },
                            _ if self.description_tag.eq_ignore_ascii_case(&active) => { last.description = node_text.to_string() },
                            _ if self.publish_tag.eq_ignore_ascii_case(&active) => { last.publish = node_text.to_string() },
                            _ => (),
                        }
                    }
                }

                Ok(Event::Eof) => break,
                _ =>(),
                Err(e) => return Err(Box::new(e)),
            }
            buff.clear();
        }

        Ok(nodes)
    }

    pub async fn parse_json(&mut self)->Result<String,Box<dyn std::error::Error>>{
        let item = self.parse_vec().await?;

        Ok(String::new())
    }
}




#[cfg(test)]
mod tests {
    use crate::RssParser;

    #[tokio::test]
    async fn future_rss_works()->Result<(),Box<dyn std::error::Error>> {
        let address = "https://www.zhihu.com/rss";
        let mut parser = RssParser::from_url(address,"utf8").await?;
        let rss = parser.parse_vec().await?;
        assert!(rss.len()>0);

        Ok(())
    }
}
