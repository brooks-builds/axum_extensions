mod router;

use std::net::SocketAddr;

use eyre::Result;
use router::create_router;

pub struct App {
    address: [u8; 4],
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        let address = [127, 0, 0, 1];

        Self { address, port }
    }

    pub async fn run(&self) -> Result<()> {
        let router = create_router(self.address, self.port);

        tracing_subscriber::fmt::init();

        let address = SocketAddr::from((self.address, self.port));

        tracing::info!("Server listening on port {}", self.port);

        axum::Server::bind(&address)
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }
}
