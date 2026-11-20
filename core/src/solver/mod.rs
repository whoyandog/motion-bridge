pub mod adapters;
pub mod kinematics;
pub mod types;

use crate::motion_bridge_schema::{
    BodyFrame, ProtoQuaternion, ProtoVector3, RigUpdatePacket, rig_update_packet,
};
use adapters::{PoseAdapter, create_adapter};
use kinematics::KinematicsSolver;
use tokio::sync::{mpsc, watch};
use types::{HUMANOID_BONE_COUNT, SolvedRigState, SolverEvent};

pub async fn run(
    mut rx_raw: mpsc::Receiver<SolverEvent>,
    tx_solved: watch::Sender<Option<RigUpdatePacket>>,
) {
    let mut adapter: Option<Box<dyn PoseAdapter>> = None;
    let mut rig_state = SolvedRigState::default();

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

                KinematicsSolver::solve_body(&pose, &mut rig_state);

                let mut bone_rotations = Vec::with_capacity(HUMANOID_BONE_COUNT);
                for q in &rig_state.local_rotations {
                    bone_rotations.push(ProtoQuaternion {
                        x: q.x,
                        y: q.y,
                        z: q.z,
                        w: q.w,
                    });
                }

                let rig_update_packet = RigUpdatePacket {
                    timestamp: pose.timestamp,
                    actor_id: pose.actor_id,
                    payload: Some(rig_update_packet::Payload::Body(BodyFrame {
                        root_position: Some(ProtoVector3 {
                            x: rig_state.root_position.x,
                            y: rig_state.root_position.y,
                            z: rig_state.root_position.z,
                        }),
                        bone_rotations,
                    })),
                };

                let _ = tx_solved.send(Some(rig_update_packet));
            }
        }
    }

    println!("[Solver] Канал закрыт.");
}
