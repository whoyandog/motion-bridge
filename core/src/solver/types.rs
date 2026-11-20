use glam::{Quat, Vec3};

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

#[derive(Debug, Clone, Copy)]
pub struct JointData {
    pub position: Vec3,
    pub visibility: f32,
}

pub const BODY_JOINT_COUNT: usize = UnifiedBodyJoint::COUNT;

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

impl UnifiedBodyJoint {
    pub const COUNT: usize = 20;
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

pub const HUMANOID_BONE_COUNT: usize = HumanoidBone::COUNT;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanoidBone {
    Root = 0,
    Hips = 1,
    Spine = 2,
    Chest = 3,
    UpperChest = 4,
    Neck = 5,
    Head = 6,
    LeftEye = 7,
    RightEye = 8,
    Jaw = 9,

    LeftShoulder = 10,
    LeftUpperArm = 11,
    LeftLowerArm = 12,
    LeftHand = 13,
    LeftThumbMetacarpal = 14,
    LeftThumbProximal = 15,
    LeftThumbDistal = 16,
    LeftIndexProximal = 17,
    LeftIndexIntermediate = 18,
    LeftIndexDistal = 19,
    LeftMiddleProximal = 20,
    LeftMiddleIntermediate = 21,
    LeftMiddleDistal = 22,
    LeftRingProximal = 23,
    LeftRingIntermediate = 24,
    LeftRingDistal = 25,
    LeftLittleProximal = 26,
    LeftLittleIntermediate = 27,
    LeftLittleDistal = 28,

    RightShoulder = 29,
    RightUpperArm = 30,
    RightLowerArm = 31,
    RightHand = 32,
    RightThumbMetacarpal = 33,
    RightThumbProximal = 34,
    RightThumbDistal = 35,
    RightIndexProximal = 36,
    RightIndexIntermediate = 37,
    RightIndexDistal = 38,
    RightMiddleProximal = 39,
    RightMiddleIntermediate = 40,
    RightMiddleDistal = 41,
    RightRingProximal = 42,
    RightRingIntermediate = 43,
    RightRingDistal = 44,
    RightLittleProximal = 45,
    RightLittleIntermediate = 46,
    RightLittleDistal = 47,

    LeftUpperLeg = 48,
    LeftLowerLeg = 49,
    LeftFoot = 50,
    LeftToes = 51,
    RightUpperLeg = 52,
    RightLowerLeg = 53,
    RightFoot = 54,
    RightToes = 55,
}

impl HumanoidBone {
    pub const COUNT: usize = 56;

    pub const ALL: [Self; Self::COUNT] = [
        Self::Root,
        Self::Hips,
        Self::Spine,
        Self::Chest,
        Self::UpperChest,
        Self::Neck,
        Self::Head,
        Self::LeftEye,
        Self::RightEye,
        Self::Jaw,
        Self::LeftShoulder,
        Self::LeftUpperArm,
        Self::LeftLowerArm,
        Self::LeftHand,
        Self::LeftThumbMetacarpal,
        Self::LeftThumbProximal,
        Self::LeftThumbDistal,
        Self::LeftIndexProximal,
        Self::LeftIndexIntermediate,
        Self::LeftIndexDistal,
        Self::LeftMiddleProximal,
        Self::LeftMiddleIntermediate,
        Self::LeftMiddleDistal,
        Self::LeftRingProximal,
        Self::LeftRingIntermediate,
        Self::LeftRingDistal,
        Self::LeftLittleProximal,
        Self::LeftLittleIntermediate,
        Self::LeftLittleDistal,
        Self::RightShoulder,
        Self::RightUpperArm,
        Self::RightLowerArm,
        Self::RightHand,
        Self::RightThumbMetacarpal,
        Self::RightThumbProximal,
        Self::RightThumbDistal,
        Self::RightIndexProximal,
        Self::RightIndexIntermediate,
        Self::RightIndexDistal,
        Self::RightMiddleProximal,
        Self::RightMiddleIntermediate,
        Self::RightMiddleDistal,
        Self::RightRingProximal,
        Self::RightRingIntermediate,
        Self::RightRingDistal,
        Self::RightLittleProximal,
        Self::RightLittleIntermediate,
        Self::RightLittleDistal,
        Self::LeftUpperLeg,
        Self::LeftLowerLeg,
        Self::LeftFoot,
        Self::LeftToes,
        Self::RightUpperLeg,
        Self::RightLowerLeg,
        Self::RightFoot,
        Self::RightToes,
    ];

