extends Node3D

@export var move_speed := 5.0
@export var fast_multiplier := 3.0
@export var slow_multiplier := 0.3

func _process(delta):
	var speed = move_speed
	if Input.is_action_pressed("camera_fast"):
		speed *= fast_multiplier
	if Input.is_action_pressed("camera_slow"):
		speed *= slow_multiplier

	var dir := Vector3.ZERO

	if Input.is_action_pressed("move_forward"):
		dir -= transform.basis.z
	if Input.is_action_pressed("move_back"):
		dir += transform.basis.z
	if Input.is_action_pressed("move_left"):
		dir -= transform.basis.x
	if Input.is_action_pressed("move_right"):
		dir += transform.basis.x
	if Input.is_action_pressed("move_up"):
		dir += transform.basis.y
	if Input.is_action_pressed("move_down"):
		dir -= transform.basis.y

	if dir != Vector3.ZERO:
		global_position += dir.normalized() * speed * delta

@export var mouse_sensitivity := 0.002
var yaw := 0.0
var pitch := 0.0

func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		yaw -= event.relative.x * mouse_sensitivity
		pitch -= event.relative.y * mouse_sensitivity
		pitch = clamp(pitch, deg_to_rad(-85), deg_to_rad(85))
		rotation.y = yaw
		$Camera3D.rotation.x = pitch
	
	if event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
