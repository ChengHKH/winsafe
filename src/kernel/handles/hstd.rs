#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::guard::CloseHandleGuard;
use crate::kernel::decl::{
	CONSOLE_READCONSOLE_CONTROL, GetLastError, SysResult, WString,
};
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;

impl_handle! { HSTD;
	/// Handle to a
	/// [standard device](https://learn.microsoft.com/en-us/windows/console/getstdhandle).
	/// Originally just a `HANDLE`.
}

impl kernel_Hstd for HSTD {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HSTD`](crate::HSTD).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hstd: Handle {
	/// [`FlushConsoleInputBuffer`](https://learn.microsoft.com/en-us/windows/console/flushconsoleinputbuffer)
	/// method.
	fn FlushConsoleInputBuffer(&self) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { kernel::ffi::FlushConsoleInputBuffer(self.as_ptr()) },
		)
	}

	/// [`GetConsoleMode`](https://learn.microsoft.com/en-us/windows/console/getconsolemode)
	/// method.
	#[must_use]
	fn GetConsoleMode(&self) -> SysResult<co::CONSOLE> {
		let mut mode = co::CONSOLE::NoValue;
		bool_to_sysresult(
			unsafe { kernel::ffi::GetConsoleMode(self.as_ptr(), &mut mode.0) },
		).map(|_| mode)
	}

	/// [`GetStdHandle`](https://learn.microsoft.com/en-us/windows/console/getstdhandle)
	/// static method.
	#[must_use]
	fn GetStdHandle(
		std_handle: co::STD_HANDLE,
	) -> SysResult<CloseHandleGuard<HSTD>>
	{
		unsafe {
			match HSTD::from_ptr(kernel::ffi::GetStdHandle(std_handle.0)) {
				HSTD::INVALID => Err(GetLastError()),
				handle => Ok(CloseHandleGuard::new(handle)),
			}
		}
	}

	/// [`ReadConsole`](https://learn.microsoft.com/en-us/windows/console/readconsole)
	/// method.
	/// 
	/// Returns the number of chars actually written.
	/// 
	/// # Examples
	/// 
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HSTD, WString};
	/// 
	/// let hstd = HSTD::GetStdHandle(co::STD_HANDLE::INPUT)?;
	/// 
	/// let mut buffer = WString::new_alloc_buf(2048);
	/// hstd.ReadConsole(&mut buffer, None)?;
	/// 
	/// let text = buffer.to_string();
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn ReadConsole(&self,
		buffer: &mut WString,
		input_control: Option<&CONSOLE_READCONSOLE_CONTROL>,
	) -> SysResult<u32>
	{
		let mut num_read = u32::default();
		bool_to_sysresult(
			unsafe {
				kernel::ffi::ReadConsoleW(
					self.as_ptr(),
					buffer.as_mut_ptr() as _,
					buffer.buf_len() as _,
					&mut num_read,
					input_control.map_or(std::ptr::null_mut(), |p| p as *const _ as _),
				)
			},
		).map(|_| num_read)
	}

	/// [`SetConsoleMode`](https://learn.microsoft.com/en-us/windows/console/setconsolemode)
	/// method.
	fn SetConsoleMode(&self, mode: co::CONSOLE) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { kernel::ffi::SetConsoleMode(self.as_ptr(), mode.0) },
		)
	}

	/// [`WriteConsole`](https://learn.microsoft.com/en-us/windows/console/writeconsole)
	/// method.
	/// 
	/// Returns the number of chars actually written.
	fn WriteConsole(&self, text: &str) -> SysResult<u32> {
		let buf = WString::from_str(text);
		let mut num_written = u32::default();

		unsafe {
			bool_to_sysresult(
				kernel::ffi::WriteConsoleW(
					self.as_ptr(),
					buf.as_ptr() as _,
					buf.str_len() as _,
					&mut num_written,
					std::ptr::null_mut(),
				),
			)
		}.map(|_| num_written)
	}
}
