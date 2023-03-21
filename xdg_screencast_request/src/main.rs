use std::collections::HashMap;
use std::error::Error;
//use zbus::export::futures_util::{FutureExt, StreamExt};
use std::future::pending;
use zbus::export::futures_util::StreamExt;
use zbus::zvariant::{ObjectPath, OwnedObjectPath, Value};
use zbus::{dbus_proxy, Connection};
#[dbus_proxy(
    interface = "org.freedesktop.portal.ScreenCast",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait ScreenCast {
    fn create_session(&self, options: HashMap<String, Value<'_>>) -> zbus::Result<OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "org.freedesktop.portal.Request",
    default_service = "org.freedesktop.portal.Desktop"
)]
trait Request {
    #[dbus_proxy(signal)]
    fn response(&self, response: u32, results: HashMap<String, Value<'_>>) -> zbus::Result<()>;
    fn close(&self) -> zbus::Result<()>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let uniqname = connection.unique_name().unwrap();
    let uniquname_identifier = uniqname.trim_start_matches(":").replace(".", "_");
    let object = ObjectPath::try_from(format!(
        "/org/freedesktop/portal/desktop/request/{uniquname_identifier}/steinsgate"
    ))
    .unwrap();
    let screencast = ScreenCastProxy::new(&connection).await?;
    tokio::spawn(async move {
        let request = RequestProxy::builder(&connection)
            .path(object)?
            .build()
            .await?;
        let mut signal = request.receive_response().await?;
        while let Some(sig) = signal.next().await {
            println!("Greate, you have receive the signal!");
            println!("{:?}", sig);
            let body: (u32, HashMap<String, Value>) = sig.body().unwrap();
            println!("{:?}", body);
        }
        Ok::<(), zbus::Error>(())
    });

    let mut input: HashMap<String, Value<'_>> = HashMap::new();
    input.insert(
        "session_handle_token".to_string(),
        Value::Str("steinsgate".into()),
    );
    input.insert("handle_token".to_string(), Value::Str("steinsgate".into()));

    let creatsession = screencast.create_session(input).await?;
    dbg!(creatsession);
    pending::<()>().await;

    Ok(())
}
