use std::{fmt, io, ptr};
use std::{ffi::CString, mem::size_of};
use std::mem::{self, MaybeUninit, size_of_val};
use std::ptr::NonNull;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, FALSE, HMODULE};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcesses, EnumProcessModules, GetModuleBaseNameA};
use winapi::um::winnt;
use winapi::um::winnt::PROCESS_VM_OPERATION;


const MAX_PROC_NAME_LENGTH: usize = 64;
const MAX_PIDS: usize = 1024;
static PROGRAM_PID: Option<&str> = option_env!("PID");


//enum processes() -> which will return an array of PIDs
pub fn enumerate_processes() -> io::Result<Vec<u32>> {
    // vec of DWORDs for the PIDs
    let mut pids = Vec::<DWORD>::with_capacity(MAX_PIDS);
    let mut size = 0;
    if unsafe {
        // pids.set_len(count);
        winapi::um::psapi::EnumProcesses(
            pids.as_mut_ptr(),
            (pids.capacity() * mem::size_of::<DWORD>()) as u32,
            &mut size,
        )
    } == FALSE {
        return Err(io::Error::last_os_error());
    }
    let count = size as usize / mem::size_of::<DWORD>();
    unsafe {
        pids.set_len(count);
    }
    Ok(pids)
}

#[derive(Debug)]
pub struct Process {
    pid: u32,
    handle: NonNull<c_void>,

}
//
//
// pub struct ProcessItem {
// 	pid: u32,
// 	name: String
// }
//
//
// impl fmt::Display for ProcessItem {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		write!(f, "{}   ===>  {}", self.name, self.pid)
// 	}
// }


impl Process {
    //open a process handle, given its process identifier
    pub fn open(pid: u32) -> io::Result<Self> {
        // the call doesnt have side effects
        NonNull::new(unsafe {
            winapi::um::processthreadsapi::OpenProcess(
                winnt::PROCESS_QUERY_INFORMATION
                    | winnt::PROCESS_VM_READ
                    | winnt::PROCESS_VM_WRITE
                    | PROCESS_VM_OPERATION,
                FALSE,
                pid,
            )
        })
            .map(|handle| Self { pid, handle })
            .ok_or_else(io::Error::last_os_error)
    }
    //return the process identifier
    pub fn pid(&self) -> u32 {
        self.pid
    }
    // return the base name of the first module loaded by this process

    pub fn module_base_name(&self) -> io::Result<String> {
        let mut module = MaybeUninit::<HMODULE>::uninit();
        let mut size = 0;
        //SAFETY : the pointer is valid and the size is correct
        if unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle.as_ptr(),
                module.as_mut_ptr(),
                mem::size_of::<HMODULE>() as u32,
                &mut size,
            )
        } == FALSE
        {
            return Err(io::Error::last_os_error());
        }

        //safety: the call succseeded so module is initialized
        let module = unsafe { module.assume_init() };

        let mut buffer = Vec::<u8>::with_capacity(MAX_PROC_NAME_LENGTH);

        let length = unsafe {
            winapi::um::psapi::GetModuleBaseNameA(
                self.handle.as_ptr(),
                module,
                buffer.as_mut_ptr().cast(),
                MAX_PROC_NAME_LENGTH as u32,
            )
        };
        if length == 0 {
            return Err(io::Error::last_os_error());
        }
        // safety : the call is successful , and length represents bytes
        unsafe { buffer.set_len(length as usize) }
        Ok(String::from_utf8(buffer).unwrap())
    }

    pub fn enumerate_modules(&self) -> io::Result<Vec<winapi::shared::minwindef::HMODULE>> {
        let mut size = 0;
        // safety : the pointer is valid and the indicated size is 0
        if unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle.as_ptr(),
                ptr::null_mut(),
                0,
                &mut size,
            )
        } == FALSE {
            return Err(io::Error::last_os_error());
        }

        let mut modules = Vec::with_capacity(size as usize / mem::size_of::<HMODULE>());
        //safety : the pointer is valid and the size is correct
        if unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle.as_ptr(),
                modules.as_mut_ptr() as *mut _,
                size,
                &mut size,
            )
        } == FALSE {
            return Err(io::Error::last_os_error());
        }

        unsafe {
            modules.set_len(size as usize / mem::size_of::<HMODULE>());
        }
        Ok(modules)
    }
}


// fn printProccess(p: &ProcessItem) {
//     println!("[-] {} , | {} |", p.name, p.pid);
// }


//for each of the PIDs -> openprocess()
//grab the process handle -> enum process modules which will return an array of modulehandles
// we take the first [0] getmodulebasename -> which returns a string of the module name
// impl Drop for Process {
// 	fn drop(&mut self) {
// 		todo!()
// 	}
// }


fn main(){
    let _ = enumerate_processes()
        .unwrap()
        .into_iter()
        .flat_map(Process::open)
        .flat_map(|proc| -> Result<(), std::io::Error> {
            let modbasename = proc.module_base_name()?;
            println!("[{}] {}", proc.pid, modbasename);
            Ok(())
        })
        .collect::<Vec<_>>();
}






