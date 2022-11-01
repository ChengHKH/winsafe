#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, HEAPLIST32, MODULEENTRY32, PROCESSENTRY32, SysResult,
	THREADENTRY32,
};
use crate::prelude::{Handle, HandleClose};

impl_handle! { HPROCESSLIST: "kernel";
	/// Handle to a process list
	/// [snapshot](https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HPROCESSLIST {}
impl kernel_Hprocesslist for HPROCESSLIST {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HPROCESSLIST`](crate::HPROCESSLIST).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hprocesslist: Handle {
	/// Returns an iterator over the heaps of a process, with
	/// [`HEAPLIST32`](crate::HEAPLIST32) structs. Calls
	/// [`HPROCESSLIST::Heap32ListFirst`](crate::prelude::kernel_Hprocesslist::Heap32ListFirst)
	/// and then
	/// [`HPROCESSLIST::Heap32ListNext`](crate::prelude::kernel_Hprocesslist::Heap32ListNext)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPHEAPLIST, None)?;
	///
	/// for heap_entry in hpl.iter_heaps() {
	///     let heap_entry = heap_entry?;
	///     let is_default_heap = heap_entry.dwFlags == co::HF32::DEFAULT;
	///     println!("{} {}",
	///         heap_entry.th32HeapID, heap_entry.th32ProcessID);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_heaps<'a>(&'a self)
		-> Box<dyn Iterator<Item = SysResult<&'a HEAPLIST32>> + 'a>
	{
		Box::new(HeapIter::new(HPROCESSLIST(unsafe { self.as_ptr() })))
	}

	/// Returns an iterator over the modules of a process, with
	/// [`MODULEENTRY32`](crate::MODULEENTRY32) structs. Calls
	/// [`HPROCESSLIST::Module32First`](crate::prelude::kernel_Hprocesslist::Module32First)
	/// and then
	/// [`HPROCESSLIST::Module32Next`](crate::prelude::kernel_Hprocesslist::Module32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPMODULE, None)?;
	///
	/// for mod_entry in hpl.iter_modules() {
	///     let mod_entry = mod_entry?;
	///     println!("{} {}",
	///         mod_entry.szModule(), mod_entry.th32ProcessID);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_modules<'a>(&'a self)
		-> Box<dyn Iterator<Item = SysResult<&'a MODULEENTRY32>> + 'a>
	{
		Box::new(ModuleIter::new(HPROCESSLIST(unsafe { self.as_ptr() })))
	}

	/// Returns an iterator over the processes of a process, with
	/// [`PROCESSENTRY32`](crate::PROCESSENTRY32) structs. Calls
	/// [`HPROCESSLIST::Process32First`](crate::prelude::kernel_Hprocesslist::Process32First)
	/// and then
	/// [`HPROCESSLIST::Process32Next`](crate::prelude::kernel_Hprocesslist::Process32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)?;
	///
	/// for proc_entry in hpl.iter_processes() {
	///     let proc_entry = proc_entry?;
	///     println!("{} {} {}",
	///         proc_entry.szExeFile(), proc_entry.th32ProcessID, proc_entry.cntThreads);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_processes<'a>(&'a self)
		-> Box<dyn Iterator<Item = SysResult<&'a PROCESSENTRY32>> + 'a>
	{
		Box::new(ProcessIter::new(HPROCESSLIST(unsafe { self.as_ptr() })))
	}

	/// Returns an iterator over the threads of a process, with
	/// [`THREADENTRY32`](crate::THREADENTRY32) structs. Calls
	/// [`HPROCESSLIST::Thread32First`](crate::prelude::kernel_Hprocesslist::Thread32First)
	/// and then
	/// [`HPROCESSLIST::Thread32Next`](crate::prelude::kernel_Hprocesslist::Thread32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESS, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::CreateToolhelp32Snapshot(
	///     co::TH32CS::SNAPTHREAD,
	///     Some(HPROCESS::GetCurrentProcessId()),
	/// )?;
	///
	/// for thread_entry in hpl.iter_threads() {
	///     let thread_entry = thread_entry?;
	///     println!("{} {}",
	///         thread_entry.th32ThreadID, thread_entry.th32OwnerProcessID);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_threads<'a>(&'a self)
		-> Box<dyn Iterator<Item = SysResult<&'a THREADENTRY32>> + 'a>
	{
		Box::new(ThreadIter::new(HPROCESSLIST(unsafe { self.as_ptr() })))
	}

	/// [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HPROCESSLIST::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	#[must_use]
	fn CreateToolhelp32Snapshot(
		flags: co::TH32CS,
		th32_process_id: Option<u32>) -> SysResult<HPROCESSLIST>
	{
		unsafe {
			kernel::ffi::CreateToolhelp32Snapshot(
				flags.0,
				th32_process_id.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| HPROCESSLIST(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`HeapList32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-heap32listfirst)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_heaps`](crate::prelude::kernel_Hprocesslist::iter_heaps),
	/// which is simpler.
	#[must_use]
	fn Heap32ListFirst(self, hl: &mut HEAPLIST32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Heap32ListFirst(self.as_ptr(), hl as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`HeapList32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-heap32listnext)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_heaps`](crate::prelude::kernel_Hprocesslist::iter_heaps),
	/// which is simpler.
	#[must_use]
	fn Heap32ListNext(self, hl: &mut HEAPLIST32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Heap32ListNext(self.as_ptr(), hl as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Module32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32firstw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::prelude::kernel_Hprocesslist::iter_modules),
	/// which is simpler.
	#[must_use]
	fn Module32First(self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Module32FirstW(self.as_ptr(), me as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Module32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32nextw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::prelude::kernel_Hprocesslist::iter_modules),
	/// which is simpler.
	#[must_use]
	fn Module32Next(self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Module32NextW(self.as_ptr(), me as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_processes`](crate::prelude::kernel_Hprocesslist::iter_processes),
	/// which is simpler.
	#[must_use]
	fn Process32First(self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Process32FirstW(self.as_ptr(), pe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_processes`](crate::prelude::kernel_Hprocesslist::iter_processes),
	/// which is simpler.
	#[must_use]
	fn Process32Next(self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Process32NextW(self.as_ptr(), pe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Thread32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32first)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::prelude::kernel_Hprocesslist::iter_threads),
	/// which is simpler.
	#[must_use]
	fn Thread32First(self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Thread32First(self.as_ptr(), te as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Thread32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32next)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::prelude::kernel_Hprocesslist::iter_threads),
	/// which is simpler.
	#[must_use]
	fn Thread32Next(self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Thread32Next(self.as_ptr(), te as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}

//------------------------------------------------------------------------------

struct HeapIter<'a> {
	hpl: HPROCESSLIST,
	hl32: HEAPLIST32,
	first_pass: bool,
	has_more: bool,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for HeapIter<'a> {
	type Item = SysResult<&'a HEAPLIST32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Heap32ListFirst(&mut self.hl32)
		} else {
			self.hpl.Heap32ListNext(&mut self.hl32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.hl32 as *const HEAPLIST32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no heap found
				}
			},
		}
	}
}

impl<'a> HeapIter<'a> {
	fn new(hpl: HPROCESSLIST) -> Self {
		Self {
			hpl,
			hl32: HEAPLIST32::default(),
			first_pass: true,
			has_more: true,
			_owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

struct ModuleIter<'a> {
	hpl: HPROCESSLIST,
	me32: MODULEENTRY32,
	first_pass: bool,
	has_more: bool,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ModuleIter<'a> {
	type Item = SysResult<&'a MODULEENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Module32First(&mut self.me32)
		} else {
			self.hpl.Module32Next(&mut self.me32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.me32 as *const MODULEENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no module found
				}
			},
		}
	}
}

impl<'a> ModuleIter<'a> {
	fn new(hpl: HPROCESSLIST) -> Self {
		Self {
			hpl,
			me32: MODULEENTRY32::default(),
			first_pass: true,
			has_more: true,
			_owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

struct ProcessIter<'a> {
	hpl: HPROCESSLIST,
	pe32: PROCESSENTRY32,
	first_pass: bool,
	has_more: bool,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ProcessIter<'a> {
	type Item = SysResult<&'a PROCESSENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Process32First(&mut self.pe32)
		} else {
			self.hpl.Process32Next(&mut self.pe32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.pe32 as *const PROCESSENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no process found
				}
			},
		}
	}
}

impl<'a> ProcessIter<'a> {
	fn new(hpl: HPROCESSLIST) -> Self {
		Self {
			hpl,
			pe32: PROCESSENTRY32::default(),
			first_pass: true,
			has_more: true,
			_owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

struct ThreadIter<'a> {
	hpl: HPROCESSLIST,
	te32: THREADENTRY32,
	first_pass: bool,
	has_more: bool,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ThreadIter<'a> {
	type Item = SysResult<&'a THREADENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Thread32First(&mut self.te32)
		} else {
			self.hpl.Thread32Next(&mut self.te32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.te32 as *const THREADENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no thread found
				}
			},
		}
	}
}

impl<'a> ThreadIter<'a> {
	fn new(hpl: HPROCESSLIST) -> Self {
		Self {
			hpl,
			te32: THREADENTRY32::default(),
			first_pass: true,
			has_more: true,
			_owner: PhantomData,
		}
	}
}