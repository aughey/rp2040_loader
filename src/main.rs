use anyhow::Result;
use axum::{extract::Multipart, extract::State, response::IntoResponse, routing::post, Router};
use axum_macros::debug_handler;
use clap::Parser;
use std::sync::{Arc, Mutex};

type AppState = Arc<Mutex<dyn FnMut(Vec<u8>) + Send>>;

// clap derive args
#[derive(Parser)]
struct Args {
    /// File to look for for changes that we will copy to the output
    #[clap(long)]
    watch_file: String,
}

struct FirmwareHandler {
    pending_firmware: Option<Vec<u8>>,
    firmware_path: String,
}
impl FirmwareHandler {
    fn new(firmware_path: String) -> Self {
        Self {
            pending_firmware: None,
            firmware_path,
        }
    }
    fn new_firmware(&mut self, data: Vec<u8>) {
        self.pending_firmware = Some(data);
        self.try_write_to_2040();
    }
    fn try_write_to_2040(&mut self) {
        println!("Trying to write firmware");
        match self.pending_firmware.as_ref() {
            None => {
                println!("No pending firmware");
            }
            Some(firmware) => {
                println!("Pending firmware exists");
                match std::path::Path::new(&self.firmware_path).exists() {
                    true => {
                        println!("Firmware location exists");
                        println!("Writing Firmware...");
                        // write firmware to file

                        match std::fs::write(self.firmware_path.clone() + "/firmware", firmware) {
                            Ok(_) => {
                                println!("Firmware written");
                                self.pending_firmware = None;
                            }
                            Err(e) => println!("Error writing firmware: {}", e),
                        }
                    }
                    false => {
                        println!("Firmware path doesn't exist: {}", self.firmware_path);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let handler = Arc::new(Mutex::new(FirmwareHandler::new(args.watch_file.clone())));

    #[allow(unreachable_code)] // we want Ok(()) without necessarily an associated Err.
    let monitor_plugin = {
        let handler = handler.clone();
        async move {
            loop {
                // pico not plugged in
                let searchpath = std::path::Path::new(&args.watch_file);
                loop {
                    if searchpath.exists() {
                        println!("Pico found");
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }

                // pico plugged in
                handler.lock().unwrap().try_write_to_2040();

                // wait for pico to be unplugged
                loop {
                    if !searchpath.exists() {
                        println!("Pico unplugged");
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
            Ok(())
        }
    };

    let on_new_firmware: AppState = {
        let handler = handler.clone();
        Arc::new(Mutex::new(move |data| {
            let mut handler = handler.lock().unwrap();
            handler.new_firmware(data);
        }))
    };

    let app = Router::new()
        .route("/upload", post(upload_handler))
        .with_state(on_new_firmware);

    println!("listening on http://localhost:3000");
    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());

    // join all tasks
    tokio::try_join!(server, monitor_plugin)?;

    Ok(())
}

#[debug_handler]
async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, String> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| format!("Could not get next field: {}", e))?
    {
        let name = field.name().unwrap();
        // only accept firmware content
        if name != "firmware" {
            continue;
        }
        let data = field
            .bytes()
            .await
            .map_err(|e| format!("Could not get field bytes: {}", e))?;

        let mut on_new_data = state.lock().unwrap();

        on_new_data(data.to_vec());

        //        println!("Length of `{}` is {} bytes", name, data.len());
    }
    Ok(())
}
