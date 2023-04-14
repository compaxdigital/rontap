# Library for Netapp API access

This is a rust library that can be used to access the Netapp ONTAP REST API.

It is handwritten and not necessarily including all the functionalities you can find in the swagger docs.

## Example

```rust
use reqwest::Url;
use rontap::OntapConnectionParams;

#[tokio::main]
async fn main() -> Result<()> {
    let params = OntapConnectionParams {
        url: Url::from_str("https://mynetapp.example.com/api")?,
        username: "foobar".into(),
        password: "secret".into(),
    };
    let c = params.connect().await?;

    dbg!(c.get_volumes().await?);
}
```
