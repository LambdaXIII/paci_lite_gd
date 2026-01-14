extends TimelineTrack
class_name SubtitleTrack


const MAX_PREVIEW_LINES:int = 10

#func get_preview_lines() -> Array[String]:
	#var result:Array[String] = []
	#
	#var items = get_items()
	#if items.is_empty():
		#return result
		#
	#var preview_end_index = min(MAX_PREVIEW_LINES,items.size())
	#for i in items.slice(0,preview_end_index):
		#result.append((i as SubtitleClip).content)
	#return result
	#
