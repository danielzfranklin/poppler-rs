#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes
)]

pub(crate) mod vendored_bindings;

#[warn(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes
)]
mod dep_types {
    pub use cairo_sys::{cairo_region_t, cairo_surface_t, cairo_t};
    pub use glib_sys::{
        gboolean, gpointer, GArray, GBytes, GDate, GError, GList, GQuark, GString, GTime, GTree,
        GType,
    };
    pub use gobject_sys::{GObject, GObjectClass};
    pub use gtypes::{gchar, gdouble, gint, gsize, guint, gushort};
    pub use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};
    #[allow(non_camel_case_types)]
    pub type time_t = c_long;
    pub use gio_sys::{GCancellable, GFile, GInputStream};
}

pub mod poppler {
    pub(crate) use super::action::{_PopplerAction, _PopplerActionLayer, _PopplerDest};
    pub(crate) use super::annot::_PopplerAnnotCalloutLine;
    pub(crate) use super::attachment::_PopplerAttachment;
    pub(crate) use super::movie::_PopplerMovie;
    pub(crate) use super::page::{
        _PopplerAnnotMapping, _PopplerColor, _PopplerFormFieldMapping, _PopplerImageMapping,
        _PopplerLinkMapping, _PopplerPageTransition, _PopplerPoint, _PopplerQuadrilateral,
        _PopplerRectangle, _PopplerTextAttributes,
    };
    pub use crate::vendored_bindings::bindings_poppler::*;
}

pub use vendored_bindings::bindings_poppler_action as action;
pub use vendored_bindings::bindings_poppler_annot as annot;
pub use vendored_bindings::bindings_poppler_attachment as attachment;
pub use vendored_bindings::bindings_poppler_document as document;
pub use vendored_bindings::bindings_poppler_form_field as form_field;
pub use vendored_bindings::bindings_poppler_layer as layer;
pub use vendored_bindings::bindings_poppler_media as media;
pub use vendored_bindings::bindings_poppler_movie as movie;
pub use vendored_bindings::bindings_poppler_page as page;
