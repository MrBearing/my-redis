use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // バイト文字列の先頭からいくつかを書き込む。
    // 必ずしもすべてを書き込むわけではないことに注意。
    let n = file.write(b"some bytes").await?;
    
    println!("Wrote the first {} bytes of 'some bytes'.", n);
    // let mut buffer = File::create("foo.txt").await?;
    // buffer.write_all(b"some bytes with all").await?;
    
    Ok(())
}