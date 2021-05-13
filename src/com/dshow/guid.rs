//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM assorted GUIDs.

use crate::structs::GUID;

pub_const_guid! { GUID,
	MR_VIDEO_RENDER_SERVICE, 0x1092a86c, 0xab1a, 0x459a, 0xa336, 0x831fbc4d11ff,
	MR_VIDEO_MIXER_SERVICE, 0x073cd2fc, 0x6cf4, 0x40b7, 0x8859, 0xe89552c841f8,
	MR_VIDEO_ACCELERATION_SERVICE, 0xefef5175, 0x5c7d, 0x4ce2, 0xbbbd, 0x34ff8bca6554,
	MR_BUFFER_SERVICE, 0xa562248c, 0x9ac6, 0x4ffc, 0x9fba, 0x3af8f8ad1a4d,
	VIDEO_ZOOM_RECT, 0x7aaa1638, 0x1b7f, 0x4c93, 0xbd89, 0x5b9c9fb6fcf0,

	TIME_FORMAT_NONE, 0x00000000, 0x0000, 0x0000, 0x0000, 0x000000000000,
	TIME_FORMAT_FRAME, 0x7b785570, 0x8c82, 0x11cf, 0xbc0c, 0x00aa00ac74f6,
	TIME_FORMAT_BYTE, 0x7b785571, 0x8c82, 0x11cf, 0xbc0c, 0x00aa00ac74f6,
	TIME_FORMAT_SAMPLE, 0x7b785572, 0x8c82, 0x11cf, 0xbc0c, 0x00aa00ac74f6,
	TIME_FORMAT_FIELD, 0x7b785573, 0x8c82, 0x11cf, 0xbc0c, 0x00aa00ac74f6,
	TIME_FORMAT_MEDIA_TIME, 0x7b785574, 0x8c82, 0x11cf, 0xbc0c, 0x00aa00ac74f6,
}
