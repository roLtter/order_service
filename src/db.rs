use tokio_postgres::{Client, NoTls, Error};

pub async fn connect_to_db() -> Result<Client, Error> {
    let database_url = "host=localhost user=postgres password=1111 dbname=order_service";
    let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}
