/* automatically generated by rust-bindgen 0.63.0 */

#[doc(inline)]
pub use root::*;

#[doc(hidden)]
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    use crate::bindings::command::ConCommandBase;
    pub const COMMAND_COMPLETION_MAXITEMS: u32 = 64;
    pub const COMMAND_COMPLETION_ITEM_LENGTH: u32 = 128;
    pub const FCVAR_NONE: u32 = 0;
    pub const FCVAR_UNREGISTERED: u32 = 1;
    pub const FCVAR_DEVELOPMENTONLY: u32 = 2;
    pub const FCVAR_GAMEDLL: u32 = 4;
    pub const FCVAR_CLIENTDLL: u32 = 8;
    pub const FCVAR_HIDDEN: u32 = 16;
    pub const FCVAR_PROTECTED: u32 = 32;
    pub const FCVAR_SPONLY: u32 = 64;
    pub const FCVAR_ARCHIVE: u32 = 128;
    pub const FCVAR_NOTIFY: u32 = 256;
    pub const FCVAR_USERINFO: u32 = 512;
    pub const FCVAR_PRINTABLEONLY: u32 = 1024;
    pub const FCVAR_GAMEDLL_FOR_REMOTE_CLIENTS: u32 = 1024;
    pub const FCVAR_UNLOGGED: u32 = 2048;
    pub const FCVAR_NEVER_AS_STRING: u32 = 4096;
    pub const FCVAR_REPLICATED: u32 = 8192;
    pub const FCVAR_CHEAT: u32 = 16384;
    pub const FCVAR_SS: u32 = 32768;
    pub const FCVAR_DEMO: u32 = 65536;
    pub const FCVAR_DONTRECORD: u32 = 131072;
    pub const FCVAR_SS_ADDED: u32 = 262144;
    pub const FCVAR_RELEASE: u32 = 524288;
    pub const FCVAR_RELOAD_MATERIALS: u32 = 1048576;
    pub const FCVAR_RELOAD_TEXTURES: u32 = 2097152;
    pub const FCVAR_NOT_CONNECTED: u32 = 4194304;
    pub const FCVAR_MATERIAL_SYSTEM_THREAD: u32 = 8388608;
    pub const FCVAR_ARCHIVE_PLAYERPROFILE: u32 = 16777216;
    pub const FCVAR_SERVER_CAN_EXECUTE: u32 = 268435456;
    pub const FCVAR_SERVER_CANNOT_QUERY: u32 = 536870912;
    pub const FCVAR_CLIENTCMD_CAN_EXECUTE: u32 = 1073741824;
    pub const FCVAR_ACCESSIBLE_FROM_THREADS: u32 = 33554432;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct color24 {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }
    #[test]
    fn bindgen_test_layout_color24() {
        const UNINIT: ::std::mem::MaybeUninit<color24> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<color24>(),
            3usize,
            concat!("Size of: ", stringify!(color24))
        );
        assert_eq!(
            ::std::mem::align_of::<color24>(),
            1usize,
            concat!("Alignment of ", stringify!(color24))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(color24),
                "::",
                stringify!(r)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(color24),
                "::",
                stringify!(g)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(color24),
                "::",
                stringify!(b)
            )
        );
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct color32_s {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }
    #[test]
    fn bindgen_test_layout_color32_s() {
        const UNINIT: ::std::mem::MaybeUninit<color32_s> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<color32_s>(),
            4usize,
            concat!("Size of: ", stringify!(color32_s))
        );
        assert_eq!(
            ::std::mem::align_of::<color32_s>(),
            1usize,
            concat!("Alignment of ", stringify!(color32_s))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(color32_s),
                "::",
                stringify!(r)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(color32_s),
                "::",
                stringify!(g)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(color32_s),
                "::",
                stringify!(b)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
            3usize,
            concat!(
                "Offset of field: ",
                stringify!(color32_s),
                "::",
                stringify!(a)
            )
        );
    }
    extern "C" {
        #[link_name = "\u{1}?asInt@color32_s@@QEAAPEAIXZ"]
        pub fn color32_s_asInt(this: *mut root::color32_s) -> *mut ::std::os::raw::c_uint;
    }
    extern "C" {
        #[link_name = "\u{1}?asInt@color32_s@@QEBAPEBIXZ"]
        pub fn color32_s_asInt1(this: *const root::color32_s) -> *const ::std::os::raw::c_uint;
    }
    extern "C" {
        #[link_name = "\u{1}?Copy@color32_s@@QEAAXAEBU1@@Z"]
        pub fn color32_s_Copy(this: *mut root::color32_s, rhs: *const root::color32_s);
    }
    impl color32_s {
        #[inline]
        pub unsafe fn asInt(&mut self) -> *mut ::std::os::raw::c_uint {
            color32_s_asInt(self)
        }
        #[inline]
        pub unsafe fn asInt1(&self) -> *const ::std::os::raw::c_uint {
            color32_s_asInt1(self)
        }
        #[inline]
        pub unsafe fn Copy(&mut self, rhs: *const root::color32_s) {
            color32_s_Copy(self, rhs)
        }
    }
    pub type color32 = root::color32_s;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SourceColor {
        pub R: ::std::os::raw::c_uchar,
        pub G: ::std::os::raw::c_uchar,
        pub B: ::std::os::raw::c_uchar,
        pub A: ::std::os::raw::c_uchar,
    }
    #[test]
    fn bindgen_test_layout_SourceColor() {
        const UNINIT: ::std::mem::MaybeUninit<SourceColor> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<SourceColor>(),
            4usize,
            concat!("Size of: ", stringify!(SourceColor))
        );
        assert_eq!(
            ::std::mem::align_of::<SourceColor>(),
            1usize,
            concat!("Alignment of ", stringify!(SourceColor))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).R) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(SourceColor),
                "::",
                stringify!(R)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).G) as usize - ptr as usize },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(SourceColor),
                "::",
                stringify!(G)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).B) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(SourceColor),
                "::",
                stringify!(B)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).A) as usize - ptr as usize },
            3usize,
            concat!(
                "Offset of field: ",
                stringify!(SourceColor),
                "::",
                stringify!(A)
            )
        );
    }
    extern "C" {
        #[link_name = "\u{1}??0SourceColor@@QEAA@EEEE@Z"]
        pub fn SourceColor_SourceColor(
            this: *mut root::SourceColor,
            r: ::std::os::raw::c_uchar,
            g: ::std::os::raw::c_uchar,
            b: ::std::os::raw::c_uchar,
            a: ::std::os::raw::c_uchar,
        );
    }
    extern "C" {
        #[link_name = "\u{1}??0SourceColor@@QEAA@XZ"]
        pub fn SourceColor_SourceColor1(this: *mut root::SourceColor);
    }
    impl SourceColor {
        #[inline]
        pub unsafe fn new(
            r: ::std::os::raw::c_uchar,
            g: ::std::os::raw::c_uchar,
            b: ::std::os::raw::c_uchar,
            a: ::std::os::raw::c_uchar,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
            SourceColor_SourceColor(__bindgen_tmp.as_mut_ptr(), r, g, b, a);
            __bindgen_tmp.assume_init()
        }
        #[inline]
        pub unsafe fn new1() -> Self {
            let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
            SourceColor_SourceColor1(__bindgen_tmp.as_mut_ptr());
            __bindgen_tmp.assume_init()
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Color {
        pub _color: [::std::os::raw::c_uchar; 4usize],
    }
    #[test]
    fn bindgen_test_layout_Color() {
        const UNINIT: ::std::mem::MaybeUninit<Color> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<Color>(),
            4usize,
            concat!("Size of: ", stringify!(Color))
        );
        assert_eq!(
            ::std::mem::align_of::<Color>(),
            1usize,
            concat!("Alignment of ", stringify!(Color))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr)._color) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Color),
                "::",
                stringify!(_color)
            )
        );
    }
    extern "C" {
        #[link_name = "\u{1}?SetColor@Color@@QEAAXHHHH@Z"]
        pub fn Color_SetColor(
            this: *mut root::Color,
            _r: ::std::os::raw::c_int,
            _g: ::std::os::raw::c_int,
            _b: ::std::os::raw::c_int,
            _a: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}?GetColor@Color@@QEBAXAEAH000@Z"]
        pub fn Color_GetColor(
            this: *const root::Color,
            _r: *mut ::std::os::raw::c_int,
            _g: *mut ::std::os::raw::c_int,
            _b: *mut ::std::os::raw::c_int,
            _a: *mut ::std::os::raw::c_int,
        );
    }
    extern "C" {
        #[link_name = "\u{1}?GetValue@Color@@QEBAHH@Z"]
        pub fn Color_GetValue(
            this: *const root::Color,
            index: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}?SetRawColor@Color@@QEAAXH@Z"]
        pub fn Color_SetRawColor(this: *mut root::Color, color32: ::std::os::raw::c_int);
    }
    extern "C" {
        #[link_name = "\u{1}?GetRawColor@Color@@QEBAHXZ"]
        pub fn Color_GetRawColor(this: *const root::Color) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}?r@Color@@QEBAHXZ"]
        pub fn Color_r(this: *const root::Color) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}?g@Color@@QEBAHXZ"]
        pub fn Color_g(this: *const root::Color) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}?b@Color@@QEBAHXZ"]
        pub fn Color_b(this: *const root::Color) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}?a@Color@@QEBAHXZ"]
        pub fn Color_a(this: *const root::Color) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}?ToColor32@Color@@QEBA?AUcolor32_s@@XZ"]
        pub fn Color_ToColor32(this: *const root::Color) -> root::color32;
    }
    extern "C" {
        #[link_name = "\u{1}?ToSourceColor@Color@@QEAA?AUSourceColor@@XZ"]
        pub fn Color_ToSourceColor(this: *mut root::Color) -> root::SourceColor;
    }
    extern "C" {
        #[link_name = "\u{1}??0Color@@QEAA@HHHH@Z"]
        pub fn Color_Color(
            this: *mut root::Color,
            r: ::std::os::raw::c_int,
            g: ::std::os::raw::c_int,
            b: ::std::os::raw::c_int,
            a: ::std::os::raw::c_int,
        );
    }
    impl Color {
        #[inline]
        pub unsafe fn SetColor(
            &mut self,
            _r: ::std::os::raw::c_int,
            _g: ::std::os::raw::c_int,
            _b: ::std::os::raw::c_int,
            _a: ::std::os::raw::c_int,
        ) {
            Color_SetColor(self, _r, _g, _b, _a)
        }
        #[inline]
        pub unsafe fn GetColor(
            &self,
            _r: *mut ::std::os::raw::c_int,
            _g: *mut ::std::os::raw::c_int,
            _b: *mut ::std::os::raw::c_int,
            _a: *mut ::std::os::raw::c_int,
        ) {
            Color_GetColor(self, _r, _g, _b, _a)
        }
        #[inline]
        pub unsafe fn GetValue(&self, index: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
            Color_GetValue(self, index)
        }
        #[inline]
        pub unsafe fn SetRawColor(&mut self, color32: ::std::os::raw::c_int) {
            Color_SetRawColor(self, color32)
        }
        #[inline]
        pub unsafe fn GetRawColor(&self) -> ::std::os::raw::c_int {
            Color_GetRawColor(self)
        }
        #[inline]
        pub unsafe fn r(&self) -> ::std::os::raw::c_int {
            Color_r(self)
        }
        #[inline]
        pub unsafe fn g(&self) -> ::std::os::raw::c_int {
            Color_g(self)
        }
        #[inline]
        pub unsafe fn b(&self) -> ::std::os::raw::c_int {
            Color_b(self)
        }
        #[inline]
        pub unsafe fn a(&self) -> ::std::os::raw::c_int {
            Color_a(self)
        }
        #[inline]
        pub unsafe fn ToColor32(&self) -> root::color32 {
            Color_ToColor32(self)
        }
        #[inline]
        pub unsafe fn ToSourceColor(&mut self) -> root::SourceColor {
            Color_ToSourceColor(self)
        }
        #[inline]
        pub unsafe fn new(
            r: ::std::os::raw::c_int,
            g: ::std::os::raw::c_int,
            b: ::std::os::raw::c_int,
            a: ::std::os::raw::c_int,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
            Color_Color(__bindgen_tmp.as_mut_ptr(), r, g, b, a);
            __bindgen_tmp.assume_init()
        }
    }
    pub mod NS {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod Colors {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            extern "C" {
                #[link_name = "\u{1}?SCRIPT_UI@Colors@NS@@3VColor@@A"]
                pub static mut SCRIPT_UI: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?SCRIPT_CL@Colors@NS@@3VColor@@A"]
                pub static mut SCRIPT_CL: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?SCRIPT_SV@Colors@NS@@3VColor@@A"]
                pub static mut SCRIPT_SV: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?NATIVE_UI@Colors@NS@@3VColor@@A"]
                pub static mut NATIVE_UI: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?NATIVE_CL@Colors@NS@@3VColor@@A"]
                pub static mut NATIVE_CL: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?NATIVE_SV@Colors@NS@@3VColor@@A"]
                pub static mut NATIVE_SV: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?NATIVE_ENGINE@Colors@NS@@3VColor@@A"]
                pub static mut NATIVE_ENGINE: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?FILESYSTEM@Colors@NS@@3VColor@@A"]
                pub static mut FILESYSTEM: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?RPAK@Colors@NS@@3VColor@@A"]
                pub static mut RPAK: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?NORTHSTAR@Colors@NS@@3VColor@@A"]
                pub static mut NORTHSTAR: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?ECHO@Colors@NS@@3VColor@@A"]
                pub static mut ECHO: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?PLUGINSYS@Colors@NS@@3VColor@@A"]
                pub static mut PLUGINSYS: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?PLUGIN@Colors@NS@@3VColor@@A"]
                pub static mut PLUGIN: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?TRACE@Colors@NS@@3VColor@@A"]
                pub static mut TRACE: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?DEBUG@Colors@NS@@3VColor@@A"]
                pub static mut DEBUG: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?INFO@Colors@NS@@3VColor@@A"]
                pub static mut INFO: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?WARN@Colors@NS@@3VColor@@A"]
                pub static mut WARN: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?ERR@Colors@NS@@3VColor@@A"]
                pub static mut ERR: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?CRIT@Colors@NS@@3VColor@@A"]
                pub static mut CRIT: root::Color;
            }
            extern "C" {
                #[link_name = "\u{1}?OFF@Colors@NS@@3VColor@@A"]
                pub static mut OFF: root::Color;
            }
        }
    }
    pub type FnChangeCallback_t = ::std::option::Option<
        unsafe extern "C" fn(
            var: *mut root::ConVar,
            pOldValue: *const ::std::os::raw::c_char,
            flOldValue: f32,
        ),
    >;
    #[repr(C)]
    pub struct ConVar {
        pub m_ConCommandBase: root::ConCommandBase,
        pub m_pszDefaultValue: *const ::std::os::raw::c_char,
        pub m_Value: root::ConVar_CVValue_t,
        pub m_bHasMin: bool,
        pub m_fMinVal: f32,
        pub m_bHasMax: bool,
        pub m_fMaxVal: f32,
        pub m_pMalloc: *mut ::std::os::raw::c_void,
        pub m_pPad80: [::std::os::raw::c_char; 10usize],
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ConVar_CVValue_t {
        pub m_pszString: *const ::std::os::raw::c_char,
        pub m_iStringLength: i64,
        pub m_fValue: f32,
        pub m_nValue: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_ConVar_CVValue_t() {
        const UNINIT: ::std::mem::MaybeUninit<ConVar_CVValue_t> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<ConVar_CVValue_t>(),
            24usize,
            concat!("Size of: ", stringify!(ConVar_CVValue_t))
        );
        assert_eq!(
            ::std::mem::align_of::<ConVar_CVValue_t>(),
            8usize,
            concat!("Alignment of ", stringify!(ConVar_CVValue_t))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_pszString) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar_CVValue_t),
                "::",
                stringify!(m_pszString)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_iStringLength) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar_CVValue_t),
                "::",
                stringify!(m_iStringLength)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_fValue) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar_CVValue_t),
                "::",
                stringify!(m_fValue)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_nValue) as usize - ptr as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar_CVValue_t),
                "::",
                stringify!(m_nValue)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_ConVar() {
        const UNINIT: ::std::mem::MaybeUninit<ConVar> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<ConVar>(),
            136usize,
            concat!("Size of: ", stringify!(ConVar))
        );
        assert_eq!(
            ::std::mem::align_of::<ConVar>(),
            8usize,
            concat!("Alignment of ", stringify!(ConVar))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_ConCommandBase) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_ConCommandBase)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_pszDefaultValue) as usize - ptr as usize },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_pszDefaultValue)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_Value) as usize - ptr as usize },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_Value)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_bHasMin) as usize - ptr as usize },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_bHasMin)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_fMinVal) as usize - ptr as usize },
            100usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_fMinVal)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_bHasMax) as usize - ptr as usize },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_bHasMax)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_fMaxVal) as usize - ptr as usize },
            108usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_fMaxVal)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_pMalloc) as usize - ptr as usize },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_pMalloc)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).m_pPad80) as usize - ptr as usize },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(ConVar),
                "::",
                stringify!(m_pPad80)
            )
        );
    }
    extern "C" {
        #[link_name = "\u{1}?GetBaseName@ConVar@@QEBAPEBDXZ"]
        pub fn ConVar_GetBaseName(this: *const root::ConVar) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}?GetHelpText@ConVar@@QEBAPEBDXZ"]
        pub fn ConVar_GetHelpText(this: *const root::ConVar) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}?AddFlags@ConVar@@QEAAXH@Z"]
        pub fn ConVar_AddFlags(this: *mut root::ConVar, nFlags: ::std::os::raw::c_int);
    }
    extern "C" {
        #[link_name = "\u{1}?RemoveFlags@ConVar@@QEAAXH@Z"]
        pub fn ConVar_RemoveFlags(this: *mut root::ConVar, nFlags: ::std::os::raw::c_int);
    }
    extern "C" {
        #[link_name = "\u{1}?GetBool@ConVar@@QEBA_NXZ"]
        pub fn ConVar_GetBool(this: *const root::ConVar) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?GetFloat@ConVar@@QEBAMXZ"]
        pub fn ConVar_GetFloat(this: *const root::ConVar) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}?GetInt@ConVar@@QEBAHXZ"]
        pub fn ConVar_GetInt(this: *const root::ConVar) -> ::std::os::raw::c_int;
    }
    extern "C" {
        #[link_name = "\u{1}?GetColor@ConVar@@QEBA?AVColor@@XZ"]
        pub fn ConVar_GetColor(this: *const root::ConVar) -> root::Color;
    }
    extern "C" {
        #[link_name = "\u{1}?GetString@ConVar@@QEBAPEBDXZ"]
        pub fn ConVar_GetString(this: *const root::ConVar) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        #[link_name = "\u{1}?GetMin@ConVar@@QEBA_NAEAM@Z"]
        pub fn ConVar_GetMin(this: *const root::ConVar, flMinValue: *mut f32) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?GetMax@ConVar@@QEBA_NAEAM@Z"]
        pub fn ConVar_GetMax(this: *const root::ConVar, flMaxValue: *mut f32) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?GetMinValue@ConVar@@QEBAMXZ"]
        pub fn ConVar_GetMinValue(this: *const root::ConVar) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}?GetMaxValue@ConVar@@QEBAMXZ"]
        pub fn ConVar_GetMaxValue(this: *const root::ConVar) -> f32;
    }
    extern "C" {
        #[link_name = "\u{1}?HasMin@ConVar@@QEBA_NXZ"]
        pub fn ConVar_HasMin(this: *const root::ConVar) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?HasMax@ConVar@@QEBA_NXZ"]
        pub fn ConVar_HasMax(this: *const root::ConVar) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?SetValue@ConVar@@QEAAXH@Z"]
        pub fn ConVar_SetValue(this: *mut root::ConVar, nValue: ::std::os::raw::c_int);
    }
    extern "C" {
        #[link_name = "\u{1}?SetValue@ConVar@@QEAAXM@Z"]
        pub fn ConVar_SetValue1(this: *mut root::ConVar, flValue: f32);
    }
    extern "C" {
        #[link_name = "\u{1}?SetValue@ConVar@@QEAAXPEBD@Z"]
        pub fn ConVar_SetValue2(this: *mut root::ConVar, pszValue: *const ::std::os::raw::c_char);
    }
    extern "C" {
        #[link_name = "\u{1}?SetValue@ConVar@@QEAAXVColor@@@Z"]
        pub fn ConVar_SetValue3(this: *mut root::ConVar, clValue: root::Color);
    }
    extern "C" {
        #[link_name = "\u{1}?ChangeStringValue@ConVar@@QEAAXPEBDM@Z"]
        pub fn ConVar_ChangeStringValue(
            this: *mut root::ConVar,
            pszTempValue: *const ::std::os::raw::c_char,
            flOldValue: f32,
        );
    }
    extern "C" {
        #[link_name = "\u{1}?SetColorFromString@ConVar@@QEAA_NPEBD@Z"]
        pub fn ConVar_SetColorFromString(
            this: *mut root::ConVar,
            pszValue: *const ::std::os::raw::c_char,
        ) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?ClampValue@ConVar@@QEAA_NAEAM@Z"]
        pub fn ConVar_ClampValue(this: *mut root::ConVar, value: *mut f32) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?IsRegistered@ConVar@@QEBA_NXZ"]
        pub fn ConVar_IsRegistered(this: *const root::ConVar) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?IsCommand@ConVar@@QEBA_NXZ"]
        pub fn ConVar_IsCommand(this: *const root::ConVar) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}?IsFlagSet@ConVar@@QEBA_NH@Z"]
        pub fn ConVar_IsFlagSet(this: *const root::ConVar, nFlags: ::std::os::raw::c_int) -> bool;
    }
    extern "C" {
        #[link_name = "\u{1}??0ConVar@@QEAA@XZ"]
        pub fn ConVar_ConVar(this: *mut root::ConVar);
    }
    extern "C" {
        #[link_name = "\u{1}??0ConVar@@QEAA@PEBD0H0@Z"]
        pub fn ConVar_ConVar1(
            this: *mut root::ConVar,
            pszName: *const ::std::os::raw::c_char,
            pszDefaultValue: *const ::std::os::raw::c_char,
            nFlags: ::std::os::raw::c_int,
            pszHelpString: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        #[link_name = "\u{1}??0ConVar@@QEAA@PEBD0H0_NM1MP6AXPEAV0@0M@Z@Z"]
        pub fn ConVar_ConVar2(
            this: *mut root::ConVar,
            pszName: *const ::std::os::raw::c_char,
            pszDefaultValue: *const ::std::os::raw::c_char,
            nFlags: ::std::os::raw::c_int,
            pszHelpString: *const ::std::os::raw::c_char,
            bMin: bool,
            fMin: f32,
            bMax: bool,
            fMax: f32,
            pCallback: root::FnChangeCallback_t,
        );
    }
    extern "C" {
        #[link_name = "\u{1}??_DConVar@@QEAAXXZ"]
        pub fn ConVar_ConVar_destructor(this: *mut root::ConVar);
    }
    impl ConVar {
        #[inline]
        pub unsafe fn GetBaseName(&self) -> *const ::std::os::raw::c_char {
            ConVar_GetBaseName(self)
        }
        #[inline]
        pub unsafe fn GetHelpText(&self) -> *const ::std::os::raw::c_char {
            ConVar_GetHelpText(self)
        }
        #[inline]
        pub unsafe fn AddFlags(&mut self, nFlags: ::std::os::raw::c_int) {
            ConVar_AddFlags(self, nFlags)
        }
        #[inline]
        pub unsafe fn RemoveFlags(&mut self, nFlags: ::std::os::raw::c_int) {
            ConVar_RemoveFlags(self, nFlags)
        }
        #[inline]
        pub unsafe fn GetBool(&self) -> bool {
            ConVar_GetBool(self)
        }
        #[inline]
        pub unsafe fn GetFloat(&self) -> f32 {
            ConVar_GetFloat(self)
        }
        #[inline]
        pub unsafe fn GetInt(&self) -> ::std::os::raw::c_int {
            ConVar_GetInt(self)
        }
        #[inline]
        pub unsafe fn GetColor(&self) -> root::Color {
            ConVar_GetColor(self)
        }
        #[inline]
        pub unsafe fn GetString(&self) -> *const ::std::os::raw::c_char {
            ConVar_GetString(self)
        }
        #[inline]
        pub unsafe fn GetMin(&self, flMinValue: *mut f32) -> bool {
            ConVar_GetMin(self, flMinValue)
        }
        #[inline]
        pub unsafe fn GetMax(&self, flMaxValue: *mut f32) -> bool {
            ConVar_GetMax(self, flMaxValue)
        }
        #[inline]
        pub unsafe fn GetMinValue(&self) -> f32 {
            ConVar_GetMinValue(self)
        }
        #[inline]
        pub unsafe fn GetMaxValue(&self) -> f32 {
            ConVar_GetMaxValue(self)
        }
        #[inline]
        pub unsafe fn HasMin(&self) -> bool {
            ConVar_HasMin(self)
        }
        #[inline]
        pub unsafe fn HasMax(&self) -> bool {
            ConVar_HasMax(self)
        }
        #[inline]
        pub unsafe fn SetValue(&mut self, nValue: ::std::os::raw::c_int) {
            ConVar_SetValue(self, nValue)
        }
        #[inline]
        pub unsafe fn SetValue1(&mut self, flValue: f32) {
            ConVar_SetValue1(self, flValue)
        }
        #[inline]
        pub unsafe fn SetValue2(&mut self, pszValue: *const ::std::os::raw::c_char) {
            ConVar_SetValue2(self, pszValue)
        }
        #[inline]
        pub unsafe fn SetValue3(&mut self, clValue: root::Color) {
            ConVar_SetValue3(self, clValue)
        }
        #[inline]
        pub unsafe fn ChangeStringValue(
            &mut self,
            pszTempValue: *const ::std::os::raw::c_char,
            flOldValue: f32,
        ) {
            ConVar_ChangeStringValue(self, pszTempValue, flOldValue)
        }
        #[inline]
        pub unsafe fn SetColorFromString(
            &mut self,
            pszValue: *const ::std::os::raw::c_char,
        ) -> bool {
            ConVar_SetColorFromString(self, pszValue)
        }
        #[inline]
        pub unsafe fn ClampValue(&mut self, value: *mut f32) -> bool {
            ConVar_ClampValue(self, value)
        }
        #[inline]
        pub unsafe fn IsRegistered(&self) -> bool {
            ConVar_IsRegistered(self)
        }
        #[inline]
        pub unsafe fn IsCommand(&self) -> bool {
            ConVar_IsCommand(self)
        }
        #[inline]
        pub unsafe fn IsFlagSet(&self, nFlags: ::std::os::raw::c_int) -> bool {
            ConVar_IsFlagSet(self, nFlags)
        }
        #[inline]
        pub unsafe fn new() -> Self {
            let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
            ConVar_ConVar(__bindgen_tmp.as_mut_ptr());
            __bindgen_tmp.assume_init()
        }
        #[inline]
        pub unsafe fn new1(
            pszName: *const ::std::os::raw::c_char,
            pszDefaultValue: *const ::std::os::raw::c_char,
            nFlags: ::std::os::raw::c_int,
            pszHelpString: *const ::std::os::raw::c_char,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
            ConVar_ConVar1(
                __bindgen_tmp.as_mut_ptr(),
                pszName,
                pszDefaultValue,
                nFlags,
                pszHelpString,
            );
            __bindgen_tmp.assume_init()
        }
        #[inline]
        pub unsafe fn new2(
            pszName: *const ::std::os::raw::c_char,
            pszDefaultValue: *const ::std::os::raw::c_char,
            nFlags: ::std::os::raw::c_int,
            pszHelpString: *const ::std::os::raw::c_char,
            bMin: bool,
            fMin: f32,
            bMax: bool,
            fMax: f32,
            pCallback: root::FnChangeCallback_t,
        ) -> Self {
            let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
            ConVar_ConVar2(
                __bindgen_tmp.as_mut_ptr(),
                pszName,
                pszDefaultValue,
                nFlags,
                pszHelpString,
                bMin,
                fMin,
                bMax,
                fMax,
                pCallback,
            );
            __bindgen_tmp.assume_init()
        }
        #[inline]
        pub unsafe fn destruct(&mut self) {
            ConVar_ConVar_destructor(self)
        }
    }
    pub type ConVarRegisterType = ::std::option::Option<
        unsafe extern "C" fn(
            pConVar: *mut root::ConVar,
            pszName: *const ::std::os::raw::c_char,
            pszDefaultValue: *const ::std::os::raw::c_char,
            nFlags: ::std::os::raw::c_int,
            pszHelpString: *const ::std::os::raw::c_char,
            bMin: bool,
            fMin: f32,
            bMax: bool,
            fMax: f32,
            pCallback: *mut ::std::os::raw::c_void,
        ),
    >;
    extern "C" {
        #[link_name = "\u{1}?conVarRegister@@3P6AXPEAVConVar@@PEBD1H1_NM2MPEAX@ZEA"]
        pub static mut conVarRegister: root::ConVarRegisterType;
    }
    pub type ConVarMallocType = unsafe extern "C" fn(
        pConVarMaloc: *mut ::std::os::raw::c_void,
        a2: ::std::os::raw::c_int,
        a3: ::std::os::raw::c_int,
    );
    extern "C" {
        #[link_name = "\u{1}?conVarMalloc@@3P6AXPEAXHH@ZEA"]
        pub static mut conVarMalloc: root::ConVarMallocType;
    }
    extern "C" {
        #[link_name = "\u{1}?g_pConVar_Vtable@@3PEAXEA"]
        pub static mut g_pConVar_Vtable: *mut ::std::os::raw::c_void;
    }
    extern "C" {
        #[link_name = "\u{1}?g_pIConVar_Vtable@@3PEAXEA"]
        pub static mut g_pIConVar_Vtable: *mut ::std::os::raw::c_void;
    }
}
