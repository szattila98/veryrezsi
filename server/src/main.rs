use rocket;
use veryrezsi;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = veryrezsi::rocket().launch().await?;
    Ok(())
}
