use crate::solver::adapters::PoseAdapter;
use crate::solver::types::{JointData, UnifiedBodyJoint as J, UnifiedBodyPose};
use glam::Vec3;

const MP_LANDMARK_COUNT: usize = 33;
const STRIDE: usize = 4;
const REFERENCE_TORSO_LENGTH: f32 = 0.5;

pub struct MediaPipeAdapter;

impl MediaPipeAdapter {
    #[inline]
    fn read_joint(raw: &[f32], mp_idx: usize) -> JointData {
        let base = mp_idx * STRIDE;

        let position = Vec3::new(-(raw[base] - 0.5), -(raw[base + 1] - 0.5), raw[base + 2]);
        let visibility = raw[base + 3];
        JointData {
            position,
            visibility,
        }
    }

    #[inline]
    fn midpoint(a: JointData, b: JointData) -> JointData {
        JointData {
            position: (a.position + b.position) * 0.5,
            visibility: a.visibility.min(b.visibility),
        }
    }
}

impl PoseAdapter for MediaPipeAdapter {
    fn adapt(&self, timestamp: u64, actor_id: u32, raw: &[f32]) -> Option<UnifiedBodyPose> {
        if raw.len() < MP_LANDMARK_COUNT * STRIDE {
            return None;
        }

        let mut pose = UnifiedBodyPose::new(timestamp, actor_id);

        let l_ear = Self::read_joint(raw, 7);
        let r_ear = Self::read_joint(raw, 8);
        let l_shoulder = Self::read_joint(raw, 11);
        let r_shoulder = Self::read_joint(raw, 12);
        let l_hip = Self::read_joint(raw, 23);
        let r_hip = Self::read_joint(raw, 24);

        let hips = Self::midpoint(l_hip, r_hip);
        let neck = Self::midpoint(l_shoulder, r_shoulder);
        let spine = Self::midpoint(hips, neck);
        let head = Self::midpoint(l_ear, r_ear);

        let current_torso_len = (neck.position - hips.position).length();
        let scale = if current_torso_len > 1e-4 {
            REFERENCE_TORSO_LENGTH / current_torso_len
        } else {
            1.0
        };

        let scale_joint = |mut jd: JointData| -> Option<JointData> {
            jd.position *= scale;
            Some(jd)
        };

        pose.set(J::Hips, scale_joint(hips));
        pose.set(J::Spine, scale_joint(spine));
        pose.set(J::Neck, scale_joint(neck));
        pose.set(J::Head, scale_joint(head));

        pose.set(J::LeftShoulder, scale_joint(l_shoulder));
        pose.set(J::LeftElbow, scale_joint(Self::read_joint(raw, 13)));
        pose.set(J::LeftWrist, scale_joint(Self::read_joint(raw, 15)));

        pose.set(J::RightShoulder, scale_joint(r_shoulder));
        pose.set(J::RightElbow, scale_joint(Self::read_joint(raw, 14)));
        pose.set(J::RightWrist, scale_joint(Self::read_joint(raw, 16)));

        pose.set(J::LeftHip, scale_joint(l_hip));
        pose.set(J::LeftKnee, scale_joint(Self::read_joint(raw, 25)));
        pose.set(J::LeftAnkle, scale_joint(Self::read_joint(raw, 27)));

        pose.set(J::RightHip, scale_joint(r_hip));
        pose.set(J::RightKnee, scale_joint(Self::read_joint(raw, 26)));
        pose.set(J::RightAnkle, scale_joint(Self::read_joint(raw, 28)));

        pose.set(J::LeftHeel, scale_joint(Self::read_joint(raw, 29)));
        pose.set(J::RightHeel, scale_joint(Self::read_joint(raw, 30)));
        pose.set(J::LeftToe, scale_joint(Self::read_joint(raw, 31)));
        pose.set(J::RightToe, scale_joint(Self::read_joint(raw, 32)));

        Some(pose)
    }
}
