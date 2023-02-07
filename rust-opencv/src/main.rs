use opencv::{highgui, prelude::*, videoio, Result};

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let name = "video";

    highgui::named_window(name, highgui::WINDOW_FULLSCREEN)?;
    let mut frame = Mat::default();

    loop {
        cam.read(&mut frame)?;
        highgui::imshow(name, &frame)?;
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    Ok(())
}
