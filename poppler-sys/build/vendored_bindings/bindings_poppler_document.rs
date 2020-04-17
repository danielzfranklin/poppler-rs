/* automatically generated by rust-bindgen */

pub type gint64 = ::std::os::raw::c_long;
pub type goffset = gint64;
pub type PopplerPageRange = _PopplerPageRange;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_UNSET: PopplerPageLayout = 0;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_SINGLE_PAGE: PopplerPageLayout = 1;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_ONE_COLUMN: PopplerPageLayout = 2;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_COLUMN_LEFT: PopplerPageLayout = 3;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_COLUMN_RIGHT: PopplerPageLayout = 4;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_PAGE_LEFT: PopplerPageLayout = 5;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_PAGE_RIGHT: PopplerPageLayout = 6;
pub type PopplerPageLayout = u32;
pub const PopplerPageMode_POPPLER_PAGE_MODE_UNSET: PopplerPageMode = 0;
pub const PopplerPageMode_POPPLER_PAGE_MODE_NONE: PopplerPageMode = 1;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_OUTLINES: PopplerPageMode = 2;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_THUMBS: PopplerPageMode = 3;
pub const PopplerPageMode_POPPLER_PAGE_MODE_FULL_SCREEN: PopplerPageMode = 4;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_OC: PopplerPageMode = 5;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_ATTACHMENTS: PopplerPageMode = 6;
pub type PopplerPageMode = u32;
pub const PopplerFontType_POPPLER_FONT_TYPE_UNKNOWN: PopplerFontType = 0;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE1: PopplerFontType = 1;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE1C: PopplerFontType = 2;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE1COT: PopplerFontType = 3;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE3: PopplerFontType = 4;
pub const PopplerFontType_POPPLER_FONT_TYPE_TRUETYPE: PopplerFontType = 5;
pub const PopplerFontType_POPPLER_FONT_TYPE_TRUETYPEOT: PopplerFontType = 6;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE0: PopplerFontType = 7;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE0C: PopplerFontType = 8;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE0COT: PopplerFontType = 9;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE2: PopplerFontType = 10;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE2OT: PopplerFontType = 11;
pub type PopplerFontType = u32;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_UNSET: PopplerViewerPreferences = 0;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_HIDE_TOOLBAR:
    PopplerViewerPreferences = 1;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_HIDE_MENUBAR:
    PopplerViewerPreferences = 2;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_HIDE_WINDOWUI:
    PopplerViewerPreferences = 4;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_FIT_WINDOW: PopplerViewerPreferences =
    8;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_CENTER_WINDOW:
    PopplerViewerPreferences = 16;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_DISPLAY_DOC_TITLE:
    PopplerViewerPreferences = 32;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_DIRECTION_RTL:
    PopplerViewerPreferences = 64;
pub type PopplerViewerPreferences = u32;
pub const PopplerPrintScaling_POPPLER_PRINT_SCALING_APP_DEFAULT: PopplerPrintScaling = 0;
pub const PopplerPrintScaling_POPPLER_PRINT_SCALING_NONE: PopplerPrintScaling = 1;
pub type PopplerPrintScaling = u32;
pub const PopplerPrintDuplex_POPPLER_PRINT_DUPLEX_NONE: PopplerPrintDuplex = 0;
pub const PopplerPrintDuplex_POPPLER_PRINT_DUPLEX_SIMPLEX: PopplerPrintDuplex = 1;
pub const PopplerPrintDuplex_POPPLER_PRINT_DUPLEX_DUPLEX_FLIP_SHORT_EDGE: PopplerPrintDuplex = 2;
pub const PopplerPrintDuplex_POPPLER_PRINT_DUPLEX_DUPLEX_FLIP_LONG_EDGE: PopplerPrintDuplex = 3;
pub type PopplerPrintDuplex = u32;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_PRINT: PopplerPermissions = 1;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_MODIFY: PopplerPermissions = 2;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_COPY: PopplerPermissions = 4;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_ADD_NOTES: PopplerPermissions = 8;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_FILL_FORM: PopplerPermissions = 16;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_EXTRACT_CONTENTS: PopplerPermissions = 32;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_ASSEMBLE: PopplerPermissions = 64;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_PRINT_HIGH_RESOLUTION: PopplerPermissions =
    128;
