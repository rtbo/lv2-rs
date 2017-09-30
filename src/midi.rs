

pub const URI: &'static str = "http://lv2plug.in/ns/ext/midi";

pub mod class {
    pub const ACTIVESENSE:      &'static str = "http://lv2plug.in/ns/ext/midi#ActiveSense";
    pub const AFTERTOUCH:       &'static str = "http://lv2plug.in/ns/ext/midi#Aftertouch";
    pub const BENDER:           &'static str = "http://lv2plug.in/ns/ext/midi#Bender";
    pub const CHANNELPRESSURE:  &'static str = "http://lv2plug.in/ns/ext/midi#ChannelPressure";
    pub const CHUNK:            &'static str = "http://lv2plug.in/ns/ext/midi#Chunk";
    pub const CLOCK:            &'static str = "http://lv2plug.in/ns/ext/midi#Clock";
    pub const CONTINUE:         &'static str = "http://lv2plug.in/ns/ext/midi#Continue";
    pub const CONTROLLER:       &'static str = "http://lv2plug.in/ns/ext/midi#Controller";
    pub const MIDIEVENT:        &'static str = "http://lv2plug.in/ns/ext/midi#MidiEvent";
    pub const NOTEOFF:          &'static str = "http://lv2plug.in/ns/ext/midi#NoteOff";
    pub const NOTEON:           &'static str = "http://lv2plug.in/ns/ext/midi#NoteOn";
    pub const PROGRAMCHANGE:    &'static str = "http://lv2plug.in/ns/ext/midi#ProgramChange";
    pub const QUARTERFRAME:     &'static str = "http://lv2plug.in/ns/ext/midi#QuarterFrame";
    pub const RESET:            &'static str = "http://lv2plug.in/ns/ext/midi#Reset";
    pub const SONGPOSITION:     &'static str = "http://lv2plug.in/ns/ext/midi#SongPosition";
    pub const SONGSELECT:       &'static str = "http://lv2plug.in/ns/ext/midi#SongSelect";
    pub const START:            &'static str = "http://lv2plug.in/ns/ext/midi#Start";
    pub const STOP:             &'static str = "http://lv2plug.in/ns/ext/midi#Stop";
    pub const SYSTEMCOMMON:     &'static str = "http://lv2plug.in/ns/ext/midi#SystemCommon";
    pub const SYSTEMEXCLUSIVE:  &'static str = "http://lv2plug.in/ns/ext/midi#SystemExclusive";
    pub const SYSTEMMESSAGE:    &'static str = "http://lv2plug.in/ns/ext/midi#SystemMessage";
    pub const SYSTEMREALTIME:   &'static str = "http://lv2plug.in/ns/ext/midi#SystemRealtime";
    pub const TICK:             &'static str = "http://lv2plug.in/ns/ext/midi#Tick";
    pub const TUNEREQUEST:      &'static str = "http://lv2plug.in/ns/ext/midi#TuneRequest";
    pub const VOICEMESSAGE:     &'static str = "http://lv2plug.in/ns/ext/midi#VoiceMessage";
}

pub mod prop {
    pub const BENDERVALUE:      &'static str = "http://lv2plug.in/ns/ext/midi#benderValue";
    pub const BINDING:          &'static str = "http://lv2plug.in/ns/ext/midi#binding";
    pub const BYTENUMBER:       &'static str = "http://lv2plug.in/ns/ext/midi#byteNumber";
    pub const CHANNEL:          &'static str = "http://lv2plug.in/ns/ext/midi#channel";
    pub const CHUNK:            &'static str = "http://lv2plug.in/ns/ext/midi#chunk";
    pub const CONTROLLERNUMBER: &'static str = "http://lv2plug.in/ns/ext/midi#controllerNumber";
    pub const CONTROLLERVALUE:  &'static str = "http://lv2plug.in/ns/ext/midi#controllerValue";
    pub const NOTENUMBER:       &'static str = "http://lv2plug.in/ns/ext/midi#noteNumber";
    pub const PRESSURE:         &'static str = "http://lv2plug.in/ns/ext/midi#pressure";
    pub const PROGRAMNUMBER:    &'static str = "http://lv2plug.in/ns/ext/midi#programNumber";
    pub const PROPERTY:         &'static str = "http://lv2plug.in/ns/ext/midi#property";
    pub const SONGNUMBER:       &'static str = "http://lv2plug.in/ns/ext/midi#songNumber";
    pub const SONGPOSITION:     &'static str = "http://lv2plug.in/ns/ext/midi#songPosition";
    pub const STATUS:           &'static str = "http://lv2plug.in/ns/ext/midi#status";
    pub const STATUSMASK:       &'static str = "http://lv2plug.in/ns/ext/midi#statusMask";
    pub const VELOCITY:         &'static str = "http://lv2plug.in/ns/ext/midi#velocity";
}

