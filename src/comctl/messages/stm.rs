use crate::co;
use crate::comctl::decl::BmpIconCurMeta;
use crate::kernel::decl::SysResult;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{HBITMAP, HCURSOR, HDC, HICON};
use crate::user::privs::zero_as_err;

/// [`STM_GETICON`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-geticon)
/// message, which has no parameters.
///
/// Return type: `SysResult<HICON>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetIcon {}

unsafe impl MsgSend for GetIcon {
	type RetType = SysResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HICON(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::GETICON.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`STM_GETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-getimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIconCurMeta>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetImage {
	pub img_type: co::IMAGE_TYPE,
}

unsafe impl MsgSend for GetImage {
	type RetType = SysResult<BmpIconCurMeta>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match self.img_type {
			co::IMAGE_TYPE::BITMAP => Ok(BmpIconCurMeta::Bmp(HBITMAP(v as _))),
			co::IMAGE_TYPE::ICON => Ok(BmpIconCurMeta::Icon(HICON(v as _))),
			co::IMAGE_TYPE::CURSOR => Ok(BmpIconCurMeta::Cur(HCURSOR(v as _))),
			co::IMAGE_TYPE::ENHMETAFILE => Ok(BmpIconCurMeta::Meta(HDC(v as _))),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::GETIMAGE.into(),
			wparam: self.img_type.0 as _,
			lparam: 0,
		}
	}
}

/// [`STM_SETICON`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-seticon)
/// message parameters.
///
/// Return type: `SysResult<HICON>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetIcon {
	pub icon: HICON,
}

unsafe impl MsgSend for SetIcon {
	type RetType = SysResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HICON(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::SETICON.into(),
			wparam: self.icon.0 as _,
			lparam: 0,
		}
	}
}

/// [`STM_SETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-setimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIconCurMeta>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetImage {
	pub image: BmpIconCurMeta,
}

unsafe impl MsgSend for SetImage {
	type RetType = SysResult<BmpIconCurMeta>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		if v == 0 {
			Err(co::ERROR::BAD_ARGUMENTS)
		} else {
			match self.image {
				BmpIconCurMeta::Bmp(_) => Ok(BmpIconCurMeta::Bmp(HBITMAP(v as _))),
				BmpIconCurMeta::Icon(_) => Ok(BmpIconCurMeta::Icon(HICON(v as _))),
				BmpIconCurMeta::Cur(_) => Ok(BmpIconCurMeta::Cur(HCURSOR(v as _))),
				BmpIconCurMeta::Meta(_) => Ok(BmpIconCurMeta::Meta(HDC(v as _))),
			}
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::SETIMAGE.into(),
			wparam: match self.image {
				BmpIconCurMeta::Bmp(_) => co::IMAGE_TYPE::BITMAP.0,
				BmpIconCurMeta::Icon(_) => co::IMAGE_TYPE::ICON.0,
				BmpIconCurMeta::Cur(_) => co::IMAGE_TYPE::CURSOR.0,
				BmpIconCurMeta::Meta(_) => co::IMAGE_TYPE::ENHMETAFILE.0,
			} as _,
			lparam: self.image.as_isize(),
		}
	}
}
