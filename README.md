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

### Advanced

> todo