pub type Message = u8;
pub const MSG_INVALID          : Message = 0x00;
pub const MSG_NOTE_OFF         : Message = 0x80;
pub const MSG_NOTE_ON          : Message = 0x90;
pub const MSG_NOTE_PRESSURE    : Message = 0xA0;
pub const MSG_CONTROLLER       : Message = 0xB0;
pub const MSG_PGM_CHANGE       : Message = 0xC0;
pub const MSG_CHANNEL_PRESSURE : Message = 0xD0;
pub const MSG_BENDER           : Message = 0xE0;
pub const MSG_SYSTEM_EXCLUSIVE : Message = 0xF0;
pub const MSG_MTC_QUARTER      : Message = 0xF1;
pub const MSG_SONG_POS         : Message = 0xF2;
pub const MSG_SONG_SELECT      : Message = 0xF3;
pub const MSG_TUNE_REQUEST     : Message = 0xF6;
pub const MSG_CLOCK            : Message = 0xF8;
pub const MSG_START            : Message = 0xFA;
pub const MSG_CONTINUE         : Message = 0xFB;
pub const MSG_STOP             : Message = 0xFC;
pub const MSG_ACTIVE_SENSE     : Message = 0xFE;
pub const MSG_RESET            : Message = 0xFF;

pub type Controller = u8;
pub const CTL_MSB_BANK             : Controller = 0x00;
pub const CTL_MSB_MODWHEEL         : Controller = 0x01;
pub const CTL_MSB_BREATH           : Controller = 0x02;
pub const CTL_MSB_FOOT             : Controller = 0x04;
pub const CTL_MSB_PORTAMENTO_TIME  : Controller = 0x05;
pub const CTL_MSB_DATA_ENTRY       : Controller = 0x06;
pub const CTL_MSB_MAIN_VOLUME      : Controller = 0x07;
pub const CTL_MSB_BALANCE          : Controller = 0x08;
pub const CTL_MSB_PAN              : Controller = 0x0A;
pub const CTL_MSB_EXPRESSION       : Controller = 0x0B;
pub const CTL_MSB_EFFECT1          : Controller = 0x0C;
pub const CTL_MSB_EFFECT2          : Controller = 0x0D;
pub const CTL_MSB_GENERAL_PURPOSE1 : Controller = 0x10;
pub const CTL_MSB_GENERAL_PURPOSE2 : Controller = 0x11;
pub const CTL_MSB_GENERAL_PURPOSE3 : Controller = 0x12;
pub const CTL_MSB_GENERAL_PURPOSE4 : Controller = 0x13;
pub const CTL_LSB_BANK             : Controller = 0x20;
pub const CTL_LSB_MODWHEEL         : Controller = 0x21;
pub const CTL_LSB_BREATH           : Controller = 0x22;
pub const CTL_LSB_FOOT             : Controller = 0x24;
pub const CTL_LSB_PORTAMENTO_TIME  : Controller = 0x25;
pub const CTL_LSB_DATA_ENTRY       : Controller = 0x26;
pub const CTL_LSB_MAIN_VOLUME      : Controller = 0x27;
pub const CTL_LSB_BALANCE          : Controller = 0x28;
pub const CTL_LSB_PAN              : Controller = 0x2A;
pub const CTL_LSB_EXPRESSION       : Controller = 0x2B;
pub const CTL_LSB_EFFECT1          : Controller = 0x2C;
pub const CTL_LSB_EFFECT2          : Controller = 0x2D;
pub const CTL_LSB_GENERAL_PURPOSE1 : Controller = 0x30;
pub const CTL_LSB_GENERAL_PURPOSE2 : Controller = 0x31;
pub const CTL_LSB_GENERAL_PURPOSE3 : Controller = 0x32;
pub const CTL_LSB_GENERAL_PURPOSE4 : Controller = 0x33;
pub const CTL_SUSTAIN              : Controller = 0x40;
pub const CTL_PORTAMENTO           : Controller = 0x41;
pub const CTL_SOSTENUTO            : Controller = 0x42;
pub const CTL_SOFT_PEDAL           : Controller = 0x43;
pub const CTL_LEGATO_FOOTSWITCH    : Controller = 0x44;
pub const CTL_HOLD2                : Controller = 0x45;
pub const CTL_SC1_SOUND_VARIATION  : Controller = 0x46;
pub const CTL_SC2_TIMBRE           : Controller = 0x47;
pub const CTL_SC3_RELEASE_TIME     : Controller = 0x48;
pub const CTL_SC4_ATTACK_TIME      : Controller = 0x49;
pub const CTL_SC5_BRIGHTNESS       : Controller = 0x4A;
pub const CTL_SC6                  : Controller = 0x4B;
pub const CTL_SC7                  : Controller = 0x4C;
pub const CTL_SC8                  : Controller = 0x4D;
pub const CTL_SC9                  : Controller = 0x4E;
pub const CTL_SC10                 : Controller = 0x4F;
pub const CTL_GENERAL_PURPOSE5     : Controller = 0x50;
pub const CTL_GENERAL_PURPOSE6     : Controller = 0x51;
pub const CTL_GENERAL_PURPOSE7     : Controller = 0x52;
pub const CTL_GENERAL_PURPOSE8     : Controller = 0x53;
pub const CTL_PORTAMENTO_CONTROL   : Controller = 0x54;
pub const CTL_E1_REVERB_DEPTH      : Controller = 0x5B;
pub const CTL_E2_TREMOLO_DEPTH     : Controller = 0x5C;
pub const CTL_E3_CHORUS_DEPTH      : Controller = 0x5D;
pub const CTL_E4_DETUNE_DEPTH      : Controller = 0x5E;
pub const CTL_E5_PHASER_DEPTH      : Controller = 0x5F;
pub const CTL_DATA_INCREMENT       : Controller = 0x60;
pub const CTL_DATA_DECREMENT       : Controller = 0x61;
pub const CTL_NRPN_LSB             : Controller = 0x62;
pub const CTL_NRPN_MSB             : Controller = 0x63;
pub const CTL_RPN_LSB              : Controller = 0x64;
pub const CTL_RPN_MSB              : Controller = 0x65;
pub const CTL_ALL_SOUNDS_OFF       : Controller = 0x78;
pub const CTL_RESET_CONTROLLERS    : Controller = 0x79;
pub const CTL_LOCAL_CONTROL_SWITCH : Controller = 0x7A;
pub const CTL_ALL_NOTES_OFF        : Controller = 0x7B;
pub const CTL_OMNI_OFF             : Controller = 0x7C;
pub const CTL_OMNI_ON              : Controller = 0x7D;
pub const CTL_MONO1                : Controller = 0x7E;
pub const CTL_MONO2                : Controller = 0x7F;

pub fn is_voice_message(msg: u8) -> bool {
    msg >= 0x80 && msg < 0xF0
}

pub fn is_system_message(msg: u8) -> bool {
    match msg {
        0xF4 | 0xF5 | 0xF7 | 0xF9 | 0xFD => false,
        _ => { msg & 0xF0 == 0xF0 }
    }
}

pub fn message_type(msg: u8) -> Message {
    if is_voice_message(msg) {
        msg & 0xF0
    }
    else if is_system_message(msg) {
        msg
    }
    else {
        MSG_INVALID
    }
}

