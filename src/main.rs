use opencv::{core, highgui, imgcodecs, imgproc, objdetect, prelude::*, types, videoio, Result};

fn main() -> Result<()> {
    let mut qr_detector = objdetect::QRCodeDetector::default()?;
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    let mut res = types::VectorOfPoint::new();
    let mut qr_img = Mat::default();

    loop {
        camera.read(&mut frame)?;
        let data = qr_detector.detect_and_decode(&frame, &mut res, &mut qr_img)?;

        if res.len() > 0 {
            imgproc::polylines(
                &mut frame,
                &res,
                true,
                core::Scalar::new(0f64, 255f64, 255f64, 0f64),
                1,
                1,
                0,
            )?;
        }
        highgui::named_window("QR Code", highgui::WINDOW_NORMAL)?;
        if qr_img.size()?.width > 0 {
            highgui::imshow("QR Code", &qr_img)?;
        }

        let s = String::from_utf8_lossy(&data);
        println!("{s:?}");
        println!("{res:?}");
        highgui::imshow("Frame", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 'q' as i32 {
            break;
        }
    }

    Ok(())
}
