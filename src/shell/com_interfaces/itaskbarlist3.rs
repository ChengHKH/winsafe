#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HANDLE, HRES, PCSTR, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, shell_ITaskbarList, shell_ITaskbarList2};
use crate::user::decl::{HICON, HWND, RECT};
use crate::vt::ITaskbarList2VT;

/// [`ITaskbarList3`](crate::ITaskbarList3) virtual table.
#[repr(C)]
pub struct ITaskbarList3VT {
	pub ITaskbarList2VT: ITaskbarList2VT,
	pub SetProgressValue: fn(ComPtr, HANDLE, u64, u64) -> HRES,
	pub SetProgressState: fn(ComPtr, HANDLE, u32) -> HRES,
	pub RegisterTab: fn(ComPtr, HANDLE, HANDLE) -> HRES,
	pub UnregisterTab: fn(ComPtr, HANDLE) -> HRES,
	pub SetTabOrder: fn(ComPtr, HANDLE, HANDLE) -> HRES,
	pub SetTabActive: fn(ComPtr, HANDLE, HANDLE, u32) -> HRES,
	pub ThumbBarAddButtons: fn(ComPtr, HANDLE, u32, PVOID) -> HRES,
	pub ThumbBarUpdateButtons: fn(ComPtr, HANDLE, u32, PVOID) -> HRES,
	pub ThumbBarSetImageList: fn(ComPtr, HANDLE, HANDLE) -> HRES,
	pub SetOverlayIcon: fn(ComPtr, HANDLE, HANDLE, PCSTR) -> HRES,
	pub SetThumbnailTooltip: fn(ComPtr, HANDLE, PCSTR) -> HRES,
	pub SetThumbnailClip: fn(ComPtr, HANDLE, PVOID) -> HRES,
}

com_interface! { ITaskbarList3: "ea1afb91-9e28-4b86-90e9-9e9f8a5eefaf";
	/// [`ITaskbarList3`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
	/// COM interface over [`ITaskbarList3VT`](crate::vt::ITaskbarList3VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, ITaskbarList3};
	///
	/// let obj = CoCreateInstance::<ITaskbarList3>(
	///     &co::CLSID::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_ITaskbarList for ITaskbarList3 {}
impl shell_ITaskbarList2 for ITaskbarList3 {}
impl shell_ITaskbarList3 for ITaskbarList3 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList3`](crate::ITaskbarList3).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_ITaskbarList3: shell_ITaskbarList2 {
	/// [`ITaskbarList3::RegisterTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	fn RegisterTab(&self, hwnd_tab: &HWND, hwnd_mdi: &HWND) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.RegisterTab)(self.ptr(), hwnd_tab.as_ptr(), hwnd_mdi.as_ptr()),
			)
		}
	}

	/// [`ITaskbarList3::SetOverlayIcon`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setoverlayicon)
	/// method.
	fn SetOverlayIcon(&self,
		hwnd: &HWND, hicon: Option<&HICON>, description: &str) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.SetOverlayIcon)(
					self.ptr(),
					hwnd.as_ptr(),
					hicon.map_or(std::ptr::null_mut(), |h| h.as_ptr()),
					WString::from_str(description).as_ptr(),
				),
			)
		}
	}

	/// [`ITaskbarList3::SetProgressState`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	fn SetProgressState(&self,
		hwnd: &HWND, tbpf_flags: co::TBPF) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.SetProgressState)(self.ptr(), hwnd.as_ptr(), tbpf_flags.0),
			)
		}
	}

	/// [`ITaskbarList3::SetProgressValue`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	///
	/// # Examples
	///
	/// Setting progress to 50%:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, ITaskbarList3};
	///
	/// let tbar: ITaskbarList3; // initialized somewhere
	/// # let tbar = ITaskbarList3::from(unsafe { winsafe::ComPtr::null() });
	/// let hwnd: HWND;
	/// # let hwnd = HWND::NULL;
	///
	/// tbar.SetProgressValue(&hwnd, 50, 100)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	fn SetProgressValue(&self,
		hwnd: &HWND, completed: u64, total: u64) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.SetProgressValue)(self.ptr(), hwnd.as_ptr(), completed, total),
			)
		}
	}

	/// [`ITaskbarList3::SetTabActive`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	fn SetTabActive(&self, hwnd_tab: &HWND, hwnd_mdi: &HWND) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.SetTabActive)(
					self.ptr(),
					hwnd_tab.as_ptr(),
					hwnd_mdi.as_ptr(),
					0,
				),
			)
		}
	}

	/// [`ITaskbarList3::SetTabOrder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	fn SetTabOrder(&self,
		hwnd_tab: &HWND, hwnd_insert_before: &HWND) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.SetTabOrder)(
					self.ptr(),
					hwnd_tab.as_ptr(),
					hwnd_insert_before.as_ptr(),
				),
			)
		}
	}

	/// [`ITaskbarList3::SetThumbnailClip`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setthumbnailclip)
	/// method.
	fn SetThumbnailClip(&self, hwnd: &HWND, clip: Option<RECT>) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.SetThumbnailClip)(
					self.ptr(),
					hwnd.as_ptr(),
					&clip as *const _ as _,
				),
			)
		}
	}

	/// [`ITaskbarList3::SetThumbnailTooltip`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setthumbnailtooltip)
	/// method.
	fn SetThumbnailTooltip(&self,
		hwnd: &HWND, tip: Option<&str>) -> HrResult<()>
	{
		unsafe {
			let vt = self.vt_ref::<ITaskbarList3VT>();
			ok_to_hrresult(
				(vt.SetThumbnailTooltip)(
					self.ptr(),
					hwnd.as_ptr(),
					tip.map_or(std::ptr::null_mut(), |s| WString::from_str(s).as_ptr()),
				),
			)
		}
	}
}
