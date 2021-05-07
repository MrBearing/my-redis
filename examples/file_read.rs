use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // 最大10バイト読み込む
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    
    let mut buffer = Vec::new();

    // ファイルをすべて読み込む
    f.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", buffer);

    let mut file = File::create("foo.txt").await?;
    let n = file.write(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);


    Ok(())
}