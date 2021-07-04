use poppler_sys::{poppler as sys, poppler_page as sys_pg};
use std::ffi::CStr;
use std::os::raw::c_double;

#[derive(Debug)]
pub struct PopplerPage(*mut sys::PopplerPage);

impl PopplerPage {
    #[doc(hidden)]
    pub fn new(ptr: *mut sys::PopplerPage) -> Self {
        PopplerPage(ptr)
    }

    #[doc(alias = "poppler_page_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> (f64, f64) {
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        unsafe {
            sys_pg::poppler_page_get_size(
                self.0,
                &mut width as *mut f64 as *mut c_double,
                &mut height as *mut f64 as *mut c_double,
            )
        }

        (width, height)
    }

    #[doc(alias = "poppler_page_render")]
    pub fn render(&self, ctx: &cairo::Context) -> Result<(), cairo::Error> {
        let ctx_raw = ctx.to_raw_none();
        unsafe { sys_pg::poppler_page_render(self.0, ctx_raw) };
        ctx.status()
    }

    #[doc(alias = "poppler_page_render_for_printing")]
    pub fn render_for_printing(&self, ctx: &cairo::Context) -> Result<(), cairo::Error> {
        let ctx_raw = ctx.to_raw_none();
        unsafe { sys_pg::poppler_page_render_for_printing(self.0, ctx_raw) };
        ctx.status()
    }

    #[doc(alias = "poppler_page_get_text")]
    #[doc(alias = "get_text")]
    pub fn text(&self) -> Option<&str> {
        match unsafe { sys_pg::poppler_page_get_text(self.0) } {
            ptr if ptr.is_null() => None,
            ptr => unsafe { Some(CStr::from_ptr(ptr).to_str().unwrap()) },
        }
    }
}