    #[inline]
    pub const fn parent(self) -> Option<Self> {
        use HumanoidBone::*;
        match self {
            Root => None,
            Hips => Some(Root),
            Spine => Some(Hips),
            Chest => Some(Spine),
            UpperChest => Some(Chest),
            Neck => Some(UpperChest),
            Head => Some(Neck),

            LeftEye | RightEye | Jaw => Some(Head),

            LeftShoulder => Some(UpperChest),
            LeftUpperArm => Some(LeftShoulder),
            LeftLowerArm => Some(LeftUpperArm),
            LeftHand => Some(LeftLowerArm),

            LeftThumbMetacarpal | LeftIndexProximal | LeftMiddleProximal | LeftRingProximal
            | LeftLittleProximal => Some(LeftHand),

            LeftThumbProximal => Some(LeftThumbMetacarpal),
            LeftThumbDistal => Some(LeftThumbProximal),

            LeftIndexIntermediate => Some(LeftIndexProximal),
            LeftIndexDistal => Some(LeftIndexIntermediate),

            LeftMiddleIntermediate => Some(LeftMiddleProximal),
            LeftMiddleDistal => Some(LeftMiddleIntermediate),

            LeftRingIntermediate => Some(LeftRingProximal),
            LeftRingDistal => Some(LeftRingIntermediate),

            LeftLittleIntermediate => Some(LeftLittleProximal),
            LeftLittleDistal => Some(LeftLittleIntermediate),

            RightShoulder => Some(UpperChest),
            RightUpperArm => Some(RightShoulder),
            RightLowerArm => Some(RightUpperArm),
            RightHand => Some(RightLowerArm),

            RightThumbMetacarpal | RightIndexProximal | RightMiddleProximal | RightRingProximal
            | RightLittleProximal => Some(RightHand),

            RightThumbProximal => Some(RightThumbMetacarpal),
            RightThumbDistal => Some(RightThumbProximal),

            RightIndexIntermediate => Some(RightIndexProximal),
            RightIndexDistal => Some(RightIndexIntermediate),

            RightMiddleIntermediate => Some(RightMiddleProximal),
            RightMiddleDistal => Some(RightMiddleIntermediate),

            RightRingIntermediate => Some(RightRingProximal),
            RightRingDistal => Some(RightRingIntermediate),

            RightLittleIntermediate => Some(RightLittleProximal),
            RightLittleDistal => Some(RightLittleIntermediate),

            LeftUpperLeg => Some(Hips),
            LeftLowerLeg => Some(LeftUpperLeg),
            LeftFoot => Some(LeftLowerLeg),
            LeftToes => Some(LeftFoot),

            RightUpperLeg => Some(Hips),
            RightLowerLeg => Some(RightUpperLeg),
            RightFoot => Some(RightLowerLeg),
            RightToes => Some(RightFoot),
        }
    }

    const PARENT_INDICES: [Option<usize>; Self::COUNT] = {
        let mut table = [None; Self::COUNT];
        let mut i = 0;
        while i < Self::COUNT {
            table[i] = match Self::ALL[i].parent() {
                Some(p) => Some(p as usize),
                None => None,
            };
            i += 1;
        }
        table
    };

    #[inline]
    pub const fn parent_index(bone_idx: usize) -> Option<usize> {
        if bone_idx < Self::COUNT {
            Self::PARENT_INDICES[bone_idx]
        } else {
            None
        }
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

impl SolvedRigState {
    #[inline]
    pub fn get_global(&self, bone: HumanoidBone) -> Quat {
        self.global_rotations[bone as usize]
    }

    #[inline]
    pub fn set_global(&mut self, bone: HumanoidBone, rotation: Quat) {
        self.global_rotations[bone as usize] = rotation;
    }

    pub fn update_local_from_global(&mut self) {
        for i in 0..HUMANOID_BONE_COUNT {
            let global_rot = self.global_rotations[i];
            self.local_rotations[i] = match HumanoidBone::parent_index(i) {
                Some(parent_idx) => self.global_rotations[parent_idx].inverse() * global_rot,
                None => global_rot,
            };
        }
    }
}
