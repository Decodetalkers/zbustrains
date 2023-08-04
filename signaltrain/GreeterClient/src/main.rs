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
    #[dbus_proxy(property)]
    fn greeter_name(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn set_greeter_name(&self, name: String) -> Result<()>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::session().await?;
    let greeter = MyGreeter1Proxy::new(&conn).await?;
    greeter.set_greeter_name("Namesss".to_string()).await.ok();

    let mut greeted_happened = greeter.receive_greeted_everyone().await?;
    let mut bye_happened = greeter.receive_bye_everyone().await?;
    let mut name_changed = greeter.receive_greeter_name_changed().await;

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
        },
        async {
            while let Some(signal) = name_changed.next().await {
                let name = signal.get().await?;
                println!("new name is: {name}");
            }
            println!("end");
            Ok::<(), zbus::Error>(())
        }
    )?;

    Ok(())
}
