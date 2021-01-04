#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::aliases::{SUBCLASSPROC, WNDENUMPROC};
use crate::co;
use crate::enums::{AtomStr, HwndPlace, IdMenu, IdPos};
use crate::ffi::{comctl32, user32};
use crate::funcs::{GetLastError, SetLastError};
use crate::handles::{HACCEL, HDC, HINSTANCE, HMENU, HRGN};
use crate::msg::Wm;
use crate::priv_funcs::{const_void, mut_void, ptr_as_opt};
use crate::structs::{MSG, PAINTSTRUCT, RECT, WINDOWINFO, WINDOWPLACEMENT};
use crate::WString;

handle_type! {
	/// Handle to a
	/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
	/// Exposes methods.
	HWND
}

impl HWND {
	/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	/// method.
	///
	/// Must be paired with an [`EndPaint`](crate::HWND::EndPaint) call.
	pub fn BeginPaint(self, lpPaint: &mut PAINTSTRUCT) -> Result<HDC, ()> {
		match ptr_as_opt(
			unsafe { user32::BeginPaint(self.0, mut_void(lpPaint)) }
		 ) {
			Some(p) => Ok(unsafe { HDC::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`CreateWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// static method.
	pub fn CreateWindowEx(
		dwExStyle: co::WS_EX,
		lpClassName: AtomStr,
		lpWindowName: Option<&str>,
		dwStyle: co::WS,
		X: i32, Y: i32,
		nWidth: i32, nHeight: i32,
		hWndParent: Option<HWND>,
		hMenu: IdMenu,
		hInstance: HINSTANCE,
		lpParam: Option<isize>) -> Result<HWND, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::CreateWindowExW(
					dwExStyle.into(),
					lpClassName.as_ptr(),
					WString::from_opt_str(lpWindowName).as_ptr(),
					dwStyle.into(),
					X, Y, nWidth, nHeight,
					match hWndParent {
						Some(hParent) => hParent.0,
						None => std::ptr::null_mut(),
					},
					hMenu.as_ptr(),
					hInstance.as_ptr(),
					lpParam.unwrap_or_default() as *mut c_void,
				)
			}
		) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`DefSubclassProc`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-defsubclassproc)
	/// method.
	pub fn DefSubclassProc<P: Into<Wm>>(self, uMsg: P) -> isize {
		let wmAny: Wm = uMsg.into();
		unsafe {
			comctl32::DefSubclassProc(
				self.0, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
			)
		}
	}

	/// [`DefWindowProc`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// method.
	pub fn DefWindowProc<P: Into<Wm>>(self, Msg: P) -> isize {
		let wmAny: Wm = Msg.into();
		unsafe {
			user32::DefWindowProcW(
				self.0, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
			)
		}
	}

	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// method.
	pub fn DestroyWindow(self) {
		unsafe { user32::DestroyWindow(self.0); }
	}

	/// [`EnableWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// method.
	pub fn EnableWindow(self, bEnable: bool) -> bool {
		unsafe { user32::EnableWindow(self.0, bEnable as i32) != 0 }
	}

	/// [`EndDialog`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// method.
	pub fn EndDialog(self, nResult: isize) -> Result<(), co::ERROR> {
		match unsafe { user32::EndDialog(self.0, nResult) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// method.
	pub fn EndPaint(self, lpPaint: &PAINTSTRUCT) {
		unsafe { user32::EndPaint(self.0, const_void(lpPaint)); }
	}

	/// [`EnumChildWindows`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows)
	/// method.
	///
	/// This method can be more performant than
	/// [`EnumChildWindowsVec`](crate::HWND::EnumChildWindowsVec), which passes
	/// through all children and allocates a `Vec`. However, it has the
	/// inconvenient of the manual function pointer.
	pub fn EnumChildWindows(self, lpEnumFunc: WNDENUMPROC, lParam: isize) {
		unsafe {
			user32::EnumChildWindows(self.0, lpEnumFunc as *const c_void, lParam);
		}
	}

	/// A more convenient [`EnumChildWindows`](co::HWND::EnumChildWindows), which
	/// returns a `Vec` with the handles of all child windows, instead of taking
	/// a function pointer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// for hchild in my_hwnd.EnumChildWindowsVec() {
	///   println!("HWND: {}", hchild);
	/// }
	/// ```
	pub fn EnumChildWindowsVec(self) -> Vec<HWND> {
		let mut hchildren = Vec::new();
		self.EnumChildWindows(Self::EnumChildWindowsVecProc,
			&mut hchildren as *mut Vec<_> as isize);
		hchildren
	}

	extern "system" fn EnumChildWindowsVecProc(
		hchild: HWND, lparam: isize) -> i32
	{
		let hchildren = unsafe { &mut *(lparam as *mut Vec<HWND>) };
		hchildren.push(hchild);
		true as i32
	}

	/// [`FindWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	pub fn FindWindow(
		lpClassName: &str, lpWindowName: &str) -> Result<HWND, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::FindWindowW(
					WString::from_str(lpClassName).as_ptr(),
					WString::from_str(lpWindowName).as_ptr(),
				)
			}
		) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`GetAncestor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// method.
	pub fn GetAncestor(self, gaFlags: co::GA) -> Option<HWND> {
		ptr_as_opt(unsafe { user32::GetAncestor(self.0, gaFlags.into()) })
			.map(|p| Self(p))
	}

	/// [`GetClassLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclasslongptrw)
	/// method.
	pub fn GetClassLongPtr(self, nIndex: co::GCLP) -> usize {
		unsafe { user32::GetClassLongPtrW(self.0, nIndex.into()) }
	}

	/// [`GetClientRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
	/// method.
	pub fn GetClientRect(self) -> Result<RECT, co::ERROR> {
		let mut rc = RECT::default();
		match unsafe { user32::GetClientRect(self.0, mut_void(&mut rc)) } {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// method.
	pub fn GetDC(self) -> Result<HDC, ()> {
		match ptr_as_opt(unsafe { user32::GetDC(self.0) }) {
			Some(p) => Ok(unsafe { HDC::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`GetDesktopWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)
	/// static method.
	pub fn GetDesktopWindow() -> HWND {
		Self(unsafe { user32::GetDesktopWindow() })
	}

	/// [`GetDlgCtrlID`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgctrlid)
	/// method.
	pub fn GetDlgCtrlID(self) -> Result<i32, co::ERROR> {
		match unsafe { user32::GetDlgCtrlID(self.0) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual ID is zero
				err => Err(err),
			},
			id => Ok(id),
		}
	}

	/// [`GetDlgItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgitem)
	/// method.
	pub fn GetDlgItem(self, nIDDlgItem: i32) -> Result<Option<HWND>, co::ERROR> {
		match ptr_as_opt(unsafe { user32::GetDlgItem(self.0, nIDDlgItem) }) {
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no actual window
				err => Err(err),
			},
			Some(p) => Ok(Some(Self(p))),
		}
	}

	/// [`GetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// static method.
	pub fn GetFocus() -> Option<HWND> {
		ptr_as_opt(unsafe { user32::GetFocus() })
			.map(|p| Self(p))
	}

	/// [`GetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// static method.
	pub fn GetForegroundWindow() -> Option<HWND> {
		ptr_as_opt(unsafe { user32::GetForegroundWindow() })
			.map(|p| Self(p))
	}

	/// [`GetNextDlgGroupItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// method.
	pub fn GetNextDlgGroupItem(
		&self, hCtl: HWND, bPrevious: bool) -> Result<HWND, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::GetNextDlgGroupItem(self.0, hCtl.0, bPrevious as i32)
			}
		) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`GetNextDlgTabItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// method.
	pub fn GetNextDlgTabItem(
		&self, hCtl: HWND, bPrevious: bool) -> Result<HWND, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::GetNextDlgTabItem(self.0, hCtl.0, bPrevious as i32)
			}
		) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// method.
	pub fn GetParent(self) -> Result<Option<HWND>, co::ERROR> {
		match ptr_as_opt(unsafe { user32::GetParent(self.0) }) {
			Some(p) => Ok(Some(Self(p))),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no actual parent
				err => Err(err),
			},
		}
	}

	/// [`GetUpdateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// method.
	pub fn GetUpdateRgn(
		&self, hRgn: HRGN, bErase: bool) -> Result<co::REGION, ()>
	{
		match unsafe {
			user32::GetUpdateRgn(self.0, hRgn.as_ptr(), bErase as i32)
		} {
			0 => Err(()),
			ret => Ok(co::REGION::from(ret)),
		}
	}

	/// [`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// method.
	pub fn GetWindow(self, uCmd: co::GW) -> Result<Option<HWND>, co::ERROR> {
		match ptr_as_opt(unsafe { user32::GetWindow(self.0, uCmd.into()) }) {
			Some(p) => Ok(Some(Self(p))),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no actual window
				err => Err(err),
			},
		}
	}

	/// [`GetWindowDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// method.
	pub fn GetWindowDC(self) -> Result<HDC, ()> {
		match ptr_as_opt(unsafe { user32::GetWindowDC(self.0) }) {
			Some(p) => Ok(unsafe { HDC::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`GetWindowInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// method.
	pub fn GetWindowInfo(self, pwi: &mut WINDOWINFO) -> Result<(), co::ERROR> {
		match unsafe { user32::GetWindowInfo(self.0, mut_void(pwi)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// method.
	pub fn GetWindowLongPtr(self, nIndex: co::GWLP) -> isize {
		unsafe { user32::GetWindowLongPtrW(self.0, nIndex.into()) }
	}

	/// [`GetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// method.
	pub fn GetWindowPlacement(
		&self, lpwndpl: &mut WINDOWPLACEMENT) -> Result<(), co::ERROR>
	{
		match unsafe { user32::GetWindowPlacement(self.0, mut_void(lpwndpl)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetWindowRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// method.
	pub fn GetWindowRect(self) -> Result<RECT, co::ERROR> {
		let mut rc = RECT::default();
		match unsafe { user32::GetWindowRect(self.0, mut_void(&mut rc)) } {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// method.
	pub fn GetWindowRgn(self, hRgn: HRGN) -> Result<co::REGION, ()> {
		match unsafe { user32::GetWindowRgn(self.0, hRgn.as_ptr()) } {
			0 => Err(()),
			ret => Ok(co::REGION::from(ret)),
		}
	}

	/// [`GetWindowRgnBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// method.
	pub fn GetWindowRgnBox(self, lprc: &mut RECT) -> Result<co::REGION, ()> {
		match unsafe { user32::GetWindowRgnBox(self.0, mut_void(lprc)) } {
			0 => Err(()),
			ret => Ok(co::REGION::from(ret)),
		}
	}

	/// [`GetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)
	/// method.
	///
	/// The passed buffer will be automatically allocated with
	/// [`GetWindowTextLength`](crate::HWND::GetWindowTextLength).
	///
	/// This method can be more performant than
	/// [`GetWindowTextStr`](crate::HWND::GetWindowTextStr) because the buffer
	/// can be reused, avoiding multiple allocations. However, it has the
	/// inconvenient of the manual conversion from `WString` to `String`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// let mut buf = WString::new();
	/// my_hwnd.GetWindowText(&mut buf).unwrap();
	/// println!("Text: {}", buf.to_string());
	/// ```
	pub fn GetWindowText(self, buf: &mut WString) -> Result<i32, co::ERROR> {
		match self.GetWindowTextLength()? {
			0 => { // window has no text, simply clear buffer
				buf.realloc_buffer(0);
				Ok(0)
			},
			len => {
				buf.realloc_buffer(len as usize + 1); // plus terminating null

				match unsafe {
					user32::GetWindowTextW(self.0, buf.as_mut_ptr(), len + 1)
				} {
					0 => match GetLastError() {
						co::ERROR::SUCCESS => {
							buf.realloc_buffer(0); // no chars copied for some reason
							Ok(0)
						},
						err => Err(err),
					},
					nCopied => Ok(nCopied), // return number of copied chars without terminating null
				}
			},
		}
	}

	/// [`GetWindowTextLength`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthw)
	/// method. Does not count the terminating null.
	pub fn GetWindowTextLength(self) -> Result<i32, co::ERROR> {
		SetLastError(co::ERROR::SUCCESS);

		match unsafe { user32::GetWindowTextLengthW(self.0) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero length
				err => Err(err),
			},
			len => Ok(len),
		}
	}

	/// A more convenient [`GetWindowText`](crate::HWND::GetWindowText), which
	/// returns a `String` instead of requiring an external buffer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// let text = my_hwnd.GetWindowTextStr().unwrap();
	/// println!("Text: {}", text);
	/// ```
	pub fn GetWindowTextStr(self) -> Result<String, co::ERROR> {
		let mut buf = WString::new();
		self.GetWindowText(&mut buf)?;
		Ok(buf.to_string())
	}

	/// [`HiliteMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hilitemenuitem)
	/// method.
	pub fn HiliteMenuItem(
		&self, hMenu: HMENU, uIDHiliteItem: IdPos, uHilite: co::MF) -> bool
	{
		unsafe {
			user32::HiliteMenuItem(self.0,
				hMenu.as_ptr(), uIDHiliteItem.into(), uHilite.into()) != 0
		}
	}

	/// [`InvalidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect)
	/// method.
	///
	/// # Examples
	///
	/// Most of the time you'll just want update the entire client area:
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.InvalidateRect(None, true)
	///   .unwrap();
	/// ```
	pub fn InvalidateRect(
		&self, lpRect: Option<&RECT>, bErase: bool) -> Result<(), ()>
	{
		match unsafe {
			user32::InvalidateRect(
				self.0,
				lpRect.map_or(
					std::ptr::null(),
					|lpRect| const_void(lpRect),
				),
				bErase as i32,
			)
		} {
			0 => Err(()),
			_ => Ok(()),
		}
	}

	/// [`InvalidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// method.
	pub fn InvalidateRgn(self, hRgn: HRGN, bErase: bool) {
		unsafe { user32::InvalidateRgn(self.0, hRgn.as_ptr(), bErase as i32); }
	}

	/// [`IsChild`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// method.
	pub fn IsChild(self, hWndPossibleChild: HWND) -> bool {
		unsafe { user32::IsChild(self.0, hWndPossibleChild.0) != 0 }
	}

	/// [`IsDialogMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// method.
	pub fn IsDialogMessage(self, lpMsg: &mut MSG) -> bool {
		unsafe { user32::IsDialogMessageW(self.0, mut_void(lpMsg)) != 0 }
	}

	/// [`IsIconic`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic)
	/// method.
	pub fn IsIconic(self) -> bool {
		unsafe { user32::IsIconic(self.0) != 0 }
	}

	/// [`IsWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)
	/// method.
	pub fn IsWindow(self) -> bool {
		unsafe { user32::IsWindow(self.0) != 0 }
	}

	/// [`IsWindowEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowenabled)
	/// method.
	pub fn IsWindowEnabled(self) -> bool {
		unsafe { user32::IsWindowEnabled(self.0) != 0 }
	}

	/// [`IsWindowVisible`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible)
	/// method.
	pub fn IsWindowVisible(self) -> bool {
		unsafe { user32::IsWindowVisible(self.0) != 0 }
	}

	/// [`MapDialogRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// method.
	pub fn MapDialogRect(self, lpRect: &mut RECT) -> Result<(), co::ERROR> {
		match unsafe { user32::MapDialogRect(self.0, mut_void(lpRect)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.MessageBox("Hello, world", "title", co::MB::OKCANCEL | co::MB::ICONINFORMATION)
	///   .unwrap();
	/// ```
	///
	/// Usually the message box has a valid parent window, however, if for some
	/// reason you don't have a window to serve as parent, you still can show a
	/// non-modal, parent-less message box by retrieving the desktop handle:
	///
	/// ```rust,ignore
	/// HWND::GetDesktopWindow()
	///   .MessageBox("Hello, world", "Title", co::MB::ICONEXCLAMATION)
	///   .unwrap();
	/// ```
	pub fn MessageBox(self, lpText: &str,
		lpCaption: &str, uType: co::MB) -> Result<co::DLGID, co::ERROR>
	{
		match unsafe {
			user32::MessageBoxW(
				self.0,
				WString::from_str(lpText).as_ptr(),
				WString::from_str(lpCaption).as_ptr(),
				uType.into(),
			)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::DLGID::from(ret as u16)),
		}
	}

	/// [`PostMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)
	/// method.
	pub fn PostMessage<P: Into<Wm>>(self, Msg: P) -> Result<(), co::ERROR> {
		let wmAny: Wm = Msg.into();
		match unsafe {
			user32::PostMessageW(
				self.0, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`RemoveWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// method.
	pub fn RemoveWindowSubclass(
		&self, pfnSubclass: SUBCLASSPROC, uIdSubclass: usize) -> Result<(), ()>
	{
		match unsafe {
			comctl32::RemoveWindowSubclass(self.0,
				pfnSubclass as *const c_void, uIdSubclass)
		} {
			0 => Err(()),
			_ => Ok(()),
		}
	}

	/// [`SendMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// method.
	///
	/// Instead of receiving a message code followed by `WPARAM` and `LPARAM`,
	/// receives a single message argument, which allows you to pass the message
	/// parameters safely.
	///
	/// # Examples
	///
	/// Sending a [`WM_CLOSE`](crate::msg::WmClose) message, which has no
	/// parameters:
	/// ```rust,ignore
	/// use winsafe::msg::WmClose;
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.SendMessage(WmClose {});
	/// ```
	///
	/// Sending a [`LVM_SETITEM`](crate::msg::LvmSetItem) list view message,
	/// which demands a reference to an [`LVITEM`](crate::LVITEM) object:
	/// ```rust,ignore
	/// use winsafe::co;
	/// use winsafe::msg::LvmSetItem;
	/// use winsafe::LVITEM, HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// let mut lvi = LVITEM::default(); // object to be sent
	/// lvi.mask = co::LVIF::IMAGE;
	/// lvi.iImage = 3;
	///
	/// my_hwnd.SendMessage(LvmSetItem {
	///   lvitem: &lvi,
	/// });
	/// ```
	pub fn SendMessage<P: Into<Wm>>(self, Msg: P) -> isize {
		let wmAny: Wm = Msg.into();
		unsafe {
			user32::SendMessageW(
				self.0, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
			)
		}
	}

	/// [`SetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setfocus)
	/// method.
	pub fn SetFocus(self) -> Option<HWND> {
		ptr_as_opt(unsafe { user32::SetFocus(self.0) })
			.map(|p| Self(p))
	}

	/// [`SetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// method.
	pub fn SetParent(
		&self, hWndNewParent: HWND) -> Result<Option<HWND>, co::ERROR>
	{
		match ptr_as_opt(unsafe { user32::SetParent(self.0, hWndNewParent.0) }) {
			Some(p) => Ok(Some(Self(p))),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no previous parent
				err => Err(err),
			},
		}
	}

	/// [`SetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// method.
	pub fn SetWindowLongPtr(self, nIndex: co::GWLP, dwNewLong: isize) -> isize {
		unsafe { user32::SetWindowLongPtrW(self.0, nIndex.into(), dwNewLong) }
	}

	/// [`SetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// method.
	pub fn SetWindowPlacement(
		&self, lpwndpl: &WINDOWPLACEMENT) -> Result<(), co::ERROR>
	{
		match unsafe { user32::SetWindowPlacement(self.0, const_void(lpwndpl)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// method.
	pub fn SetWindowPos(self,
		hWndInsertAfter: HwndPlace,
		X: i32, Y: i32, cx: u32, cy: u32, uFlags: co::SWP) -> Result<(), co::ERROR>
	{
		match unsafe {
			user32::SetWindowPos(
				self.0, hWndInsertAfter.as_ptr(),
				X, Y, cx as i32, cy as i32, uFlags.into(),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowrgn)
	/// method.
	pub fn SetWindowRgn(self, hRgn: HRGN, bRedraw: bool) -> Result<(), ()> {
		match unsafe {
			user32::SetWindowRgn(self.0, hRgn.as_ptr(), bRedraw as i32)
		} {
			0 => Err(()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// method.
	pub fn SetWindowSubclass(self, pfnSubclass: SUBCLASSPROC,
		uIdSubclass: usize, dwRefData: usize) -> Result<(), ()>
	{
		match unsafe {
			comctl32::SetWindowSubclass(self.0,
				pfnSubclass as *const c_void, uIdSubclass, dwRefData)
		} {
			0 => Err(()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// method.
	pub fn SetWindowText(self, lpString: &str) -> Result<(), co::ERROR> {
		match unsafe {
			user32::SetWindowTextW(self.0, WString::from_str(lpString).as_ptr())
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	pub fn ShowWindow(self, nCmdShow: co::SW) -> bool {
		unsafe { user32::ShowWindow(self.0, nCmdShow.into()) != 0 }
	}

	/// [`TranslateAccelerator`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// method.
	pub fn TranslateAccelerator(
		&self, hAccTable: HACCEL, lpMsg: &mut MSG) -> Result<(), co::ERROR>
	{
		match unsafe {
			user32::TranslateAcceleratorW(
				self.0, hAccTable.as_ptr(), mut_void(lpMsg))
		} {
			0 => Err(GetLastError()),
			_ => Ok(())
		}
	}

	/// [`UpdateWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// method.
	pub fn UpdateWindow(self) -> Result<(), ()> {
		match unsafe { user32::UpdateWindow(self.0) } {
			0 => Err(()),
			_ => Ok(()),
		}
	}

	/// [`ValidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect)
	/// method.
	pub fn ValidateRect(self, lpRect: &RECT) -> Result<(), ()> {
		match unsafe { user32::ValidateRect(self.0, const_void(lpRect)) } {
			0 => Err(()),
			_ => Ok(()),
		}
	}

	/// [`ValidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// method.
	pub fn ValidateRgn(self, hRgn: HRGN) -> Result<(), ()> {
		match unsafe { user32::ValidateRgn(self.0, hRgn.as_ptr()) } {
			0 => Err(()),
			_ => Ok(()),
		}
	}
}
