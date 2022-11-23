#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, shell};
use crate::kernel::decl::{HINSTANCE, WString};
use crate::prelude::Handle;
use crate::user::decl::HWND;

impl shell_Hwnd for HWND {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_Hwnd: Handle {
	/// [`ShellExecute`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew)
	/// method.
	fn ShellExecute(&self,
		operation: &str,
		file: &str,
		parameters: Option<&str>,
		directory: Option<&str>,
		show_cmd: co::SW) -> Result<HINSTANCE, co::SE_ERR>
	{
		let ret = unsafe {
			shell::ffi::ShellExecuteW(
				self.as_ptr(),
				WString::from_str(operation).as_ptr(),
				WString::from_str(file).as_ptr(),
				parameters.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				directory.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				show_cmd.0,
			)
		};

		if ret <= 32 as _ {
			Err(co::SE_ERR(ret as _))
		} else {
			Ok(HINSTANCE(ret as _))
		}
	}
}
