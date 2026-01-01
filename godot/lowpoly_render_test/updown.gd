extends MeshInstance3D

@export var amplitude := 0.5   # 移動幅（上下）
@export var speed := 1.0        # 動く速さ

var base_y := 0.0
var time := 0.0

func _ready():
	base_y = global_position.y

func _process(delta):
	time += delta * speed
	global_position.y = base_y + sin(time) * amplitude
