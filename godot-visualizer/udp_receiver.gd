class_name PoseReceiver
extends Node

signal frame_received(solver_frame)

const PORT = 4242
var server := PacketPeerUDP.new()

const Schema = preload("res://proto/schema.pb.gd")

func _ready() -> void:
	var err = server.bind(PORT)
	if err == OK:
		print("UDP Receiver запущен на порту: ", PORT)
	else:
		push_error("Не удалось забиндить UDP порт: ", PORT)

func _process(_delta: float) -> void:
	while server.get_available_packet_count() > 0:
		var packet := server.get_packet()
		_decode_packet(packet)

func _decode_packet(packet: PackedByteArray) -> void:
	var frame = Schema.SolverFrame.new()
	
	var result = frame.from_bytes(packet)
	
	if result == 0:
		emit_signal("frame_received", frame)
	else:
		push_warning("Ошибка парсинга пакета, код ошибки: ", result)
