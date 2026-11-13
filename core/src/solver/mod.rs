pub mod adapters;
pub mod types;

use crate::motion_bridge_schema::{
    BodyFrame, ProtoQuaternion, ProtoVector3, RigUpdatePacket, rig_update_packet,
};
use adapters::{PoseAdapter, create_adapter};
use glam::{Quat, Vec3};
use tokio::sync::{mpsc, watch};
use types::{SolvedRigState, SolverEvent, UnifiedBodyJoint as J};

pub async fn run(
    mut rx_raw: mpsc::Receiver<SolverEvent>,
    tx_solved: watch::Sender<Option<RigUpdatePacket>>,
) {
    let mut adapter: Option<Box<dyn PoseAdapter>> = None;
    let mut rig_state = SolvedRigState::default();
    let default_bone_dir = Vec3::NEG_Y;

    while let Some(event) = rx_raw.recv().await {
        match event {
            SolverEvent::Handshake { model_name } => match create_adapter(&model_name) {
                Some(new_adapter) => {
                    println!("[Solver] Активирован адаптер: {}", model_name);
                    adapter = Some(new_adapter);
                }
                None => {
                    eprintln!("[Solver] Неизвестная модель в Handshake: {}", model_name);
                }
            },
            SolverEvent::BodyFrame {
                timestamp,
                actor_id,
                landmarks_data,
            } => {
                let Some(active_adapter) = adapter.as_ref() else {
                    eprintln!("[Solver] Модель не выбрана, ожидание Handshake.");
                    continue;
                };

                let Some(pose) = active_adapter.adapt(timestamp, actor_id, &landmarks_data) else {
                    eprintln!("[Solver] Ошибка адаптации кадра.");
                    continue;
                };

                if let Some(hips) = pose.get(J::Hips) {
                    rig_state.root_position = hips.position;
                }

                if let (Some(sh), Some(el), Some(wr)) = (
                    pose.get(J::LeftShoulder),
                    pose.get(J::LeftElbow),
                    pose.get(J::LeftWrist),
                ) {
                    let upper_left_arm_dir = (el.position - sh.position).normalize_or_zero();
                    let lower_left_arm_dir = (wr.position - el.position).normalize_or_zero();

                    let upper_left_arm_quat =
                        Quat::from_rotation_arc(default_bone_dir, upper_left_arm_dir);
                    let lower_left_arm_quat =
                        Quat::from_rotation_arc(default_bone_dir, lower_left_arm_dir);

                    let rig_update_packet = RigUpdatePacket {
                        timestamp: pose.timestamp,
                        actor_id: pose.actor_id,
                        payload: Some(rig_update_packet::Payload::Body(BodyFrame {
                            root_position: Some(ProtoVector3 {
                                x: rig_state.root_position.x,
                                y: rig_state.root_position.y,
                                z: rig_state.root_position.z,
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
                        "[Solved] Hips: ({:.2}, {:.2}, {:.2}) | UpperArm: ({:.2}, {:.2}, {:.2}, {:.2})",
                        rig_state.root_position.x,
                        rig_state.root_position.y,
                        rig_state.root_position.z,
                        upper_left_arm_quat.x,
                        upper_left_arm_quat.y,
                        upper_left_arm_quat.z,
                        upper_left_arm_quat.w,
                    );
                }
            }
        }
    }

    println!("[Solver] Канал закрыт.");
}
