#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Starting server...");
    let _rocket = veryrezsi::rocket().launch().await?;
    Ok(())
}
