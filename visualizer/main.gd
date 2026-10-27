extends Node3D

func _ready() -> void:
	var receiver = PoseReceiver.new()
	add_child(receiver)
	
	var skeleton = SkeletonVisualizer.new()
	skeleton.receiver = receiver
	add_child(skeleton)
