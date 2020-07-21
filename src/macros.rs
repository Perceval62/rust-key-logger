#![allow(dead_code)]
pub const KEY_RESERVED: u8 =	0;
pub const KEY_ESC: u8 =			1;
pub const KEY_1: u8 =			2;
pub const KEY_2: u8 =			3;
pub const KEY_3: u8 =			4;
pub const KEY_4: u8 =			5;
pub const KEY_5: u8 =			6;
pub const KEY_6: u8 =			7;
pub const KEY_7: u8 =			8;
pub const KEY_8: u8 =			9;
pub const KEY_9: u8 =			10;
pub const KEY_0: u8 =			11;
pub const KEY_MINUS: u8 =		12;
pub const KEY_EQUAL: u8 =		13;
pub const KEY_BACKSPACE: u8 =		14;
pub const KEY_TAB: u8 =			15;
pub const KEY_Q: u8 =			16;
pub const KEY_W: u8 =			17;
pub const KEY_E: u8 =			18;
pub const KEY_R: u8 =			19;
pub const KEY_T: u8 =			20;
pub const KEY_Y: u8 =			21;
pub const KEY_U: u8 =			22;
pub const KEY_I: u8 =			23;
pub const KEY_O: u8 =			24;
pub const KEY_P: u8 =			25;
pub const KEY_LEFTBRACE: u8 =		26;
pub const KEY_RIGHTBRACE: u8 =		27;
pub const KEY_ENTER: u8 =		28;
pub const KEY_LEFTCTRL: u8 =		29;
pub const KEY_A: u8 =			30;
pub const KEY_S: u8 =			31;
pub const KEY_D: u8 =			32;
pub const KEY_F: u8 =			33;
pub const KEY_G: u8 =			34;
pub const KEY_H: u8 =			35;
pub const KEY_J: u8 =			36;
pub const KEY_K: u8 =			37;
pub const KEY_L: u8 =			38;
pub const KEY_SEMICOLON: u8 =		39;
pub const KEY_APOSTROPHE: u8 =		40;
pub const KEY_GRAVE: u8 =		41;
pub const KEY_LEFTSHIFT: u8 =		42;
pub const KEY_BACKSLASH: u8 =		43;
pub const KEY_Z: u8 =			44;
pub const KEY_X: u8 =			45;
pub const KEY_C: u8 =			46;
pub const KEY_V: u8 =			47;
pub const KEY_B: u8 =			48;
pub const KEY_N: u8 =			49;
pub const KEY_M: u8 =			50;
pub const KEY_COMMA: u8 =		51;
pub const KEY_DOT: u8 =			52;
pub const KEY_SLASH: u8 =		53;
pub const KEY_RIGHTSHIFT: u8 =		54;
pub const KEY_KPASTERISK: u8 =		55;
pub const KEY_LEFTALT: u8 =		56;
pub const KEY_SPACE: u8 =		57;
pub const KEY_CAPSLOCK: u8 =		58;
pub const KEY_F1: u8 =			59;
pub const KEY_F2: u8 =			60;
pub const KEY_F3: u8 =			61;
pub const KEY_F4: u8 =			62;
pub const KEY_F5: u8 =			63;
pub const KEY_F6: u8 =			64;
pub const KEY_F7: u8 =			65;
pub const KEY_F8: u8 =			66;
pub const KEY_F9: u8 =			67;
pub const KEY_F10: u8 =			68;
pub const KEY_NUMLOCK: u8 =		69;
pub const KEY_SCROLLLOCK: u8 =		70;
pub const KEY_KP7: u8 =			71;
pub const KEY_KP8: u8 =			72;
pub const KEY_KP9: u8 =			73;
pub const KEY_KPMINUS: u8 =		74;
pub const KEY_KP4: u8 =			75;
pub const KEY_KP5: u8 =			76;
pub const KEY_KP6: u8 =			77;
pub const KEY_KPPLUS: u8 =		78;
pub const KEY_KP1: u8 =			79;
pub const KEY_KP2: u8 =			80;
pub const KEY_KP3: u8 =			81;
pub const KEY_KP0: u8 =			82;
pub const KEY_KPDOT: u8 =		83;
pub const KEY_ZENKAKUHANKAKU: u8 =	85;
pub const KEY_102ND: u8 =		86;
pub const KEY_F11: u8 =			87;
pub const KEY_F12: u8 =			88;
pub const KEY_RO: u8 =			89;
pub const KEY_KATAKANA: u8 =		90;
pub const KEY_HIRAGANA: u8 =		91;
pub const KEY_HENKAN: u8 =		92;
pub const KEY_KATAKANAHIRAGANA: u8 =	93;
pub const KEY_MUHENKAN: u8 =		94;
pub const KEY_KPJPCOMMA: u8 =		95;
pub const KEY_KPENTER: u8 =		96;
pub const KEY_RIGHTCTRL: u8 =		97;
pub const KEY_KPSLASH: u8 =		98;
pub const KEY_SYSRQ: u8 =		99;
pub const KEY_RIGHTALT: u8 =		100;
pub const KEY_LINEFEED: u8 =		101;
pub const KEY_HOME: u8 =		102;
pub const KEY_UP: u8 =			103;
pub const KEY_PAGEUP: u8 =		104;
pub const KEY_LEFT: u8 =		105;
pub const KEY_RIGHT: u8 =		106;
pub const KEY_END: u8 =			107;
pub const KEY_DOWN: u8 =		108;
pub const KEY_PAGEDOWN: u8 =		109;
pub const KEY_INSERT: u8 =		110;
pub const KEY_DELETE: u8 =		111;
pub const KEY_MACRO: u8 =		112;
pub const KEY_MUTE: u8 =		113;
pub const KEY_VOLUMEDOWN: u8 =		114;
pub const KEY_VOLUMEUP: u8 =		115;
pub const KEY_POWER: u8 =		116	/* SC System Power Down */;
pub const KEY_KPEQUAL: u8 =		117;
pub const KEY_KPPLUSMINUS: u8 =		118;
pub const KEY_PAUSE: u8 =		119;
pub const KEY_SCALE: u8 =		120	/* AL Compiz Scale (Expose) */;
pub const KEY_KPCOMMA: u8 =		121;
pub const KEY_HANGEUL: u8 =		122;
pub const KEY_HANGUEL: u8 =		KEY_HANGEUL;
pub const KEY_HANJA: u8 =		123;
pub const KEY_YEN: u8 =			124;
pub const KEY_LEFTMETA: u8 =		125;
pub const KEY_RIGHTMETA: u8 =		126;
pub const KEY_COMPOSE: u8 =		127;
pub const KEY_STOP: u8 =		128	/* AC Stop */;
pub const KEY_AGAIN: u8 =		129;
pub const KEY_PROPS: u8 =		130	/* AC Properties */;
pub const KEY_UNDO: u8 =		131	/* AC Undo */;
pub const KEY_FRONT: u8 =		132;
pub const KEY_COPY: u8 =		133	/* AC Copy */;
pub const KEY_OPEN: u8 =		134	/* AC Open */;
pub const KEY_PASTE: u8 =		135	/* AC Paste */;
pub const KEY_FIND: u8 =		136	/* AC Search */;
pub const KEY_CUT: u8 =			137	/* AC Cut */;
pub const KEY_HELP: u8 =		138	/* AL Integrated Help Center */;
pub const KEY_MENU: u8 =		139	/* Menu (show menu) */;
pub const KEY_CALC: u8 =		140	/* AL Calculator */;
pub const KEY_SETUP: u8 =		141;
pub const KEY_SLEEP: u8 =	142	/* SC System Sleep */;
pub const KEY_WAKEUP: u8 =		143	/* System Wake Up */;
pub const KEY_FILE: u8 =		144	/* AL Local Machine Browser */;
pub const KEY_SENDFILE: u8 =		145;
pub const KEY_DELETEFILE: u8 =		146;
pub const KEY_XFER: u8 =		147;
pub const KEY_PROG1: u8 =		148;
pub const KEY_PROG2: u8 =		149;
pub const KEY_WWW: u8 =			150	/* AL Internet Browser */;
pub const KEY_MSDOS: u8 =		151;
pub const KEY_COFFEE: u8 =		152	/* AL Terminal Lock/Screensaver */;
pub const KEY_SCREENLOCK: u8 =		KEY_COFFEE;
pub const KEY_ROTATE_DISPLAY: u8 =	153	/* Display orientation for e.g. tablets */;
pub const KEY_DIRECTION: u8 =		KEY_ROTATE_DISPLAY;
pub const KEY_CYCLEWINDOWS: u8 =	154;
pub const KEY_MAIL: u8 =		155;
pub const KEY_BOOKMARKS: u8 =		156	/* AC Bookmarks */;
pub const KEY_COMPUTER: u8 =		157;
pub const KEY_BACK: u8 =		158	/* AC Back */;
pub const KEY_FORWARD: u8 =		159	/* AC Forward */;
pub const KEY_CLOSECD: u8 =		160;
pub const KEY_EJECTCD: u8 =		161;
pub const KEY_EJECTCLOSECD: u8 =	162;
pub const KEY_NEXTSONG: u8 =		163;
pub const KEY_PLAYPAUSE: u8 =		164;
pub const KEY_PREVIOUSSONG: u8 =	165;
pub const KEY_STOPCD: u8 =		166;
pub const KEY_RECORD: u8 =		167;
pub const KEY_REWIND: u8 =		168;
pub const KEY_PHONE: u8 =		169	/* Media Select Telephone */;
pub const KEY_ISO: u8 =			170;
pub const KEY_CONFIG: u8 =		171	/* AL Consumer Control Configuration */;
pub const KEY_HOMEPAGE: u8 =		172	/* AC Home */;
pub const KEY_REFRESH	: u8 =	173	/* AC Refresh */;
pub const KEY_EXIT		: u8 =174	/* AC Exit */;
pub const KEY_MOVE		: u8 =175;
pub const KEY_EDIT		: u8 =176;
pub const KEY_SCROLLUP	: u8 =	177;
pub const KEY_SCROLLDOWN	: u8 =	178;
pub const KEY_KPLEFTPAREN	: u8 =	179;
pub const KEY_KPRIGHTPAREN	: u8 =180;
pub const KEY_NEW		: u8 =181	/* AC New */;
pub const KEY_REDO		: u8 =182	/* AC Redo/Repeat */;
pub const KEY_F13		: u8 =	183;
pub const KEY_F14		: u8 =	184;
pub const KEY_F15		: u8 =	185;
pub const KEY_F16		: u8 =	186;
pub const KEY_F17		: u8 =	187;
pub const KEY_F18		: u8 =	188;
pub const KEY_F19		: u8 =	189;
pub const KEY_F20		: u8 =	190;
pub const KEY_F21		: u8 =	191;
pub const KEY_F22		: u8 =	192;
pub const KEY_F23		: u8 =	193;
pub const KEY_F24		: u8 =	194;
pub const KEY_PLAYCD	: u8 =	200;
pub const KEY_PAUSECD	: u8 =	201;
pub const KEY_PROG3		: u8 =  202;
pub const KEY_PROG4		: u8 =  203;
pub const KEY_DASHBOARD	: u8 =  	204	/* AL Dashboard */;
pub const KEY_SUSPEND	: u8 =  	205;
pub const KEY_CLOSE		: u8 =  206	/* AC Close */;
pub const KEY_PLAY		: u8 =  207;
pub const KEY_FASTFORWARD : u8 =		208;
pub const KEY_BASSBOOST : u8 =		209;
pub const KEY_PRINT		: u8 =210	/* AC Print */;
pub const KEY_HP		: u8 =	211;
pub const KEY_CAMERA	: u8 =	212;
pub const KEY_SOUND		: u8 =213;
pub const KEY_QUESTION	: u8 =	214;
pub const KEY_EMAIL		: u8 =215;
pub const KEY_CHAT		: u8 =216;
pub const KEY_SEARCH	: u8 =	217;
pub const KEY_CONNECT	: u8 =	218;
pub const KEY_FINANCE	: u8 =	219	/* AL Checkbook/Finance */;
pub const KEY_SPORT		: u8 =  220;
pub const KEY_SHOP		: u8 =  221;
pub const KEY_ALTERASE	: u8 =  	222;
pub const KEY_CANCEL	: u8 =  	223	/* AC Cancel */;
pub const KEY_BRIGHTNESSDOWN : u8 =	224;
pub const KEY_BRIGHTNESSUP	: u8 =225;
pub const KEY_MEDIA		: u8 =226;
pub const KEY_SWITCHVIDEOMODE	: u8 =227	/* Cycle between available video;
pub 					   outputs (Monitor/LCD/TV-out/etc) */;
pub const KEY_KBDILLUMTOGGLE: u8 =	228;
pub const KEY_KBDILLUMDOWN: u8 =	229;
pub const KEY_KBDILLUMUP: u8 =		230;
pub const KEY_SEND		: u8 =231	/* AC Send */;
pub const KEY_REPLY		: u8 =232	/* AC Reply */;
pub const KEY_FORWARDMAIL		: u8 =233	/* AC Forward Msg */;
pub const KEY_SAVE		: u8 =234	/* AC Save */;
pub const KEY_DOCUMENTS		: u8 =235;
pub const KEY_BATTERY		: u8 =236;
pub const KEY_BLUETOOTH		: u8 =237;
pub const KEY_WLAN		: u8 =238;
pub const KEY_UWB		: u8 =	239;
pub const KEY_UNKNOWN	: u8 =	240;
pub const KEY_VIDEO_NEXT		: u8 =241	/* drive next video source */;
pub const KEY_VIDEO_PREV		: u8 =242	/* drive previous video source */;
pub const KEY_BRIGHTNESS_CYCLE	: u8 =243	/* brightness up, after max is min */;
pub const KEY_BRIGHTNESS_AUTO	: u8 =244	/* Set Auto Brightness: manual;
pub 					  brightness control is off,;
pub 					  rely on ambient */;
pub const KEY_BRIGHTNESS_ZERO	: u8 = KEY_BRIGHTNESS_AUTO;
pub const KEY_DISPLAY_OFF		: u8 =245	/* display device to off state */;
pub const KEY_WWAN		: u8 =246	/* Wireless WAN (LTE, UMTS, GSM, etc.) */;
pub const KEY_WIMAX		: u8 =KEY_WWAN;
pub const KEY_RFKILL	: u8 =	247	/* Key that controls all radios */;
pub const KEY_MICMUTE	: u8 =	248	/* Mute / unmute the microphone */;