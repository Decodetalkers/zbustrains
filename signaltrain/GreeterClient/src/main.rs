use futures_util::StreamExt;
use zbus::{dbus_proxy, Connection, Result};
#[dbus_proxy(
    default_service = "org.zbus.MyGreeter",
    interface = "org.zbus.MyGreeter1",
    default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter1 {
    #[dbus_proxy(signal)]
    fn greeted_everyone(&self) -> Result<()>;
    #[dbus_proxy(signal)]
    fn bye_everyone(&self) -> Result<String>;
    // add code here
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::session().await?;
    let greeter = MyGreeter1Proxy::new(&conn).await?;

    let mut greeted_happened = greeter.receive_greeted_everyone().await?;
    let mut bye_happened = greeter.receive_bye_everyone().await?;
    futures_util::try_join!(
        async {
            while let Some(_signal) = greeted_happened.next().await {
                println!("happened");
            }
            Ok::<(), zbus::Error>(())
        },
        async {
            while let Some(signal) = bye_happened.next().await {
                let bytes = signal.body_as_bytes()?;
                let name = String::from_utf8_lossy(bytes);
                println!("name = {name}");
            }
            Ok::<(), zbus::Error>(())
        }
    )?;

    Ok(())
}
