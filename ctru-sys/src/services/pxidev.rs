/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum PXIDEV_WaitType {
    WAIT_NONE = 0,
    WAIT_SLEEP = 1,
    WAIT_IREQ_RETURN = 2,
    WAIT_IREQ_CONTINUE = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum PXIDEV_DeassertType {
    DEASSERT_NONE = 0,
    DEASSERT_BEFORE_WAIT = 1,
    DEASSERT_AFTER_WAIT = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct PXIDEV_SPIBuffer {
    pub ptr: *mut ::libc::c_void,
    pub size: u32_,
    pub transferOption: u8_,
    pub waitOperation: u64_,
}
impl ::core::default::Default for PXIDEV_SPIBuffer {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
extern "C" {
    pub fn pxiDevInit() -> Result;
    pub fn pxiDevExit();
    pub fn PXIDEV_SPIMultiWriteRead(header: *mut PXIDEV_SPIBuffer,
                                    writeBuffer1: *mut PXIDEV_SPIBuffer,
                                    readBuffer1: *mut PXIDEV_SPIBuffer,
                                    writeBuffer2: *mut PXIDEV_SPIBuffer,
                                    readBuffer2: *mut PXIDEV_SPIBuffer,
                                    footer: *mut PXIDEV_SPIBuffer) -> Result;
    pub fn PXIDEV_SPIWriteRead(bytesRead: *mut u32_,
                               initialWaitOperation: u64_,
                               writeBuffer: *mut PXIDEV_SPIBuffer,
                               readBuffer: *mut PXIDEV_SPIBuffer) -> Result;
}
use ::types::*;