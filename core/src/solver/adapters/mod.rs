pub mod mediapipe;

use crate::solver::types::UnifiedBodyPose;
use mediapipe::MediaPipeAdapter;

pub trait PoseAdapter: Send + Sync {
    fn adapt(&self, timestamp: u64, actor_id: u32, raw: &[f32]) -> Option<UnifiedBodyPose>;
}

pub fn create_adapter(model_name: &str) -> Option<Box<dyn PoseAdapter>> {
    match model_name {
        "mediapipe_33" => Some(Box::new(MediaPipeAdapter)),
        _ => None,
    }
}
