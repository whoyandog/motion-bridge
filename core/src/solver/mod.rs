use crate::mocap::MocapFrame;
use tokio::sync::watch;

pub async fn run(mut rx_raw: watch::Receiver<Option<MocapFrame>>) {
    loop {
        if rx_raw.changed().await.is_err() {
            println!("[Solver] Канал rx_raw закрыт.");
            break;
        }

        let frame_opt = rx_raw.borrow().clone();

        if let Some(frame) = frame_opt {
            if frame.landmarks.len() >= 33 {
                let shoulder = &frame.landmarks[11];
                let elbow = &frame.landmarks[13];
                let wrist = &frame.landmarks[15];

                println!(
                    "[Solver] плечо: ({:.2}, {:.2}, {:.2}), локоть: ({:.2}, {:.2}, {:.2}), кисть ({:.2}, {:.2}, {:.2})",
                    shoulder.x,
                    shoulder.y,
                    shoulder.z,
                    elbow.x,
                    elbow.y,
                    elbow.z,
                    wrist.x,
                    wrist.y,
                    wrist.z
                );
            } else {
                println!("[Solver] Получено меньше 33 точек");
            }
        }
    }
}
