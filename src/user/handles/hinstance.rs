#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::{GetLastError, HINSTANCE, IdStr, SysResult, WString};
use crate::kernel::privs::ptr_to_sysresult;
use crate::prelude::Handle;
use crate::user;
use crate::user::decl::{
	ATOM, DLGPROC, HACCEL, HCURSOR, HICON, HMENU, HWND, IdIdcStr, IdIdiStr,
	WNDCLASSEX, DLGTEMPLATEEX,
};
use crate::user::guard::{DestroyCursorGuard, DestroyIconGuard};

impl user_Hinstance for HINSTANCE {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HINSTANCE`](crate::HINSTANCE).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hinstance: Handle {
	/// [`CreateDialogParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdialogparamw)
	/// method.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	unsafe fn CreateDialogParam(&self,
		resource_id: IdStr,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>) -> SysResult<HWND>
	{
		ptr_to_sysresult(
			unsafe {
				user::ffi::CreateDialogParamW(
					self.as_ptr(),
					resource_id.as_ptr(),
					hwnd_parent.map_or(std::ptr::null_mut(), |h| h.0),
					dialog_proc as _,
					init_param.unwrap_or_default(),
				)
			},
			|ptr| HWND(ptr),
		)
	}

	unsafe fn DialogBoxIndirectParam(&self,
		template: *const DLGTEMPLATEEX,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>) -> SysResult<isize>
	{
		match unsafe {
			user::ffi::DialogBoxIndirectParamW(
				self.as_ptr(),
				template as _,
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.0),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res),
		}
	}

	/// [`DialogBoxParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparamw)
	/// method.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	unsafe fn DialogBoxParam(&self,
		resource_id: IdStr,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>) -> SysResult<isize>
	{
		match unsafe {
			user::ffi::DialogBoxParamW(
				self.as_ptr(),
				resource_id.as_ptr(),
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.0),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res), // assumes hWndParent as valid, so no check for zero
		}
	}

	/// [`GetClassInfoEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassinfoexw)
	/// method.
	///
	/// # Examples
	///
	/// Retrieving information of a window class created in our application:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HINSTANCE, WNDCLASSEX};
	///
	/// let mut wcx = WNDCLASSEX::default();
	/// HINSTANCE::GetModuleHandle(None)?
	///     .GetClassInfoEx("SOME_CLASS_NAME", &mut wcx)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetClassInfoEx(&self,
		class_name: &str, wcx: &mut WNDCLASSEX) -> SysResult<ATOM>
	{
		match unsafe {
			user::ffi::GetClassInfoExW(
				self.as_ptr(),
				WString::from_str(class_name).as_ptr(),
				wcx as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			atom => Ok(ATOM(atom as _)),
		}
	}

	/// [`LoadAccelerators`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadacceleratorsw)
	/// method.
	#[must_use]
	fn LoadAccelerators(&self, table_name: IdStr) -> SysResult<HACCEL> {
		ptr_to_sysresult(
			unsafe {
				user::ffi::LoadAcceleratorsW(self.as_ptr(), table_name.as_ptr())
			},
			|ptr| HACCEL(ptr),
		)
	}

	/// [`LoadCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	/// method.
	///
	/// # Examples
	///
	/// Loading a system cursor:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HINSTANCE, IdIdcStr};
	///
	/// let sys_cursor = HINSTANCE::NULL
	///     .LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LoadCursor(&self, resource_id: IdIdcStr) -> SysResult<DestroyCursorGuard> {
		ptr_to_sysresult(
			unsafe { user::ffi::LoadCursorW(self.as_ptr(), resource_id.as_ptr()) },
			|ptr| DestroyCursorGuard::new(HCURSOR(ptr)),
		)
	}

	/// [`LoadIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)
	/// method.
	///
	/// # Examples
	///
	/// Loading a system icon:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IdIdiStr, HINSTANCE};
	///
	/// let sys_icon = HINSTANCE::NULL
	///     .LoadIcon(IdIdiStr::Idi(co::IDI::INFORMATION))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LoadIcon(&self, icon_id: IdIdiStr) -> SysResult<DestroyIconGuard> {
		ptr_to_sysresult(
			unsafe { user::ffi::LoadIconW(self.as_ptr(), icon_id.as_ptr()) },
			|ptr| DestroyIconGuard::new(HICON(ptr)),
		)
	}

	/// [`LoadMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadmenuw)
	/// method.
	#[must_use]
	fn LoadMenu(&self, resource_id: IdStr) -> SysResult<HMENU> {
		ptr_to_sysresult(
			unsafe { user::ffi::LoadMenuW(self.as_ptr(), resource_id.as_ptr()) },
			|ptr| HMENU(ptr),
		)
	}

	/// [`LoadString`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadstringw)
	/// method.
	#[must_use]
	fn LoadString(&self, id: u16) -> SysResult<String> {
		let mut pdata: *const u16 = std::ptr::null_mut();
		match unsafe {
			user::ffi::LoadStringW(
				self.as_ptr(),
				id as _,
				&mut pdata as *mut _ as  _, 0,
			)
		} {
			0 => Err(GetLastError()),
			len => Ok(WString::from_wchars_count(pdata, len as _).to_string())
		}
	}
}
