class_name SkeletonVisualizer 
extends Skeleton3D

var receiver: PoseReceiver

var rust_to_godot_bones = {
	1: "LeftUpperArm"
}

func _ready() -> void:
	_build_skeleton()
	if receiver: 
		receiver.frame_received.connect(_on_frame_received)
	
func _build_skeleton() -> void: 
	add_bone("Root")
	add_bone("LeftUpperArm")

	var root_idx = find_bone("Root")
	var upper_idx = find_bone("LeftUpperArm")

	set_bone_parent(upper_idx, root_idx)
	set_bone_rest(upper_idx, Transform3D(Basis(), Vector3(0, 1.0, 0)))
	
	_attach_mesh("LeftUpperArm")
	
func _attach_mesh(bone_name: String) -> void:
	var attachment = BoneAttachment3D.new()
	attachment.bone_name = bone_name
	add_child(attachment)
	
	var mesh_instance = MeshInstance3D.new()
	var cylinder = CylinderMesh.new()
	cylinder.top_radius = 0.04
	cylinder.bottom_radius = 0.03
	cylinder.height = 0.4
	mesh_instance.mesh = cylinder
	
	mesh_instance.position = Vector3(0, -0.2, 0)
	attachment.add_child(mesh_instance)

func _on_frame_received(frame) -> void: 
	var bone_rotations = frame.get_bone_rotations()
	for i in range(bone_rotations.size()): 
		var bone_data = bone_rotations[i]
		var rust_id = i 

		if rust_to_godot_bones.has(rust_id):
			var bone_name = rust_to_godot_bones[rust_id]
			var bone_idx = find_bone(bone_name)
				
			if bone_idx != -1:
				var q = Quaternion (
					bone_data.get_x(),
					bone_data.get_y(),
					bone_data.get_z(),
					bone_data.get_w()
				)

				set_bone_pose_rotation(bone_idx, q)

