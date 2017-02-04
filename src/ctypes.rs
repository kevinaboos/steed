#![allow(non_camel_case_types)]

#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "powerpc64"))]
pub type c_char = u8;

#[cfg(any(target_arch = "mips",
          target_arch = "mips64",
          target_arch = "sparc64",
          target_arch = "x86",
          target_arch = "x86_64"))]
pub type c_char = i8;

#[cfg(any(target_arch = "arm",
          target_arch = "mips",
          target_arch = "powerpc",
          target_arch = "x86"))]
pub type c_long = i32;

#[cfg(any(target_arch = "aarch64",
          target_arch = "mips64",
          target_arch = "powerpc64",
          target_arch = "sparc64",
          target_arch = "x86_64"))]
pub type c_long = i64;

#[cfg(any(target_arch = "arm",
          target_arch = "mips",
          target_arch = "powerpc",
          target_arch = "x86"))]
pub type c_ulong = u32;

#[cfg(any(target_arch = "aarch64",
          target_arch = "mips64",
          target_arch = "powerpc64",
          target_arch = "sparc64",
          target_arch = "x86_64"))]
pub type c_ulong = u64;

pub type c_uchar = u8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_ushort = u16;
pub type c_longlong = i64;

pub type size_t = usize;
pub type ssize_t = isize;

// Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help enable
// more optimization opportunities around it recognizing things like
// malloc/free.
#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}
