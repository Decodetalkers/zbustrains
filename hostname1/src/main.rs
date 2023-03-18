use zbus::{dbus_proxy, Connection, Result};

#[dbus_proxy(
    interface = "org.freedesktop.hostname1",
    default_service = "org.freedesktop.hostname1",
    default_path = "/org/freedesktop/hostname1"
)]
trait Hostname1 {
    fn describe(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn static_hostname(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn icon_name(&self) -> Result<String>;
    // add code here
}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;

    let proxy = Hostname1Proxy::new(&connection).await?;
    let hostname = proxy.static_hostname().await?;
    let iconname = proxy.icon_name().await?;
    let discribe = proxy.describe().await?;
    println!("hostname :{hostname}");
    println!("iconname :{iconname}");
    println!("discribe :{discribe}");
    Ok(())
}
