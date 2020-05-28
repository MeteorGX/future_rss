# future_rss

Use Rust to Serialize the Rss structure.

### Usage

```toml
tokio = { version ="*", features = [ "macros","rt-core" ] }
future_rss = "*"
```

### Examples

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

