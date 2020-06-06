# future_rss

Use Rust to Serialize the Rss structure.

### Usage

```toml
tokio = { version ="*", features = [ "macros","rt-core" ] }
future_rss = "*"
```

### Examples

##### Parse Xml

```rust
use future_rss::*;


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::from_url(address,"utf8").await?;
    parser.author_tag = String::from("dc:creator");
    
    let rss = parser.parse_vec().await?;
    println!("{:?}",rss);    
    Ok(())
}
```
#### Parse Web XMl

```rust
use future_rss::RssParser;

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::from_url(address,"utf8").await?;
    parser.author_tag = String::from("dc:creator");
    assert!(parser.parse_json().await.is_ok());
    Ok(())
}
```

#### RSS To Json

```rust
use future_rss::RssParser;

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::from_url(address,"utf8").await?;
    parser.author_tag = String::from("dc:creator");
    assert!(parser.parse_json().await.is_ok());
    Ok(())
}
```

### Rss Request Buidler

```rust
use future_rss::RssParser;

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let address = "https://www.zhihu.com/rss";
    let mut parser = RssParser::new();
    parser.author_tag = "dc:creator".into();
    parser.publish_tag = "pubDate".into();
    let xml = parser.request_xml(address.as_str(),charset.as_str()).await?;
    parser.set_xml(xml);
    assert!(parser.parse_vec().await.is_ok());
    Ok(())
}
```

### Advanced

[Examples](https://github.com/MeteorGX/future_rss_examples)
