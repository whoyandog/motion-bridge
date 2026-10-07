use crate::motion_bridge_schema::{
    BodyFrame, CaptureEventPacket, ProtoQuaternion, ProtoVector3, RigUpdatePacket,
    capture_event_packet::Payload, rig_update_packet,
};
use glam::{Quat, Vec3};
use tokio::sync::watch;

const LEFT_SHOULDER: usize = 11;
const LEFT_ELBOW: usize = 13;
const LEFT_WRIST: usize = 15;

pub async fn run(
    mut rx_raw: watch::Receiver<Option<CaptureEventPacket>>,
    tx_solved: watch::Sender<Option<RigUpdatePacket>>,
) {
    let default_bone_dir = Vec3::NEG_Y;

    loop {
        if rx_raw.changed().await.is_err() {
            println!("[Solver] Канал закрыт.");
            break;
        }

        let event_opt = rx_raw.borrow().clone();

        if let Some(event) = event_opt {
            if let Some(Payload::Body(body)) = event.payload {
                let stride = 4;

                if body.landmarks_data.len() >= 33 * stride {
                    let sh_idx = LEFT_SHOULDER * stride;
                    let el_idx = LEFT_ELBOW * stride;
                    let wr_idx = LEFT_WRIST * stride;

                    let v_shoulder = Vec3::new(
                        body.landmarks_data[sh_idx],
                        body.landmarks_data[sh_idx + 1],
                        body.landmarks_data[sh_idx + 2],
                    );

                    let v_elbow = Vec3::new(
                        body.landmarks_data[el_idx],
                        body.landmarks_data[el_idx + 1],
                        body.landmarks_data[el_idx + 2],
                    );

                    let v_wrist = Vec3::new(
                        body.landmarks_data[wr_idx],
                        body.landmarks_data[wr_idx + 1],
                        body.landmarks_data[wr_idx + 2],
                    );

                    let upper_left_arm_dir = (v_elbow - v_shoulder).normalize();
                    let lower_left_arm_dir = (v_wrist - v_elbow).normalize();

                    let upper_left_arm_quat =
                        Quat::from_rotation_arc(default_bone_dir, upper_left_arm_dir);
                    let lower_left_arm_quat =
                        Quat::from_rotation_arc(default_bone_dir, lower_left_arm_dir);

                    let rig_update_packet = RigUpdatePacket {
                        timestamp: event.timestamp,
                        actor_id: event.actor_id,
                        payload: Some(rig_update_packet::Payload::Body(BodyFrame {
                            root_position: Some(ProtoVector3 {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            }),
                            bone_rotations: vec![
                                ProtoQuaternion {
                                    x: upper_left_arm_quat.x,
                                    y: upper_left_arm_quat.y,
                                    z: upper_left_arm_quat.z,
                                    w: upper_left_arm_quat.w,
                                },
                                ProtoQuaternion {
                                    x: lower_left_arm_quat.x,
                                    y: lower_left_arm_quat.y,
                                    z: lower_left_arm_quat.z,
                                    w: lower_left_arm_quat.w,
                                },
                            ],
                        })),
                    };

                    let _ = tx_solved.send(Some(rig_update_packet));

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
}
