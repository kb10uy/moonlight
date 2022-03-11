mod app;
mod preview;
mod tracking;

use crate::{preview::PreviewWindow, tracking::TrackingState};

use std::{thread::sleep, time::Duration};

use anyhow::Result;
use flexi_logger::Logger;
use moonlight_openvr::{ApplicationType, Context};

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?.start()?;

    let preview_window = PreviewWindow::create_window().await?;
    preview_window.run();

    let ovr_context = Context::new(ApplicationType::Background)?;
    let ovr_system = ovr_context.system()?;

    let mut tracked_state = TrackingState::new();
    loop {
        tracked_state.update(&ovr_system)?;

        for device in tracked_state.tracked_devices() {
            println!("{:?}: {:?}", device.description(), device.position());
        }

        println!();
        sleep(Duration::from_millis(100));
    }
}
