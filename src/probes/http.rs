use anyhow::{Result, ensure};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
};

pub async fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let url = format!("http://{}/stream", listener.local_addr()?);
    let (first_seen, wait_for_first) = oneshot::channel();

    let server = async move {
        let (mut socket, _) = listener.accept().await?;
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") {
            ensure!(request.len() < 4096, "oversized fixture request");
            request.push(socket.read_u8().await?);
        }
        ensure!(
            request.starts_with(b"GET /stream "),
            "unexpected fixture request"
        );
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n6\r\nfirst\n\r\n")
            .await?;
        // Withhold the rest of the body until the client has consumed the first part.
        wait_for_first.await?;
        socket.write_all(b"7\r\nsecond\n\r\n0\r\n\r\n").await?;
        Ok(())
    };

    let client = async move {
        let mut response = reqwest::Client::builder()
            .no_proxy()
            .build()?
            .get(url)
            .send()
            .await?
            .error_for_status()?;
        let first = b"first\n";
        let expected = b"first\nsecond\n";
        let mut body = Vec::new();
        while body.len() < first.len() {
            let Some(chunk) = response.chunk().await? else {
                anyhow::bail!("body ended before the first part");
            };
            body.extend_from_slice(&chunk);
            ensure!(first.starts_with(&body), "unexpected first part");
        }
        first_seen
            .send(())
            .map_err(|()| anyhow::anyhow!("fixture disconnected"))?;
        while let Some(chunk) = response.chunk().await? {
            body.extend_from_slice(&chunk);
            ensure!(expected.starts_with(&body), "unexpected response body");
        }
        ensure!(body == expected, "incomplete response body");
        Ok(())
    };

    let ((), ()) = tokio::try_join!(server, client)?;
    println!(
        "PASS: Reqwest received the first HTTP body part before the server released the second."
    );
    Ok(())
}
