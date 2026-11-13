use glam::{Quat, Vec3};

pub const BODY_JOINT_COUNT: usize = 20;
pub const HUMANOID_BONE_COUNT: usize = 56;

#[derive(Debug, Clone)]
pub enum SolverEvent {
    Handshake {
        model_name: String,
    },
    BodyFrame {
        timestamp: u64,
        actor_id: u32,
        landmarks_data: Vec<f32>,
    },
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnifiedBodyJoint {
    Hips = 0,
    Spine = 1,
    Neck = 2,
    Head = 3,
    LeftShoulder = 4,
    LeftElbow = 5,
    LeftWrist = 6,
    RightShoulder = 7,
    RightElbow = 8,
    RightWrist = 9,
    LeftHip = 10,
    LeftKnee = 11,
    LeftAnkle = 12,
    RightHip = 13,
    RightKnee = 14,
    RightAnkle = 15,
    LeftHeel = 16,
    LeftToe = 17,
    RightHeel = 18,
    RightToe = 19,
}

#[derive(Debug, Clone, Copy)]
pub struct JointData {
    pub position: Vec3,
    pub visibility: f32,
}

#[derive(Debug, Clone)]
pub struct UnifiedBodyPose {
    pub timestamp: u64,
    pub actor_id: u32,
    pub joints: [Option<JointData>; BODY_JOINT_COUNT],
}

impl UnifiedBodyPose {
    pub fn new(timestamp: u64, actor_id: u32) -> Self {
        Self {
            timestamp,
            actor_id,
            joints: [None; BODY_JOINT_COUNT],
        }
    }

    #[inline]
    pub fn get(&self, joint: UnifiedBodyJoint) -> Option<&JointData> {
        self.joints[joint as usize].as_ref()
    }

    #[inline]
    pub fn set(&mut self, joint: UnifiedBodyJoint, data: Option<JointData>) {
        self.joints[joint as usize] = data;
    }
}

#[derive(Debug, Clone)]
pub struct SolvedRigState {
    pub root_position: Vec3,
    pub local_rotations: [Quat; HUMANOID_BONE_COUNT],
    pub global_rotations: [Quat; HUMANOID_BONE_COUNT],
}

impl Default for SolvedRigState {
    fn default() -> Self {
        Self {
            root_position: Vec3::ZERO,
            local_rotations: [Quat::IDENTITY; HUMANOID_BONE_COUNT],
            global_rotations: [Quat::IDENTITY; HUMANOID_BONE_COUNT],
        }
    }
}
