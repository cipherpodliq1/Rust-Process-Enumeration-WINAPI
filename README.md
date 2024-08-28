# Rust-Process-Enumeration-WINAPI
A rust based program for enumerating processes on a Windows Machine using Windows API.

![rustenumeration](https://github.com/user-attachments/assets/c975e143-0dae-4ed6-a7fa-47538669b0f4)

[+] The code enumerates running processes on a Windows-based machine,
opens a process handle and retrieves the module base name for the given process.

[+] Main functionality
 - Get a list of all running [PIDs](https://en.wikipedia.org/wiki/Process_identifier) in the format : 

  ![image](https://github.com/user-attachments/assets/74d14feb-eda6-4ab0-88c2-72a7b0cde669)
  
 - Process struct for encapsulating PID + handle

 - open(pid : u32) - Opens a process HANDLE , with ids PID
   
 - pid() -> returns the PID of the process

 - module_base_name() -> retrieves the name of the first module loaded by a process


[+] Constants and Imoports

MAX_PROC_NAME_LEN
MAX_PIDS (to be enumerated)

[+] Function Documentation

1. enumerate_processes() -> Enumerates all running processes, and returns a vector of PIDs or an I/O error.

[+] Struct and Methods 

1. Process struct with 2 fields: Pid(Process ID) , handle -> pointer to process handle

2. open(pid : u32) -> Opens a process HANDLE with specific access rights. Returns a process instance or an error.

3. pid() -> Simple getter for the process ID.

4. module_base_name() -> Retrieves the name of the first loaded module. Uses EnumProcessModules , GetModuleBaseNameA

5. enumerate_modules() -> Retrieves all module handles for the process.

[+] Main function
 - calls enumerate_process() to obtain the list of PIDs
 - FOR EACH PID, opens it with Process::open
 - Retrieves the module base name using module_vase_name
 - Outputs

I APPRECIATE CODING CRITISISM.THANKS IN ADVANCE.
NOT RESPONSIBLE FOR MISUSING.
