extends Resource
class_name TimelineItem

@export
var start_ms:int

@export
var duration_ms:int

func get_end_ms() -> int:
	return start_ms + duration_ms
	
