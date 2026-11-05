use miette::{
    Result,
    miette,
};
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::{
        TcpListener,
        TcpStream,
    },
};

const CALLBACK_PATH: &str = "/callback";

const MAX_CALLBACK_REQUEST_SIZE: usize = 16 * 1024;

/// A single-use loopback HTTP server that receives an OAuth redirect
pub struct CallbackServer {
    listener: TcpListener,
    redirect_uri: String,
    completion_message: String,
}

impl CallbackServer {
    pub async fn bind(completion_message: &str) -> Result<Self> {
        let listener = TcpListener::bind(("127.0.0.1", 0))
            .await
            .map_err(|error| miette!("failed to start the OAuth callback server: {error}"))?;

        let address = listener
            .local_addr()
            .map_err(|error| miette!("failed to determine the OAuth callback address: {error}"))?;

        let redirect_uri = format!("http://127.0.0.1:{}{CALLBACK_PATH}", address.port());

        Ok(Self { listener, redirect_uri, completion_message: completion_message.to_string() })
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub async fn receive_callback_url(self) -> Result<String> {
        let (mut stream, _) = self
            .listener
            .accept()
            .await
            .map_err(|error| miette!("failed to accept the OAuth callback: {error}"))?;

        let request = read_request(&mut stream).await?;

        let request_target = parse_request_target(&request)?;

        let callback_origin = self
            .redirect_uri
            .strip_suffix(CALLBACK_PATH)
            .ok_or_else(|| miette!("invalid OAuth redirect URI"))?;

        let callback_url = format!("{callback_origin}{request_target}");

        write_response(&mut stream, &self.completion_message).await?;

        Ok(callback_url)
    }
}

async fn read_request(stream: &mut TcpStream) -> Result<String> {
    let mut request = Vec::new();

    let mut buffer = [0_u8; 1024];

    while request.len() < MAX_CALLBACK_REQUEST_SIZE {
        let bytes_read = stream
            .read(&mut buffer)
            .await
            .map_err(|error| miette!("failed to read the OAuth callback: {error}"))?;

        if bytes_read == 0 {
            break;
        }

        let chunk = buffer
            .get(..bytes_read)
            .ok_or_else(|| miette!("OAuth callback read exceeded the read buffer"))?;

        request.extend_from_slice(chunk);

        if request
            .windows(4)
            .any(|window| window == b"\r\n\r\n")
        {
            break;
        }
    }

    String::from_utf8(request)
        .map_err(|error| miette!("OAuth callback was not valid UTF-8: {error}"))
}

fn parse_request_target(request: &str) -> Result<&str> {
    request
        .lines()
        .next()
        .and_then(|request_line| {
            request_line
                .split_whitespace()
                .nth(1)
        })
        .ok_or_else(|| miette!("OAuth callback did not contain a request target"))
}

async fn write_response(stream: &mut TcpStream, completion_message: &str) -> Result<()> {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{completion_message}",
        completion_message.len()
    );

    stream
        .write_all(response.as_bytes())
        .await
        .map_err(|error| miette!("failed to respond to the OAuth callback: {error}"))?;

    Ok(())
}
