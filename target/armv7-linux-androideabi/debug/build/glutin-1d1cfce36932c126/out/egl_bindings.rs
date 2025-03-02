
        mod __gl_imports {
            pub use std::mem;
            pub use std::os::raw;
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = extern "system" fn(source: GLenum,
                                          gltype: GLenum,
                                          id: GLuint,
                                          severity: GLenum,
                                          length: GLsizei,
                                          message: *const GLchar,
                                          userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint,
                                             category: GLenum,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

// platform-specific aliases are unknown
// IMPORTANT: these are alises to the same level of the bindings
// the values must be defined by the user
#[allow(dead_code)]
pub type khronos_utime_nanoseconds_t = super::khronos_utime_nanoseconds_t;
#[allow(dead_code)]
pub type khronos_uint64_t = super::khronos_uint64_t;
#[allow(dead_code)]
pub type khronos_ssize_t = super::khronos_ssize_t;
pub type EGLNativeDisplayType = super::EGLNativeDisplayType;
#[allow(dead_code)]
pub type EGLNativePixmapType = super::EGLNativePixmapType;
#[allow(dead_code)]
pub type EGLNativeWindowType = super::EGLNativeWindowType;
pub type EGLint = super::EGLint;
#[allow(dead_code)]
pub type NativeDisplayType = super::NativeDisplayType;
#[allow(dead_code)]
pub type NativePixmapType = super::NativePixmapType;
#[allow(dead_code)]
pub type NativeWindowType = super::NativeWindowType;

// EGL alises
pub type Bool = EGLBoolean; // TODO: not sure
pub type EGLBoolean = super::__gl_imports::raw::c_uint;
pub type EGLenum = super::__gl_imports::raw::c_uint;
pub type EGLAttribKHR = isize;
pub type EGLAttrib = isize;
pub type EGLConfig = *const super::__gl_imports::raw::c_void;
pub type EGLContext = *const super::__gl_imports::raw::c_void;
pub type EGLDeviceEXT = *const super::__gl_imports::raw::c_void;
pub type EGLDisplay = *const super::__gl_imports::raw::c_void;
pub type EGLSurface = *const super::__gl_imports::raw::c_void;
pub type EGLClientBuffer = *const super::__gl_imports::raw::c_void;
pub type __eglMustCastToProperFunctionPointerType = extern "system" fn() -> ();
pub type EGLImageKHR = *const super::__gl_imports::raw::c_void;
pub type EGLImage = *const super::__gl_imports::raw::c_void;
pub type EGLOutputLayerEXT = *const super::__gl_imports::raw::c_void;
pub type EGLOutputPortEXT = *const super::__gl_imports::raw::c_void;
pub type EGLSyncKHR = *const super::__gl_imports::raw::c_void;
pub type EGLSync = *const super::__gl_imports::raw::c_void;
pub type EGLTimeKHR = khronos_utime_nanoseconds_t;
pub type EGLTime = khronos_utime_nanoseconds_t;
pub type EGLSyncNV = *const super::__gl_imports::raw::c_void;
pub type EGLTimeNV = khronos_utime_nanoseconds_t;
pub type EGLuint64NV = khronos_utime_nanoseconds_t;
pub type EGLStreamKHR = *const super::__gl_imports::raw::c_void;
pub type EGLuint64KHR = khronos_uint64_t;
pub type EGLNativeFileDescriptorKHR = super::__gl_imports::raw::c_int;
pub type EGLsizeiANDROID = khronos_ssize_t;
pub type EGLSetBlobFuncANDROID = extern "system" fn(*const super::__gl_imports::raw::c_void,
                                                    EGLsizeiANDROID,
                                                    *const super::__gl_imports::raw::c_void,
                                                    EGLsizeiANDROID)
                                                    -> ();
pub type EGLGetBlobFuncANDROID = extern "system" fn(*const super::__gl_imports::raw::c_void,
                                                    EGLsizeiANDROID,
                                                    *mut super::__gl_imports::raw::c_void,
                                                    EGLsizeiANDROID)
                                                    -> EGLsizeiANDROID;

#[repr(C)]
pub struct EGLClientPixmapHI {
    pData: *const super::__gl_imports::raw::c_void,
    iWidth: EGLint,
    iHeight: EGLint,
    iStride: EGLint,
}

}
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_FORMAT: types::GLenum = 0x3088;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_FORMAT_NONPRE: types::GLenum = 0x308B;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_FORMAT_PRE: types::GLenum = 0x308C;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_MASK_SIZE: types::GLenum = 0x303E;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_SIZE: types::GLenum = 0x3021;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_BUFFER: types::GLenum = 0x3084;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_ACCESS: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_ALLOC: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_ATTRIBUTE: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_CONFIG: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_CONTEXT: types::GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_CURRENT_SURFACE: types::GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_DISPLAY: types::GLenum = 0x3008;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_MATCH: types::GLenum = 0x3009;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_NATIVE_PIXMAP: types::GLenum = 0x300A;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_NATIVE_WINDOW: types::GLenum = 0x300B;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_PARAMETER: types::GLenum = 0x300C;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_SURFACE: types::GLenum = 0x300D;
#[allow(dead_code, non_upper_case_globals)] pub const BIND_TO_TEXTURE_RGB: types::GLenum = 0x3039;
#[allow(dead_code, non_upper_case_globals)] pub const BIND_TO_TEXTURE_RGBA: types::GLenum = 0x303A;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_SIZE: types::GLenum = 0x3022;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_DESTROYED: types::GLenum = 0x3095;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_PRESERVED: types::GLenum = 0x3094;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 0x3020;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_APIS: types::GLenum = 0x308D;
#[allow(dead_code, non_upper_case_globals)] pub const CL_EVENT_HANDLE: types::GLenum = 0x309C;
#[allow(dead_code, non_upper_case_globals)] pub const COLORSPACE: types::GLenum = 0x3087;
#[allow(dead_code, non_upper_case_globals)] pub const COLORSPACE_LINEAR: types::GLenum = 0x308A;
#[allow(dead_code, non_upper_case_globals)] pub const COLORSPACE_sRGB: types::GLenum = 0x3089;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_TYPE: types::GLenum = 0x303F;
#[allow(dead_code, non_upper_case_globals)] pub const CONDITION_SATISFIED: types::GLenum = 0x30F6;
#[allow(dead_code, non_upper_case_globals)] pub const CONFIG_CAVEAT: types::GLenum = 0x3027;
#[allow(dead_code, non_upper_case_globals)] pub const CONFIG_ID: types::GLenum = 0x3028;
#[allow(dead_code, non_upper_case_globals)] pub const CONFORMANT: types::GLenum = 0x3042;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_CLIENT_TYPE: types::GLenum = 0x3097;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_CLIENT_VERSION: types::GLenum = 0x3098;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAGS_KHR: types::GLenum = 0x30FC;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_LOST: types::GLenum = 0x300E;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_MAJOR_VERSION: types::GLenum = 0x3098;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_MAJOR_VERSION_KHR: types::GLenum = 0x3098;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_MINOR_VERSION: types::GLenum = 0x30FB;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_MINOR_VERSION_KHR: types::GLenum = 0x30FB;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT_KHR: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_CORE_PROFILE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_CORE_PROFILE_BIT_KHR: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_DEBUG: types::GLenum = 0x31B0;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_DEBUG_BIT_KHR: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_FORWARD_COMPATIBLE: types::GLenum = 0x31B1;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_FORWARD_COMPATIBLE_BIT_KHR: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_NO_ERROR_KHR: types::GLenum = 0x31B3;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_PROFILE_MASK: types::GLenum = 0x30FD;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_PROFILE_MASK_KHR: types::GLenum = 0x30FD;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY: types::GLenum = 0x31BD;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY_EXT: types::GLenum = 0x3138;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY_KHR: types::GLenum = 0x31BD;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_ROBUST_ACCESS: types::GLenum = 0x31B2;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_ROBUST_ACCESS_BIT_KHR: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_OPENGL_ROBUST_ACCESS_EXT: types::GLenum = 0x30BF;
#[allow(dead_code, non_upper_case_globals)] pub const CORE_NATIVE_ENGINE: types::GLenum = 0x305B;
#[allow(dead_code, non_upper_case_globals)] pub const DEFAULT_DISPLAY: types::EGLNativeDisplayType = 0 as types::EGLNativeDisplayType;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_SIZE: types::GLenum = 0x3025;
#[allow(dead_code, non_upper_case_globals)] pub const DISPLAY_SCALING: types::GLenum = 10000;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::EGLint = -1 as types::EGLint;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW: types::GLenum = 0x3059;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x3055;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FOREVER: types::GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const GL_COLORSPACE: types::GLenum = 0x309D;
#[allow(dead_code, non_upper_case_globals)] pub const GL_COLORSPACE_LINEAR: types::GLenum = 0x308A;
#[allow(dead_code, non_upper_case_globals)] pub const GL_COLORSPACE_SRGB: types::GLenum = 0x3089;
#[allow(dead_code, non_upper_case_globals)] pub const GL_RENDERBUFFER: types::GLenum = 0x30B9;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_2D: types::GLenum = 0x30B1;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_3D: types::GLenum = 0x30B2;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x30B4;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x30B6;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x30B8;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x30B3;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x30B5;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x30B7;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_LEVEL: types::GLenum = 0x30BC;
#[allow(dead_code, non_upper_case_globals)] pub const GL_TEXTURE_ZOFFSET: types::GLenum = 0x30BD;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_SIZE: types::GLenum = 0x3023;
#[allow(dead_code, non_upper_case_globals)] pub const HEIGHT: types::GLenum = 0x3056;
#[allow(dead_code, non_upper_case_globals)] pub const HORIZONTAL_RESOLUTION: types::GLenum = 0x3090;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_PRESERVED: types::GLenum = 0x30D2;
#[allow(dead_code, non_upper_case_globals)] pub const LARGEST_PBUFFER: types::GLenum = 0x3058;
#[allow(dead_code, non_upper_case_globals)] pub const LEVEL: types::GLenum = 0x3029;
#[allow(dead_code, non_upper_case_globals)] pub const LOSE_CONTEXT_ON_RESET: types::GLenum = 0x31BF;
#[allow(dead_code, non_upper_case_globals)] pub const LOSE_CONTEXT_ON_RESET_EXT: types::GLenum = 0x31BF;
#[allow(dead_code, non_upper_case_globals)] pub const LOSE_CONTEXT_ON_RESET_KHR: types::GLenum = 0x31BF;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE_BUFFER: types::GLenum = 0x308F;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE_SIZE: types::GLenum = 0x303D;
#[allow(dead_code, non_upper_case_globals)] pub const MATCH_NATIVE_PIXMAP: types::GLenum = 0x3041;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PBUFFER_HEIGHT: types::GLenum = 0x302A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PBUFFER_PIXELS: types::GLenum = 0x302B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PBUFFER_WIDTH: types::GLenum = 0x302C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SWAP_INTERVAL: types::GLenum = 0x303C;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_SWAP_INTERVAL: types::GLenum = 0x303B;
#[allow(dead_code, non_upper_case_globals)] pub const MIPMAP_LEVEL: types::GLenum = 0x3083;
#[allow(dead_code, non_upper_case_globals)] pub const MIPMAP_TEXTURE: types::GLenum = 0x3082;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE_RESOLVE: types::GLenum = 0x3099;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE_RESOLVE_BOX: types::GLenum = 0x309B;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE_RESOLVE_BOX_BIT: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE_RESOLVE_DEFAULT: types::GLenum = 0x309A;
#[allow(dead_code, non_upper_case_globals)] pub const NATIVE_RENDERABLE: types::GLenum = 0x302D;
#[allow(dead_code, non_upper_case_globals)] pub const NATIVE_VISUAL_ID: types::GLenum = 0x302E;
#[allow(dead_code, non_upper_case_globals)] pub const NATIVE_VISUAL_TYPE: types::GLenum = 0x302F;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0x3038;
#[allow(dead_code, non_upper_case_globals)] pub const NON_CONFORMANT_CONFIG: types::GLenum = 0x3051;
#[allow(dead_code, non_upper_case_globals)] pub const NOT_INITIALIZED: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const NO_CONTEXT: types::EGLContext = 0 as types::EGLContext;
#[allow(dead_code, non_upper_case_globals)] pub const NO_DISPLAY: types::EGLDisplay = 0 as types::EGLDisplay;
#[allow(dead_code, non_upper_case_globals)] pub const NO_IMAGE: types::EGLImage = 0 as types::EGLImage;
#[allow(dead_code, non_upper_case_globals)] pub const NO_RESET_NOTIFICATION: types::GLenum = 0x31BE;
#[allow(dead_code, non_upper_case_globals)] pub const NO_RESET_NOTIFICATION_EXT: types::GLenum = 0x31BE;
#[allow(dead_code, non_upper_case_globals)] pub const NO_RESET_NOTIFICATION_KHR: types::GLenum = 0x31BE;
#[allow(dead_code, non_upper_case_globals)] pub const NO_SURFACE: types::EGLSurface = 0 as types::EGLSurface;
#[allow(dead_code, non_upper_case_globals)] pub const NO_SYNC: types::EGLSync = 0 as types::EGLSync;
#[allow(dead_code, non_upper_case_globals)] pub const NO_TEXTURE: types::GLenum = 0x305C;
#[allow(dead_code, non_upper_case_globals)] pub const OPENGL_API: types::GLenum = 0x30A2;
#[allow(dead_code, non_upper_case_globals)] pub const OPENGL_BIT: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const OPENGL_ES2_BIT: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const OPENGL_ES3_BIT: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const OPENGL_ES3_BIT_KHR: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const OPENGL_ES_API: types::GLenum = 0x30A0;
#[allow(dead_code, non_upper_case_globals)] pub const OPENGL_ES_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const OPENVG_API: types::GLenum = 0x30A1;
#[allow(dead_code, non_upper_case_globals)] pub const OPENVG_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const OPENVG_IMAGE: types::GLenum = 0x3096;
#[allow(dead_code, non_upper_case_globals)] pub const PBUFFER_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_ASPECT_RATIO: types::GLenum = 0x3092;
#[allow(dead_code, non_upper_case_globals)] pub const PIXMAP_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_ANDROID_KHR: types::GLenum = 0x3141;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_DEVICE_EXT: types::GLenum = 0x313F;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_GBM_KHR: types::GLenum = 0x31D7;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_GBM_MESA: types::GLenum = 0x31D7;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_WAYLAND_EXT: types::GLenum = 0x31D8;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_WAYLAND_KHR: types::GLenum = 0x31D8;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_X11_EXT: types::GLenum = 0x31D5;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_X11_KHR: types::GLenum = 0x31D5;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_X11_SCREEN_EXT: types::GLenum = 0x31D6;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_X11_SCREEN_KHR: types::GLenum = 0x31D6;
#[allow(dead_code, non_upper_case_globals)] pub const READ: types::GLenum = 0x305A;
#[allow(dead_code, non_upper_case_globals)] pub const RED_SIZE: types::GLenum = 0x3024;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERABLE_TYPE: types::GLenum = 0x3040;
#[allow(dead_code, non_upper_case_globals)] pub const RENDER_BUFFER: types::GLenum = 0x3086;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_BUFFER: types::GLenum = 0x308E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x3031;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x3032;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNALED: types::GLenum = 0x30F2;
#[allow(dead_code, non_upper_case_globals)] pub const SINGLE_BUFFER: types::GLenum = 0x3085;
#[allow(dead_code, non_upper_case_globals)] pub const SLOW_CONFIG: types::GLenum = 0x3050;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_SIZE: types::GLenum = 0x3026;
#[allow(dead_code, non_upper_case_globals)] pub const SUCCESS: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const SURFACE_TYPE: types::GLenum = 0x3033;
#[allow(dead_code, non_upper_case_globals)] pub const SWAP_BEHAVIOR: types::GLenum = 0x3093;
#[allow(dead_code, non_upper_case_globals)] pub const SWAP_BEHAVIOR_PRESERVED_BIT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CL_EVENT: types::GLenum = 0x30FE;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CL_EVENT_COMPLETE: types::GLenum = 0x30FF;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CONDITION: types::GLenum = 0x30F8;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FENCE: types::GLenum = 0x30F9;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLUSH_COMMANDS_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_PRIOR_COMMANDS_COMPLETE: types::GLenum = 0x30F0;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_STATUS: types::GLenum = 0x30F1;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_TYPE: types::GLenum = 0x30F7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x305F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FORMAT: types::GLenum = 0x3080;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RGB: types::GLenum = 0x305D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RGBA: types::GLenum = 0x305E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_TARGET: types::GLenum = 0x3081;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_EXPIRED: types::GLenum = 0x30F5;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPARENT_BLUE_VALUE: types::GLenum = 0x3035;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPARENT_GREEN_VALUE: types::GLenum = 0x3036;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPARENT_RED_VALUE: types::GLenum = 0x3037;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPARENT_RGB: types::GLenum = 0x3052;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPARENT_TYPE: types::GLenum = 0x3034;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const UNKNOWN: types::EGLint = -1 as types::EGLint;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNALED: types::GLenum = 0x30F3;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x3053;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x3054;
#[allow(dead_code, non_upper_case_globals)] pub const VERTICAL_RESOLUTION: types::GLenum = 0x3091;
#[allow(dead_code, non_upper_case_globals)] pub const VG_ALPHA_FORMAT: types::GLenum = 0x3088;
#[allow(dead_code, non_upper_case_globals)] pub const VG_ALPHA_FORMAT_NONPRE: types::GLenum = 0x308B;
#[allow(dead_code, non_upper_case_globals)] pub const VG_ALPHA_FORMAT_PRE: types::GLenum = 0x308C;
#[allow(dead_code, non_upper_case_globals)] pub const VG_ALPHA_FORMAT_PRE_BIT: types::GLenum = 0x0040;
#[allow(dead_code, non_upper_case_globals)] pub const VG_COLORSPACE: types::GLenum = 0x3087;
#[allow(dead_code, non_upper_case_globals)] pub const VG_COLORSPACE_LINEAR: types::GLenum = 0x308A;
#[allow(dead_code, non_upper_case_globals)] pub const VG_COLORSPACE_LINEAR_BIT: types::GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)] pub const VG_COLORSPACE_sRGB: types::GLenum = 0x3089;
#[allow(dead_code, non_upper_case_globals)] pub const WIDTH: types::GLenum = 0x3057;
#[allow(dead_code, non_upper_case_globals)] pub const WINDOW_BIT: types::GLenum = 0x0004;

        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #[derive(Copy, Clone)]
        pub struct Egl;
impl Egl {
            /// Stub function.
            #[allow(dead_code)]
            pub fn load_with<F>(mut _loadfn: F) -> Egl where F: FnMut(&str) -> *const __gl_imports::raw::c_void {
                Egl
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn BindAPI(&self, api: types::EGLenum) -> types::EGLBoolean {
                BindAPI(api)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn BindTexImage(&self, dpy: types::EGLDisplay, surface: types::EGLSurface, buffer: types::EGLint) -> types::EGLBoolean {
                BindTexImage(dpy, surface, buffer)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn ChooseConfig(&self, dpy: types::EGLDisplay, attrib_list: *const types::EGLint, configs: *mut types::EGLConfig, config_size: types::EGLint, num_config: *mut types::EGLint) -> types::EGLBoolean {
                ChooseConfig(dpy, attrib_list, configs, config_size, num_config)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn ClientWaitSync(&self, dpy: types::EGLDisplay, sync: types::EGLSync, flags: types::EGLint, timeout: types::EGLTime) -> types::EGLint {
                ClientWaitSync(dpy, sync, flags, timeout)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CopyBuffers(&self, dpy: types::EGLDisplay, surface: types::EGLSurface, target: types::EGLNativePixmapType) -> types::EGLBoolean {
                CopyBuffers(dpy, surface, target)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreateContext(&self, dpy: types::EGLDisplay, config: types::EGLConfig, share_context: types::EGLContext, attrib_list: *const types::EGLint) -> types::EGLContext {
                CreateContext(dpy, config, share_context, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreateImage(&self, dpy: types::EGLDisplay, ctx: types::EGLContext, target: types::EGLenum, buffer: types::EGLClientBuffer, attrib_list: *const types::EGLAttrib) -> types::EGLImage {
                CreateImage(dpy, ctx, target, buffer, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreatePbufferFromClientBuffer(&self, dpy: types::EGLDisplay, buftype: types::EGLenum, buffer: types::EGLClientBuffer, config: types::EGLConfig, attrib_list: *const types::EGLint) -> types::EGLSurface {
                CreatePbufferFromClientBuffer(dpy, buftype, buffer, config, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreatePbufferSurface(&self, dpy: types::EGLDisplay, config: types::EGLConfig, attrib_list: *const types::EGLint) -> types::EGLSurface {
                CreatePbufferSurface(dpy, config, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreatePixmapSurface(&self, dpy: types::EGLDisplay, config: types::EGLConfig, pixmap: types::EGLNativePixmapType, attrib_list: *const types::EGLint) -> types::EGLSurface {
                CreatePixmapSurface(dpy, config, pixmap, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreatePlatformPixmapSurface(&self, dpy: types::EGLDisplay, config: types::EGLConfig, native_pixmap: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLAttrib) -> types::EGLSurface {
                CreatePlatformPixmapSurface(dpy, config, native_pixmap, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreatePlatformPixmapSurfaceEXT(&self, dpy: types::EGLDisplay, config: types::EGLConfig, native_pixmap: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLint) -> types::EGLSurface {
                CreatePlatformPixmapSurfaceEXT(dpy, config, native_pixmap, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreatePlatformWindowSurface(&self, dpy: types::EGLDisplay, config: types::EGLConfig, native_window: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLAttrib) -> types::EGLSurface {
                CreatePlatformWindowSurface(dpy, config, native_window, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreatePlatformWindowSurfaceEXT(&self, dpy: types::EGLDisplay, config: types::EGLConfig, native_window: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLint) -> types::EGLSurface {
                CreatePlatformWindowSurfaceEXT(dpy, config, native_window, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreateSync(&self, dpy: types::EGLDisplay, type_: types::EGLenum, attrib_list: *const types::EGLAttrib) -> types::EGLSync {
                CreateSync(dpy, type_, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn CreateWindowSurface(&self, dpy: types::EGLDisplay, config: types::EGLConfig, win: types::EGLNativeWindowType, attrib_list: *const types::EGLint) -> types::EGLSurface {
                CreateWindowSurface(dpy, config, win, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn DestroyContext(&self, dpy: types::EGLDisplay, ctx: types::EGLContext) -> types::EGLBoolean {
                DestroyContext(dpy, ctx)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn DestroyImage(&self, dpy: types::EGLDisplay, image: types::EGLImage) -> types::EGLBoolean {
                DestroyImage(dpy, image)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn DestroySurface(&self, dpy: types::EGLDisplay, surface: types::EGLSurface) -> types::EGLBoolean {
                DestroySurface(dpy, surface)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn DestroySync(&self, dpy: types::EGLDisplay, sync: types::EGLSync) -> types::EGLBoolean {
                DestroySync(dpy, sync)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetConfigAttrib(&self, dpy: types::EGLDisplay, config: types::EGLConfig, attribute: types::EGLint, value: *mut types::EGLint) -> types::EGLBoolean {
                GetConfigAttrib(dpy, config, attribute, value)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetConfigs(&self, dpy: types::EGLDisplay, configs: *mut types::EGLConfig, config_size: types::EGLint, num_config: *mut types::EGLint) -> types::EGLBoolean {
                GetConfigs(dpy, configs, config_size, num_config)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetCurrentContext(&self, ) -> types::EGLContext {
                GetCurrentContext()
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetCurrentDisplay(&self, ) -> types::EGLDisplay {
                GetCurrentDisplay()
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetCurrentSurface(&self, readdraw: types::EGLint) -> types::EGLSurface {
                GetCurrentSurface(readdraw)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetDisplay(&self, display_id: types::EGLNativeDisplayType) -> types::EGLDisplay {
                GetDisplay(display_id)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetError(&self, ) -> types::EGLint {
                GetError()
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetPlatformDisplay(&self, platform: types::EGLenum, native_display: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLAttrib) -> types::EGLDisplay {
                GetPlatformDisplay(platform, native_display, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetPlatformDisplayEXT(&self, platform: types::EGLenum, native_display: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLint) -> types::EGLDisplay {
                GetPlatformDisplayEXT(platform, native_display, attrib_list)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetProcAddress(&self, procname: *const __gl_imports::raw::c_char) -> types::__eglMustCastToProperFunctionPointerType {
                GetProcAddress(procname)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn GetSyncAttrib(&self, dpy: types::EGLDisplay, sync: types::EGLSync, attribute: types::EGLint, value: *mut types::EGLAttrib) -> types::EGLBoolean {
                GetSyncAttrib(dpy, sync, attribute, value)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn Initialize(&self, dpy: types::EGLDisplay, major: *mut types::EGLint, minor: *mut types::EGLint) -> types::EGLBoolean {
                Initialize(dpy, major, minor)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn MakeCurrent(&self, dpy: types::EGLDisplay, draw: types::EGLSurface, read: types::EGLSurface, ctx: types::EGLContext) -> types::EGLBoolean {
                MakeCurrent(dpy, draw, read, ctx)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn QueryAPI(&self, ) -> types::EGLenum {
                QueryAPI()
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn QueryContext(&self, dpy: types::EGLDisplay, ctx: types::EGLContext, attribute: types::EGLint, value: *mut types::EGLint) -> types::EGLBoolean {
                QueryContext(dpy, ctx, attribute, value)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn QueryString(&self, dpy: types::EGLDisplay, name: types::EGLint) -> *const __gl_imports::raw::c_char {
                QueryString(dpy, name)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn QuerySurface(&self, dpy: types::EGLDisplay, surface: types::EGLSurface, attribute: types::EGLint, value: *mut types::EGLint) -> types::EGLBoolean {
                QuerySurface(dpy, surface, attribute, value)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn ReleaseTexImage(&self, dpy: types::EGLDisplay, surface: types::EGLSurface, buffer: types::EGLint) -> types::EGLBoolean {
                ReleaseTexImage(dpy, surface, buffer)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn ReleaseThread(&self, ) -> types::EGLBoolean {
                ReleaseThread()
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn SurfaceAttrib(&self, dpy: types::EGLDisplay, surface: types::EGLSurface, attribute: types::EGLint, value: types::EGLint) -> types::EGLBoolean {
                SurfaceAttrib(dpy, surface, attribute, value)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn SwapBuffers(&self, dpy: types::EGLDisplay, surface: types::EGLSurface) -> types::EGLBoolean {
                SwapBuffers(dpy, surface)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn SwapInterval(&self, dpy: types::EGLDisplay, interval: types::EGLint) -> types::EGLBoolean {
                SwapInterval(dpy, interval)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn Terminate(&self, dpy: types::EGLDisplay) -> types::EGLBoolean {
                Terminate(dpy)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn WaitClient(&self, ) -> types::EGLBoolean {
                WaitClient()
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn WaitGL(&self, ) -> types::EGLBoolean {
                WaitGL()
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn WaitNative(&self, engine: types::EGLint) -> types::EGLBoolean {
                WaitNative(engine)
            }
#[allow(non_snake_case)]
            // #[allow(unused_variables)]
            #[allow(dead_code)]
            #[inline]
            pub unsafe fn WaitSync(&self, dpy: types::EGLDisplay, sync: types::EGLSync, flags: types::EGLint) -> types::EGLBoolean {
                WaitSync(dpy, sync, flags)
            }
}

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(dead_code)]
        extern "system" {
#[link_name="eglBindAPI"] fn BindAPI(api: types::EGLenum) -> types::EGLBoolean;
#[link_name="eglBindTexImage"] fn BindTexImage(dpy: types::EGLDisplay, surface: types::EGLSurface, buffer: types::EGLint) -> types::EGLBoolean;
#[link_name="eglChooseConfig"] fn ChooseConfig(dpy: types::EGLDisplay, attrib_list: *const types::EGLint, configs: *mut types::EGLConfig, config_size: types::EGLint, num_config: *mut types::EGLint) -> types::EGLBoolean;
#[link_name="eglClientWaitSync"] fn ClientWaitSync(dpy: types::EGLDisplay, sync: types::EGLSync, flags: types::EGLint, timeout: types::EGLTime) -> types::EGLint;
#[link_name="eglCopyBuffers"] fn CopyBuffers(dpy: types::EGLDisplay, surface: types::EGLSurface, target: types::EGLNativePixmapType) -> types::EGLBoolean;
#[link_name="eglCreateContext"] fn CreateContext(dpy: types::EGLDisplay, config: types::EGLConfig, share_context: types::EGLContext, attrib_list: *const types::EGLint) -> types::EGLContext;
#[link_name="eglCreateImage"] fn CreateImage(dpy: types::EGLDisplay, ctx: types::EGLContext, target: types::EGLenum, buffer: types::EGLClientBuffer, attrib_list: *const types::EGLAttrib) -> types::EGLImage;
#[link_name="eglCreatePbufferFromClientBuffer"] fn CreatePbufferFromClientBuffer(dpy: types::EGLDisplay, buftype: types::EGLenum, buffer: types::EGLClientBuffer, config: types::EGLConfig, attrib_list: *const types::EGLint) -> types::EGLSurface;
#[link_name="eglCreatePbufferSurface"] fn CreatePbufferSurface(dpy: types::EGLDisplay, config: types::EGLConfig, attrib_list: *const types::EGLint) -> types::EGLSurface;
#[link_name="eglCreatePixmapSurface"] fn CreatePixmapSurface(dpy: types::EGLDisplay, config: types::EGLConfig, pixmap: types::EGLNativePixmapType, attrib_list: *const types::EGLint) -> types::EGLSurface;
#[link_name="eglCreatePlatformPixmapSurface"] fn CreatePlatformPixmapSurface(dpy: types::EGLDisplay, config: types::EGLConfig, native_pixmap: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLAttrib) -> types::EGLSurface;
#[link_name="eglCreatePlatformPixmapSurfaceEXT"] fn CreatePlatformPixmapSurfaceEXT(dpy: types::EGLDisplay, config: types::EGLConfig, native_pixmap: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLint) -> types::EGLSurface;
#[link_name="eglCreatePlatformWindowSurface"] fn CreatePlatformWindowSurface(dpy: types::EGLDisplay, config: types::EGLConfig, native_window: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLAttrib) -> types::EGLSurface;
#[link_name="eglCreatePlatformWindowSurfaceEXT"] fn CreatePlatformWindowSurfaceEXT(dpy: types::EGLDisplay, config: types::EGLConfig, native_window: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLint) -> types::EGLSurface;
#[link_name="eglCreateSync"] fn CreateSync(dpy: types::EGLDisplay, type_: types::EGLenum, attrib_list: *const types::EGLAttrib) -> types::EGLSync;
#[link_name="eglCreateWindowSurface"] fn CreateWindowSurface(dpy: types::EGLDisplay, config: types::EGLConfig, win: types::EGLNativeWindowType, attrib_list: *const types::EGLint) -> types::EGLSurface;
#[link_name="eglDestroyContext"] fn DestroyContext(dpy: types::EGLDisplay, ctx: types::EGLContext) -> types::EGLBoolean;
#[link_name="eglDestroyImage"] fn DestroyImage(dpy: types::EGLDisplay, image: types::EGLImage) -> types::EGLBoolean;
#[link_name="eglDestroySurface"] fn DestroySurface(dpy: types::EGLDisplay, surface: types::EGLSurface) -> types::EGLBoolean;
#[link_name="eglDestroySync"] fn DestroySync(dpy: types::EGLDisplay, sync: types::EGLSync) -> types::EGLBoolean;
#[link_name="eglGetConfigAttrib"] fn GetConfigAttrib(dpy: types::EGLDisplay, config: types::EGLConfig, attribute: types::EGLint, value: *mut types::EGLint) -> types::EGLBoolean;
#[link_name="eglGetConfigs"] fn GetConfigs(dpy: types::EGLDisplay, configs: *mut types::EGLConfig, config_size: types::EGLint, num_config: *mut types::EGLint) -> types::EGLBoolean;
#[link_name="eglGetCurrentContext"] fn GetCurrentContext() -> types::EGLContext;
#[link_name="eglGetCurrentDisplay"] fn GetCurrentDisplay() -> types::EGLDisplay;
#[link_name="eglGetCurrentSurface"] fn GetCurrentSurface(readdraw: types::EGLint) -> types::EGLSurface;
#[link_name="eglGetDisplay"] fn GetDisplay(display_id: types::EGLNativeDisplayType) -> types::EGLDisplay;
#[link_name="eglGetError"] fn GetError() -> types::EGLint;
#[link_name="eglGetPlatformDisplay"] fn GetPlatformDisplay(platform: types::EGLenum, native_display: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLAttrib) -> types::EGLDisplay;
#[link_name="eglGetPlatformDisplayEXT"] fn GetPlatformDisplayEXT(platform: types::EGLenum, native_display: *mut __gl_imports::raw::c_void, attrib_list: *const types::EGLint) -> types::EGLDisplay;
#[link_name="eglGetProcAddress"] fn GetProcAddress(procname: *const __gl_imports::raw::c_char) -> types::__eglMustCastToProperFunctionPointerType;
#[link_name="eglGetSyncAttrib"] fn GetSyncAttrib(dpy: types::EGLDisplay, sync: types::EGLSync, attribute: types::EGLint, value: *mut types::EGLAttrib) -> types::EGLBoolean;
#[link_name="eglInitialize"] fn Initialize(dpy: types::EGLDisplay, major: *mut types::EGLint, minor: *mut types::EGLint) -> types::EGLBoolean;
#[link_name="eglMakeCurrent"] fn MakeCurrent(dpy: types::EGLDisplay, draw: types::EGLSurface, read: types::EGLSurface, ctx: types::EGLContext) -> types::EGLBoolean;
#[link_name="eglQueryAPI"] fn QueryAPI() -> types::EGLenum;
#[link_name="eglQueryContext"] fn QueryContext(dpy: types::EGLDisplay, ctx: types::EGLContext, attribute: types::EGLint, value: *mut types::EGLint) -> types::EGLBoolean;
#[link_name="eglQueryString"] fn QueryString(dpy: types::EGLDisplay, name: types::EGLint) -> *const __gl_imports::raw::c_char;
#[link_name="eglQuerySurface"] fn QuerySurface(dpy: types::EGLDisplay, surface: types::EGLSurface, attribute: types::EGLint, value: *mut types::EGLint) -> types::EGLBoolean;
#[link_name="eglReleaseTexImage"] fn ReleaseTexImage(dpy: types::EGLDisplay, surface: types::EGLSurface, buffer: types::EGLint) -> types::EGLBoolean;
#[link_name="eglReleaseThread"] fn ReleaseThread() -> types::EGLBoolean;
#[link_name="eglSurfaceAttrib"] fn SurfaceAttrib(dpy: types::EGLDisplay, surface: types::EGLSurface, attribute: types::EGLint, value: types::EGLint) -> types::EGLBoolean;
#[link_name="eglSwapBuffers"] fn SwapBuffers(dpy: types::EGLDisplay, surface: types::EGLSurface) -> types::EGLBoolean;
#[link_name="eglSwapInterval"] fn SwapInterval(dpy: types::EGLDisplay, interval: types::EGLint) -> types::EGLBoolean;
#[link_name="eglTerminate"] fn Terminate(dpy: types::EGLDisplay) -> types::EGLBoolean;
#[link_name="eglWaitClient"] fn WaitClient() -> types::EGLBoolean;
#[link_name="eglWaitGL"] fn WaitGL() -> types::EGLBoolean;
#[link_name="eglWaitNative"] fn WaitNative(engine: types::EGLint) -> types::EGLBoolean;
#[link_name="eglWaitSync"] fn WaitSync(dpy: types::EGLDisplay, sync: types::EGLSync, flags: types::EGLint) -> types::EGLBoolean;
}
