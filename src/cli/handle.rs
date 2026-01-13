use daemonize::Daemonize;
use tracing::info;

use crate::{app::App, cli::Cli};

impl Cli {
    pub async fn handle(self) {
        if self.detach {
            info!("detach icypeak from the current terminal");
            Daemonize::new()
                .start()
                .expect("unable to detach icypeak from the current terminal");
        }

        iced::daemon(App::new, App::update, App::view)
            .run()
            .expect("unable to run icypeak");
    }
}
