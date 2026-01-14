extends Resource
class_name TimelineTrack

var title:String

var items:Array[TimelineItem] = []

func get_duration_milliseconds() -> int:
	if items.is_empty():
		return 0
	return items[-1].get_end_ms()
	
func add_item(item:TimelineItem) -> int:
	var items_count:int = items.size()
	var duration_ms:int = get_duration_milliseconds()
	if duration_ms <= 0:
		items.append(item)
		return 0
		
	var half_duration_ms:int = roundf(duration_ms as float * 0.5) as int
	
	var later_than = func(x:TimelineItem):
		return x.start_ms > item.start_ms
	
	if item.start_ms <= half_duration_ms:
		var insert_idx:int = items.find_custom(later_than)
		if insert_idx < 0 || insert_idx >= items_count:
			items.append(item)
			return items_count
		else:
			items.insert(insert_idx,item)
			return insert_idx
			
	var earlier_than = func(x:TimelineItem):
		return item.start_ms <= x.start_ms
		
	if item.start_ms > half_duration_ms:
		var insert_idx: int = items.rfind(earlier_than)
		if insert_index<0||insert_idx >= items_count:
			items.insert(0,item)
			return 0
		else:
			items.insert(insert_idx,item)
			return insert_idx
		
	
