use crate::solver::types::{
    HumanoidBone as B, SolvedRigState, UnifiedBodyJoint as J, UnifiedBodyPose,
};
use glam::{Mat3, Quat, Vec3};

const MIN_VISIBILITY: f32 = 0.3;
const MIN_VEC_LEN_SQ: f32 = 1e-5;

pub struct KinematicsSolver;

impl KinematicsSolver {
    pub fn solve_body(pose: &UnifiedBodyPose, state: &mut SolvedRigState) {
        if let Some(hips) = Self::joint_pos(pose, J::Hips) {
            state.root_position = hips;
        }

        state.set_global(B::Root, Quat::IDENTITY);

        Self::solve_torso_and_head(pose, state);

        Self::solve_limb_chain(
            pose,
            state,
            &[
                (
                    B::LeftUpperArm,
                    B::LeftShoulder,
                    J::LeftShoulder,
                    J::LeftElbow,
                    Vec3::NEG_Y,
                ),
                (
                    B::LeftLowerArm,
                    B::LeftUpperArm,
                    J::LeftElbow,
                    J::LeftWrist,
                    Vec3::NEG_Y,
                ),
            ],
        );

        Self::solve_limb_chain(
            pose,
            state,
            &[
                (
                    B::RightUpperArm,
                    B::RightShoulder,
                    J::RightShoulder,
                    J::RightElbow,
                    Vec3::NEG_Y,
                ),
                (
                    B::RightLowerArm,
                    B::RightUpperArm,
                    J::RightElbow,
                    J::RightWrist,
                    Vec3::NEG_Y,
                ),
            ],
        );

        Self::solve_limb_chain(
            pose,
            state,
            &[
                (
                    B::LeftUpperLeg,
                    B::Hips,
                    J::LeftHip,
                    J::LeftKnee,
                    Vec3::NEG_Y,
                ),
                (
                    B::LeftLowerLeg,
                    B::LeftUpperLeg,
                    J::LeftKnee,
                    J::LeftAnkle,
                    Vec3::NEG_Y,
                ),
                (
                    B::LeftFoot,
                    B::LeftLowerLeg,
                    J::LeftAnkle,
                    J::LeftToe,
                    Vec3::NEG_Z,
                ),
            ],
        );

        Self::solve_limb_chain(
            pose,
            state,
            &[
                (
                    B::RightUpperLeg,
                    B::Hips,
                    J::RightHip,
                    J::RightKnee,
                    Vec3::NEG_Y,
                ),
                (
                    B::RightLowerLeg,
                    B::RightUpperLeg,
                    J::RightKnee,
                    J::RightAnkle,
                    Vec3::NEG_Y,
                ),
                (
                    B::RightFoot,
                    B::RightLowerLeg,
                    J::RightAnkle,
                    J::RightToe,
                    Vec3::NEG_Z,
                ),
            ],
        );

        state.update_local_from_global();
    }

    #[inline]
    fn joint_pos(pose: &UnifiedBodyPose, joint: J) -> Option<Vec3> {
        pose.get(joint)
            .filter(|jd| jd.visibility >= MIN_VISIBILITY)
            .map(|jd| jd.position)
    }

    fn solve_torso_and_head(pose: &UnifiedBodyPose, state: &mut SolvedRigState) {
        let hips_pos = Self::joint_pos(pose, J::Hips);
        let spine_pos = Self::joint_pos(pose, J::Spine);
        let neck_pos = Self::joint_pos(pose, J::Neck);
        let head_pos = Self::joint_pos(pose, J::Head);

        if let (Some(h), Some(s), Some(lh), Some(rh)) = (
            hips_pos,
            spine_pos,
            Self::joint_pos(pose, J::LeftHip),
            Self::joint_pos(pose, J::RightHip),
        ) {
            let right_hint = (lh - rh).normalize_or_zero();
            let up = (s - h).normalize_or_zero();
            let q = Self::basis_from_up_and_right(up, right_hint, state.get_global(B::Root));
            state.set_global(B::Hips, q);
        } else {
            state.set_global(B::Hips, state.get_global(B::Root));
        }

        if let (Some(s), Some(n), Some(ls), Some(rs)) = (
            spine_pos,
            neck_pos,
            Self::joint_pos(pose, J::LeftShoulder),
            Self::joint_pos(pose, J::RightShoulder),
        ) {
            let right_hint = (ls - rs).normalize_or_zero();
            let up = (n - s).normalize_or_zero();
            let q = Self::basis_from_up_and_right(up, right_hint, state.get_global(B::Hips));
            state.set_global(B::Spine, q);
            state.set_global(B::Chest, q);
            state.set_global(B::UpperChest, q);
        } else {
            let parent_q = state.get_global(B::Hips);
            state.set_global(B::Spine, parent_q);
            state.set_global(B::Chest, parent_q);
            state.set_global(B::UpperChest, parent_q);
        }

        let upper_chest_q = state.get_global(B::UpperChest);
        state.set_global(B::LeftShoulder, upper_chest_q);
        state.set_global(B::RightShoulder, upper_chest_q);

        if let (Some(n), Some(hd)) = (neck_pos, head_pos) {
            let up = (hd - n).normalize_or_zero();
            let right_hint = match (
                Self::joint_pos(pose, J::LeftShoulder),
                Self::joint_pos(pose, J::RightShoulder),
            ) {
                (Some(ls), Some(rs)) => (ls - rs).normalize_or_zero(),
                _ => upper_chest_q * Vec3::X,
            };
            let q = Self::basis_from_up_and_right(up, right_hint, upper_chest_q);
            state.set_global(B::Neck, q);
            state.set_global(B::Head, q);
        } else {
            state.set_global(B::Neck, upper_chest_q);
            state.set_global(B::Head, upper_chest_q);
        }
    }

    fn solve_limb_chain(
        pose: &UnifiedBodyPose,
        state: &mut SolvedRigState,
        chain: &[(B, B, J, J, Vec3)],
    ) {
        for &(bone, parent_bone, from_joint, to_joint, rest_dir) in chain {
            let parent_global = state.get_global(parent_bone);
            if let (Some(from), Some(to)) = (
                Self::joint_pos(pose, from_joint),
                Self::joint_pos(pose, to_joint),
            ) {
                let dir = (to - from).normalize_or_zero();
                if dir.length_squared() > MIN_VEC_LEN_SQ {
                    let q = Quat::from_rotation_arc(rest_dir, dir);
                    state.set_global(bone, q);
                } else {
                    state.set_global(bone, parent_global);
                }
            } else {
                state.set_global(bone, parent_global);
            }
        }
    }

    fn basis_from_up_and_right(up: Vec3, right_hint: Vec3, fallback: Quat) -> Quat {
        if up.length_squared() < MIN_VEC_LEN_SQ || right_hint.length_squared() < MIN_VEC_LEN_SQ {
            return fallback;
        }
        let forward = right_hint.cross(up).normalize_or_zero();
        if forward.length_squared() < MIN_VEC_LEN_SQ {
            return fallback;
        }
        let right = up.cross(forward).normalize_or_zero();
        Quat::from_mat3(&Mat3::from_cols(right, up, forward))
    }
}
