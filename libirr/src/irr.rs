use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct IRRClient {
    stream: TcpStream,
}

impl IRRClient {
    pub async fn connect(server: &str) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(server).await?;
        Ok(Self { stream })
    }

    async fn query(&mut self, query: &str) -> anyhow::Result<String> {
        self.stream.write_all(query.as_bytes()).await?;

        // get header
        let mut line = String::new();
        let mut reader = tokio::io::BufReader::new(&mut self.stream);
        reader.read_line(&mut line).await?;

        let len = line[1..].trim().parse::<usize>()?;

        let mut payload = vec![0; len];
        reader.read_exact(&mut payload).await?;
        let payload_str = String::from_utf8(payload)?;

        Ok(payload_str)
    }

    pub async fn route(&mut self, asn: i32) -> anyhow::Result<Vec<String>> {
        let query = format!("!gAS{}\n", asn);

        let response = self.query(&query).await?;
        Ok(response.split_whitespace().map(|s| s.to_string()).collect())
    }
}
