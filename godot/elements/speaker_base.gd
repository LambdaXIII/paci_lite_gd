extends Resource
class_name SpeakerBase

var speakers:Dictionary[StringName,SpeakerInfo]
var shortcuts:Dictionary[StringName,StringName]


func new_speaker(id:StringName) -> SpeakerInfo:
	var speaker = SpeakerInfo.new()
	speaker.speaker_id = id
	add_speaker(speaker)
	return speaker
	
func add_speaker(speaker:SpeakerInfo):
	speakers.set(speaker.speaker_id,speaker)
	
func set_shortcut(short_cut:StringName,speaker_id:StringName):
	assert(speakers.find_key(speaker_id))
	shortcuts.set(short_cut,speaker_id)
	
func get_speaker(speaker_id:StringName)->SpeakerInfo:
	return speakers.get(speaker_id)
	
func get_speaker_id(short_cut:StringName)->StringName:
	return shortcuts.get(short_cut)
	