pub const PopplerPermissions_POPPLER_PERMISSIONS_FULL: PopplerPermissions = 255;
pub type PopplerPermissions = u32;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_UNSET: PopplerPDFSubtype = 0;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_A: PopplerPDFSubtype = 1;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_E: PopplerPDFSubtype = 2;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_UA: PopplerPDFSubtype = 3;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_VT: PopplerPDFSubtype = 4;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_X: PopplerPDFSubtype = 5;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_NONE: PopplerPDFSubtype = 6;
pub type PopplerPDFSubtype = u32;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_UNSET: PopplerPDFPart = 0;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_1: PopplerPDFPart = 1;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_2: PopplerPDFPart = 2;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_3: PopplerPDFPart = 3;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_4: PopplerPDFPart = 4;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_5: PopplerPDFPart = 5;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_6: PopplerPDFPart = 6;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_7: PopplerPDFPart = 7;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_8: PopplerPDFPart = 8;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_NONE: PopplerPDFPart = 9;
pub type PopplerPDFPart = u32;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_UNSET: PopplerPDFConformance = 0;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_A: PopplerPDFConformance = 1;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_B: PopplerPDFConformance = 2;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_G: PopplerPDFConformance = 3;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_N: PopplerPDFConformance = 4;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_P: PopplerPDFConformance = 5;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_PG: PopplerPDFConformance = 6;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_U: PopplerPDFConformance = 7;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_NONE: PopplerPDFConformance = 8;
pub type PopplerPDFConformance = u32;
extern "C" {
    pub fn poppler_document_get_type() -> GType;
}
extern "C" {
    pub fn poppler_document_new_from_file(
        uri: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
}
extern "C" {
    pub fn poppler_document_new_from_data(
        data: *mut ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
        password: *const ::std::os::raw::c_char,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
}
extern "C" {
    pub fn poppler_document_new_from_bytes(
        bytes: *mut GBytes,
        password: *const ::std::os::raw::c_char,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
}
extern "C" {
    pub fn poppler_document_new_from_stream(
        stream: *mut GInputStream,
        length: goffset,
        password: *const ::std::os::raw::c_char,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
}
extern "C" {
    pub fn poppler_document_new_from_gfile(
        file: *mut GFile,
        password: *const ::std::os::raw::c_char,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
}
extern "C" {
    pub fn poppler_document_save(
        document: *mut PopplerDocument,
        uri: *const ::std::os::raw::c_char,
        error: *mut *mut GError,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_document_save_a_copy(
        document: *mut PopplerDocument,
        uri: *const ::std::os::raw::c_char,
        error: *mut *mut GError,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_document_get_id(
        document: *mut PopplerDocument,
        permanent_id: *mut *mut gchar,
        update_id: *mut *mut gchar,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_document_get_n_pages(document: *mut PopplerDocument) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn poppler_document_get_page(
        document: *mut PopplerDocument,
        index: ::std::os::raw::c_int,
    ) -> *mut PopplerPage;
}
extern "C" {
    pub fn poppler_document_get_page_by_label(
        document: *mut PopplerDocument,
        label: *const ::std::os::raw::c_char,
    ) -> *mut PopplerPage;
}
extern "C" {
    pub fn poppler_document_get_pdf_version_string(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_get_pdf_version(
        document: *mut PopplerDocument,
        major_version: *mut guint,
        minor_version: *mut guint,
    );
}
extern "C" {
    pub fn poppler_document_get_title(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_set_title(document: *mut PopplerDocument, title: *const gchar);
}
extern "C" {
    pub fn poppler_document_get_author(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_set_author(document: *mut PopplerDocument, author: *const gchar);
}
extern "C" {
    pub fn poppler_document_get_subject(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_set_subject(document: *mut PopplerDocument, subject: *const gchar);
}
extern "C" {
    pub fn poppler_document_get_keywords(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_set_keywords(document: *mut PopplerDocument, keywords: *const gchar);
}
extern "C" {
    pub fn poppler_document_get_creator(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_set_creator(document: *mut PopplerDocument, creator: *const gchar);
}
extern "C" {
    pub fn poppler_document_get_producer(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_set_producer(document: *mut PopplerDocument, producer: *const gchar);
}
extern "C" {
    pub fn poppler_document_get_creation_date(document: *mut PopplerDocument) -> time_t;
}
extern "C" {
    pub fn poppler_document_set_creation_date(
        document: *mut PopplerDocument,
        creation_date: time_t,
    );
}
extern "C" {
    pub fn poppler_document_get_modification_date(document: *mut PopplerDocument) -> time_t;
}
extern "C" {
    pub fn poppler_document_set_modification_date(
        document: *mut PopplerDocument,
        modification_date: time_t,
    );
}
extern "C" {
    pub fn poppler_document_is_linearized(document: *mut PopplerDocument) -> gboolean;
}
extern "C" {
    pub fn poppler_document_get_page_layout(document: *mut PopplerDocument) -> PopplerPageLayout;
}
extern "C" {
    pub fn poppler_document_get_page_mode(document: *mut PopplerDocument) -> PopplerPageMode;
}
extern "C" {
    pub fn poppler_document_get_permissions(document: *mut PopplerDocument) -> PopplerPermissions;
}
extern "C" {
    pub fn poppler_document_get_pdf_subtype_string(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_get_pdf_subtype(document: *mut PopplerDocument) -> PopplerPDFSubtype;
}
extern "C" {
    pub fn poppler_document_get_pdf_part(document: *mut PopplerDocument) -> PopplerPDFPart;
}
extern "C" {
    pub fn poppler_document_get_pdf_conformance(
        document: *mut PopplerDocument,
    ) -> PopplerPDFConformance;
}
extern "C" {
    pub fn poppler_document_get_metadata(document: *mut PopplerDocument) -> *mut gchar;
}
extern "C" {
    pub fn poppler_document_get_print_scaling(
        document: *mut PopplerDocument,
    ) -> PopplerPrintScaling;
}
extern "C" {
    pub fn poppler_document_get_print_duplex(document: *mut PopplerDocument) -> PopplerPrintDuplex;
}
extern "C" {
    pub fn poppler_document_get_print_n_copies(document: *mut PopplerDocument) -> gint;
}
extern "C" {
    pub fn poppler_document_get_print_page_ranges(
        document: *mut PopplerDocument,
        n_ranges: *mut ::std::os::raw::c_int,
    ) -> *mut PopplerPageRange;
}
extern "C" {
    pub fn poppler_document_get_n_attachments(document: *mut PopplerDocument) -> guint;
}
extern "C" {
    pub fn poppler_document_has_attachments(document: *mut PopplerDocument) -> gboolean;
}
extern "C" {
    pub fn poppler_document_get_attachments(document: *mut PopplerDocument) -> *mut GList;
}
extern "C" {
    pub fn poppler_document_find_dest(
        document: *mut PopplerDocument,
        link_name: *const gchar,
    ) -> *mut PopplerDest;
}
extern "C" {
    pub fn poppler_document_create_dests_tree(document: *mut PopplerDocument) -> *mut GTree;
}
extern "C" {
    pub fn poppler_document_get_form_field(
        document: *mut PopplerDocument,
        id: gint,
    ) -> *mut PopplerFormField;
}
extern "C" {
    pub fn poppler_index_iter_get_type() -> GType;
}
extern "C" {
    pub fn poppler_index_iter_new(document: *mut PopplerDocument) -> *mut PopplerIndexIter;
}
extern "C" {
    pub fn poppler_index_iter_copy(iter: *mut PopplerIndexIter) -> *mut PopplerIndexIter;
}
extern "C" {
    pub fn poppler_index_iter_free(iter: *mut PopplerIndexIter);
}
extern "C" {
    pub fn poppler_index_iter_get_child(parent: *mut PopplerIndexIter) -> *mut PopplerIndexIter;
}
extern "C" {
    pub fn poppler_index_iter_is_open(iter: *mut PopplerIndexIter) -> gboolean;
}
extern "C" {
    pub fn poppler_index_iter_get_action(iter: *mut PopplerIndexIter) -> *mut PopplerAction;
}
extern "C" {
    pub fn poppler_index_iter_next(iter: *mut PopplerIndexIter) -> gboolean;
}
extern "C" {
    pub fn poppler_font_info_get_type() -> GType;
}
extern "C" {
    pub fn poppler_font_info_new(document: *mut PopplerDocument) -> *mut PopplerFontInfo;
}
extern "C" {
    pub fn poppler_font_info_scan(
        font_info: *mut PopplerFontInfo,
        n_pages: ::std::os::raw::c_int,
        iter: *mut *mut PopplerFontsIter,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_font_info_free(font_info: *mut PopplerFontInfo);
}
extern "C" {
    pub fn poppler_fonts_iter_get_type() -> GType;
}
extern "C" {
    pub fn poppler_fonts_iter_copy(iter: *mut PopplerFontsIter) -> *mut PopplerFontsIter;
}
extern "C" {
    pub fn poppler_fonts_iter_free(iter: *mut PopplerFontsIter);
}
extern "C" {
    pub fn poppler_fonts_iter_get_name(
        iter: *mut PopplerFontsIter,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_fonts_iter_get_full_name(
        iter: *mut PopplerFontsIter,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_fonts_iter_get_substitute_name(
        iter: *mut PopplerFontsIter,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_fonts_iter_get_file_name(
        iter: *mut PopplerFontsIter,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_fonts_iter_get_font_type(iter: *mut PopplerFontsIter) -> PopplerFontType;
}
extern "C" {
    pub fn poppler_fonts_iter_get_encoding(
        iter: *mut PopplerFontsIter,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_fonts_iter_is_embedded(iter: *mut PopplerFontsIter) -> gboolean;
}
extern "C" {
    pub fn poppler_fonts_iter_is_subset(iter: *mut PopplerFontsIter) -> gboolean;
}
extern "C" {
    pub fn poppler_fonts_iter_next(iter: *mut PopplerFontsIter) -> gboolean;
}
extern "C" {
    pub fn poppler_layers_iter_get_type() -> GType;
}
extern "C" {
    pub fn poppler_layers_iter_new(document: *mut PopplerDocument) -> *mut PopplerLayersIter;
}
extern "C" {
    pub fn poppler_layers_iter_copy(iter: *mut PopplerLayersIter) -> *mut PopplerLayersIter;
}
extern "C" {
    pub fn poppler_layers_iter_free(iter: *mut PopplerLayersIter);
}
extern "C" {
    pub fn poppler_layers_iter_get_child(parent: *mut PopplerLayersIter) -> *mut PopplerLayersIter;
}
extern "C" {
    pub fn poppler_layers_iter_get_title(iter: *mut PopplerLayersIter) -> *mut gchar;
}
extern "C" {
    pub fn poppler_layers_iter_get_layer(iter: *mut PopplerLayersIter) -> *mut PopplerLayer;
}
extern "C" {
    pub fn poppler_layers_iter_next(iter: *mut PopplerLayersIter) -> gboolean;
}
extern "C" {
    pub fn poppler_ps_file_get_type() -> GType;
}
extern "C" {
    pub fn poppler_ps_file_new(
        document: *mut PopplerDocument,
        filename: *const ::std::os::raw::c_char,
        first_page: ::std::os::raw::c_int,
        n_pages: ::std::os::raw::c_int,
    ) -> *mut PopplerPSFile;
}
extern "C" {
    pub fn poppler_ps_file_set_paper_size(ps_file: *mut PopplerPSFile, width: f64, height: f64);
}
extern "C" {
    pub fn poppler_ps_file_set_duplex(ps_file: *mut PopplerPSFile, duplex: gboolean);
}
extern "C" {
    pub fn poppler_ps_file_free(ps_file: *mut PopplerPSFile);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerPageRange {
    pub start_page: gint,
    pub end_page: gint,
}
#[test]
fn bindgen_test_layout__PopplerPageRange() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerPageRange>(),
        8usize,
        concat!("Size of: ", stringify!(_PopplerPageRange))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerPageRange>(),
        4usize,
        concat!("Alignment of ", stringify!(_PopplerPageRange))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPageRange>())).start_page as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageRange),
            "::",
            stringify!(start_page)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerPageRange>())).end_page as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerPageRange),
            "::",
            stringify!(end_page)
        )
    );
}
