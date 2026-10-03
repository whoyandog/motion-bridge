use crate::mocap::MocapFrame;
use glam::{Quat, Vec3};
use tokio::sync::watch;

const LEFT_SHOULDER: usize = 11;
const LEFT_ELBOW: usize = 13;
const LEFT_WRIST: usize = 15;

pub async fn run(mut rx_raw: watch::Receiver<Option<MocapFrame>>) {
    let default_bone_dir = Vec3::NEG_Y;

    loop {
        if rx_raw.changed().await.is_err() {
            println!("[Solver] Канал rx_raw закрыт.");
            break;
        }

        let frame_opt = rx_raw.borrow().clone();

        if let Some(frame) = frame_opt {
            if frame.landmarks.len() >= 33 {
                let shoulder = &frame.landmarks[LEFT_SHOULDER];
                let elbow = &frame.landmarks[LEFT_ELBOW];
                let wrist = &frame.landmarks[LEFT_WRIST];

                let v_shoulder = Vec3::new(shoulder.x, shoulder.y, shoulder.z);
                let v_elbow = Vec3::new(elbow.x, elbow.y, elbow.z);
                let v_wrist = Vec3::new(wrist.x, wrist.y, wrist.z);

                let upper_left_arm_dir = (v_elbow - v_shoulder).normalize();
                let lower_left_arm_dir = (v_wrist - v_elbow).normalize();

                let upper_left_arm_quat =
                    Quat::from_rotation_arc(default_bone_dir, upper_left_arm_dir);
                let lower_left_arm_quat =
                    Quat::from_rotation_arc(default_bone_dir, lower_left_arm_dir);

                println!(
                    "[Solver] Кватернион, верхняя часть: ({:.2}, {:.2}, {:.2}, {:.2}); нижняя часть: ({:.2}, {:.2}, {:.2}, {:.2})",
                    upper_left_arm_quat.x,
                    upper_left_arm_quat.y,
                    upper_left_arm_quat.z,
                    upper_left_arm_quat.w,
                    lower_left_arm_quat.x,
                    lower_left_arm_quat.y,
                    lower_left_arm_quat.z,
                    lower_left_arm_quat.w
                );
            } else {
                println!("[Solver] Получено меньше 33 точек");
            }
        }
    }
}
