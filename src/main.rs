use color_eyre::eyre::Result;
use vaporz::{logging, tui::Tui, ui::app::App};

#[tokio::main]
async fn main() -> Result<()> {
    logging::init()?;
    let tui = Tui::new()?
        .tick_rate(1.0) // 4 ticks per second
        .frame_rate(2.0); // 30 frames per second
    let mut app = App::new();
    let result = app.run(tui).await;
    result?;
    Ok(())
}
