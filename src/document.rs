use crate::util;
use crate::PopplerPage;
use poppler_sys::{poppler as sys, poppler_document as sys_doc};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::path;
use cairo::glib;

#[derive(Debug)]
pub struct PopplerDocument(*mut sys::PopplerDocument);

impl PopplerDocument {
    #[doc(alias = "poppler_document_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file<P: AsRef<path::Path>>(
        p: P,
        password: &str,
    ) -> Result<PopplerDocument, glib::error::Error> {
        let pw = CString::new(password).map_err(|_| {
            glib::error::Error::new(
                glib::FileError::Inval,
                "Password invalid (possibly contains NUL characters)",
            )
        })?;

        let path_cstring = util::path_to_glib_url(p)?;
        let doc = util::call_with_gerror(|err_ptr| unsafe {
            sys_doc::poppler_document_new_from_file(path_cstring.as_ptr(), pw.as_ptr(), err_ptr)
        })?;

        Ok(PopplerDocument(doc))
    }

    #[doc(alias = "poppler_document_new_from_data")]
    #[doc(alias = "new_from_data")]
    #[deprecated(note = "[`from_data`] has been deprecated since version 0.82 and should not be used in newly-written code. This requires directly managing length and data . Use [`from_bytes`] instead.", since = "Poppler v0.82")]
    pub fn from_data(
        data: &mut [u8],
        password: &str,
    ) -> Result<PopplerDocument, glib::error::Error> {
        if data.is_empty() {
            return Err(glib::error::Error::new(
                glib::FileError::Inval,
                "data is empty",
            ));
        }
        let pw = CString::new(password).map_err(|_| {
            glib::error::Error::new(
                glib::FileError::Inval,
                "Password invalid (possibly contains NUL characters)",
            )
        })?;

        let doc = util::call_with_gerror(|err_ptr| unsafe {
            sys_doc::poppler_document_new_from_data(
                data.as_mut_ptr() as *mut c_char,
                data.len() as c_int,
                pw.as_ptr(),
                err_ptr,
            )
        })?;

        Ok(PopplerDocument(doc))
    }

    #[doc(alias = "poppler_document_new_from_bytes")]
    #[doc(alias = "new_from_bytes")]
    pub fn from_bytes(
        bytes: glib::Bytes,
        password: &str,
    ) -> Result<PopplerDocument, glib::error::Error> {
        let pw = CString::new(password).map_err(|_| {
            glib::error::Error::new(
                glib::FileError::Inval,
                "Password invalid (possibly contains NUL characters)",
            )
        })?;
        use glib::translate::ToGlibPtr;
        let doc = util::call_with_gerror(|err_ptr| unsafe {
            sys_doc::poppler_document_new_from_bytes(
                bytes.to_glib_full(),
                pw.as_ptr(),
                err_ptr,
            )
        })?;

        Ok(PopplerDocument(doc))
    }

    #[doc(alias = "poppler_document_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = sys_doc::poppler_document_get_title(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
    }

    #[doc(alias = "poppler_document_get_metadata")]
    #[doc(alias = "get_metadata")]
    pub fn metadata(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = sys_doc::poppler_document_get_metadata(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
    }

    #[doc(alias = "poppler_document_get_pdf_version_string")]
    #[doc(alias = "get_pdf_version_string")]
    pub fn pdf_version_string(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = sys_doc::poppler_document_get_pdf_version_string(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
    }

    #[doc(alias = "poppler_document_get_permissions")]
    #[doc(alias = "get_permissions")]
    pub fn permissions(&self) -> u8 {
        unsafe { sys_doc::poppler_document_get_permissions(self.0) as u8 }
    }

    #[doc(alias = "poppler_document_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    pub fn n_pages(&self) -> usize {
        // FIXME: what's the correct type here? can we assume a document
        //        has a positive number of pages?
        (unsafe { sys_doc::poppler_document_get_n_pages(self.0) }) as usize
    }

    #[doc(alias = "poppler_document_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self, index: usize) -> Option<PopplerPage> {
        match unsafe { sys_doc::poppler_document_get_page(self.0, index as c_int) } {
            ptr if ptr.is_null() => None,
            ptr => Some(PopplerPage::new(ptr)),
        }
    }
}

// TODO replace Box<dyn FnMut> with an opaque type once we have existential types
impl <'a> std::iter::IntoIterator for &'a PopplerDocument {
    type Item = PopplerPage;
    type IntoIter = std::iter::Map<std::ops::Range<usize>, Box<dyn FnMut(usize) -> Self::Item + 'a>>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.n_pages()).map(Box::new(move |page| self.page(page).expect("Poppler internal error: PDF is missing a page?!")))
    }
}
