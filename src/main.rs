#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let url = "https://www.youtube.com/watch?v=722c09D-ePs";
    let path_to_video = rustube::download_best_quality(url).await?;


    println!("path_to_video: {}", path_to_video.display());

    return Ok(())
}

