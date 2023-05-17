/*
sudo docker start elasticsearc
sudo docker start confident_cori

sudo docker run -d -p 3000:3000 grafana/grafana
docker run -d --name elasticsearch --net somenetwork -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" elasticsearch:7.17.10

curl -XPUT "localhost:9200/index5"
curl "http://localhost:9200/index5/_search?pretty"
curl -H "Content-Type: application/json" -XPOST "http://localhost:9200/index5/_doc" -d '{"qwe":2, "name":"name_qwe", "@timestamp" : "'$(date -Iseconds)'"}'
*/

use elasticsearch::{
    Elasticsearch,
    http::Method,
    http::headers::HeaderMap,
    http::transport::Transport,
};

use serde_json::{json, Value};

use chrono::{Utc, Duration};

use std::{thread, time};

use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let transport = Transport::single_node("http://localhost:9200")?;
    let client = Elasticsearch::new(transport);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for _ in 0..100000 {

        let body = json!({
            "qwe": rng.gen_range(0..1000),
            "name": "TestData_2",
            "@timestamp" : Utc::now().to_rfc3339()
        });
    
        let response = client
            .send(Method::Post,
                "index5/_doc",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(body.to_string().as_bytes()),
                None
            )
            .await?;

        std::thread::sleep(time::Duration::from_millis(500));

        /*let status_code = response.status_code();
        let response_body = response.json::<Value>().await?;

        println!("status_code: {status_code}");
        println!("response_body: {response_body}");*/

    }

    Ok(())

}


