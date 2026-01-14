extends Resource
class_name PaciProject

var title:String
var speaker_base:SpeakerBase


func _init()->void:
	speaker_base = SpeakerBase.new()
