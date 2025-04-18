use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = run()?;
    server.await
}
