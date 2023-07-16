use crate::error::Result;
use std::io::prelude::*;
use std::os::unix::net::UnixListener;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::{fs, usize};
use xdg::BaseDirectories;

pub struct Socket {
    pub count: Arc<RwLock<(usize, usize)>>,
}

impl Clone for Socket {
    fn clone(&self) -> Self {
        Self {
            count: Arc::clone(&self.count),
        }
    }
}

impl Socket {
    pub fn init(xdg_dirs: &BaseDirectories) -> Result<Self> {
        let socket_path = xdg_dirs.place_runtime_file("socket")?;

        if socket_path.exists() {
            fs::remove_file(&socket_path)?;
        }

        let stream = UnixListener::bind(&socket_path)?;
        let count: (usize, usize) = (0, 0);
        let count = Arc::new(RwLock::new(count));
        let count_cloned = count.clone();

        println!("UNIX socket listening at {}", socket_path.display());

        thread::spawn(move || loop {
            for mut client in stream.incoming().flatten() {
                let mut response = String::new();
                client
                    .set_read_timeout(Some(Duration::from_millis(10)))
                    .unwrap();

                let _read = client.read_to_string(&mut response);

                let (normal, urgent) = *count_cloned.read().unwrap();
                let message = format!("{},{}", normal, urgent);

                match client.write(message.as_bytes()) {
                    Ok(_) => tracing::debug!("Sent counts: {}", message),
                    Err(e) => tracing::error!("Error sending notification counts: {}", e),
                }
            }
        });

        Ok(Self { count })
    }

    pub fn update(&self, counts: (usize, usize)) {
        let mut count = self.count.write().unwrap();
        *count = counts;
    }
}
