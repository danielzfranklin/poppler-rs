#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "generate-bindings")]
extern crate bindgen;

extern crate pkg_config;
extern crate strum;

use std::collections::HashMap;

pub const POPPLER_GLIB_VERSION: &str = "0.87.0";
#[cfg(feature = "generate-bindings")]
const BINDINGS_VENDOR_DIR: &str = "src/vendored_bindings";

#[derive(Hash, Eq, PartialEq, Clone, Display)]
#[strum(serialize_all = "snake_case")]
enum Modules {
    Poppler,
    PopplerDocument,
    PopplerPage,
    PopplerAction,
    PopplerAnnot,
    PopplerAttachment,
    PopplerFormField,
    PopplerLayer,
    PopplerMedia,
    PopplerMovie,
}

// TODO: observe recomendations and standards from
// - https://kornel.ski/rust-sys-crate
// - https://rust-lang.github.io/rust-bindgen/introduction.html

#[cfg(feature = "generate-bindings")]
fn main() {
    // this println ensures a lazy_static execution
    // that will setup the linking
    println!("{:?}", POPPLER_LIBRARY.libs);

    gen(builder(), Modules::Poppler);
    gen(builder(), Modules::PopplerDocument);
    gen(builder(), Modules::PopplerPage);
    gen(builder(), Modules::PopplerAction);
    gen(builder(), Modules::PopplerAnnot);
    gen(builder(), Modules::PopplerAttachment);
    gen(builder(), Modules::PopplerFormField);
    gen(builder(), Modules::PopplerLayer);
    gen(builder(), Modules::PopplerMedia);
    gen(builder(), Modules::PopplerMovie);
}

#[cfg(not(feature = "generate-bindings"))]
fn main() {
    // this println ensures a lazy_static execution
    // that will setup the linking
    println!("{:?}", POPPLER_LIBRARY.libs);
}

#[cfg(feature = "generate-bindings")]
/// Initialize the builder with some include paths
fn builder() -> bindgen::Builder {
    let mut builder = bindgen::Builder::default();

    // have header depedencies be included into clang
    for incl in POPPLER_LIBRARY.include_paths.iter() {
        builder = builder.clang_args(&[
            "-I",
            incl.to_str()
                .expect(&format!("failed to stringfy {:?}", incl)),
        ]);
    }

    // have wrapping headers be included into clang
    // (the wrapping headers use files from poppler as a library,
    // already linked into rustc)
    builder
        // includes the wrapper headers
        .clang_args(&["-I", "build"])
        // extra options
        .whitelist_recursively(false)
        // TODO: also add more types and functions? (cairo, etc)
        .whitelist_type("_?Poppler.*")
        .whitelist_function("poppler_.*")
        .whitelist_var("_?Poppler.*")
        .whitelist_var("_?poppler.*")
        //
        .disable_name_namespacing()
        .generate_comments(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        //
        .raw_line("use crate::dep_types::*;")
        .raw_line("use crate::poppler::*;")
}

#[cfg(feature = "generate-bindings")]
/// Prevent re-defition of some types, generates and writes.
fn gen(mut builder: bindgen::Builder, module: Modules) {
    // enable/disable (re)definition of some types/functions
    use std::path::PathBuf;

    // types
    let (white_types, black_types): (Vec<(&Modules, &Vec<&str>)>, Vec<(&Modules, &Vec<&str>)>) =
        WHITELIST_TYPES.iter().partition(|(k, _v)| k == &&module);
    for ref ty in white_types.iter().cloned().flat_map(|(_k, v)| v) {
        builder = builder.whitelist_type(ty);
    }
    for ref ty in black_types.iter().cloned().flat_map(|(_k, v)| v) {
        builder = builder.blacklist_type(ty);
    }

    // functions
    let (white_fns, black_fns): (Vec<(&Modules, &Vec<&str>)>, Vec<(&Modules, &Vec<&str>)>) =
        WHITELIST_FUNCTIONS
            .iter()
            .partition(|(k, _v)| k == &&module);
    for ty in white_fns.iter().cloned().flat_map(|(_k, v)| v) {
        builder = builder.whitelist_function(ty);
    }
    for ty in black_fns.iter().cloned().flat_map(|(_k, v)| v) {
        builder = builder.blacklist_function(ty);
    }

    // final builder configuration and generation
    let binding = builder
        .header(format!("build/header_wrappers/{}_wrp.h", module))
        .generate()
        .expect(&format!("Unable to generate bindings for {}", module));

    let file_name = format!("bindings_{}.rs", module);

    // writing of the bindings into OUT_DIR
    let out_dir = std::env::var("OUT_DIR").expect("Missing OUT_DIR env var");
    let out_path = PathBuf::from(&out_dir).join(&file_name);
    binding
        .write_to_file(out_path)
        .expect(&format!("Couldn't write bindings for {}.", module));

    // also writes it into the binding vendoring directory
    let vend_dir = BINDINGS_VENDOR_DIR;
    let vend_path = PathBuf::from(&vend_dir).join(file_name);

    binding
        .write_to_file(vend_path)
        .expect(&format!("Couldn't write bindings for {}.", module));
}

lazy_static! {
    static ref POPPLER_LIBRARY: pkg_config::Library = pkg_config::Config::new()
        // emits link bindings to poppler-glib
        .cargo_metadata(true)
        .atleast_version(POPPLER_GLIB_VERSION)
        .probe("poppler-glib")
        .expect("pkg-config could not find poppler");

    static ref WHITELIST_TYPES: HashMap<Modules, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(
            Modules::Poppler,
            WHITELIST_POPPLER.iter().cloned().collect(),
        );
        m.insert(
            Modules::PopplerDocument,
            WHITELIST_POPPLER_DOCUMENT.to_vec(),
        );
        m.insert(
            Modules::PopplerPage,
            WHITELIST_POPPLER_PAGE.to_vec(),
        );
        m.insert(
            Modules::PopplerAction,
            WHITELIST_POPPLER_ACTION.to_vec(),
        );
        m.insert(
            Modules::PopplerAnnot,
            WHITELIST_POPPLER_ANNOT.to_vec(),
        );
        m.insert(
            Modules::PopplerAttachment,
            WHITELIST_POPPLER_ATTACHMENT.to_vec(),
        );
        m.insert(
            Modules::PopplerFormField,
            WHITELIST_POPPLER_FORM_FIELD.to_vec(),
        );
        m.insert(
            Modules::PopplerLayer,
            WHITELIST_POPPLER_LAYER.to_vec(),
        );
        m.insert(
            Modules::PopplerMedia,
            WHITELIST_POPPLER_MEDIA.to_vec(),
        );
        m.insert(
            Modules::PopplerMovie,
            WHITELIST_POPPLER_MOVIE.to_vec(),
        );
        m
    };
    static ref WHITELIST_FUNCTIONS: HashMap<Modules, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(
            Modules::Poppler,
            WHITELIST_FUNC_POPPLER.to_vec(),
        );
        m
    };
}

const WHITELIST_FUNC_POPPLER: [&str; 3] = [
    "poppler_error_quark",
    "poppler_get_backend",
    "poppler_get_version",
];
const WHITELIST_POPPLER: [&str; 73] = [
    "guint16",
    //
    "PopplerError",
    "PopplerPageTransitionType",
    "PopplerPageTransitionAlignment",
    "PopplerPageTransitionDirection",
    "PopplerSelectionStyle",
    "PopplerPrintFlags",
    "PopplerFindFlags",
    "PopplerBackend",
    // poppler-private.h
    "_PopplerDocument",
    "PopplerDocument",
    "_PopplerPage",
    "PopplerPage",
    "_PopplerFontInfo",
    "PopplerFontInfo",
    "_PopplerLayer",
    "PopplerLayer",
    "_PopplerPSFile",
    "PopplerPSFile",
    "_PopplerFormField",
    "PopplerFormField",
    "_PopplerAnnot",
    "PopplerAnnot",
    "_PopplerStructureElement",
    "PopplerStructureElement",
    // poppler-document.cc
    "_PopplerIndexIter",
    "PopplerIndexIter",
    "_PopplerFontsIter",
    "PopplerFontsIter",
    "_PopplerLayersIter",
    "PopplerLayersIter",
    // poppler-page.h
    "PopplerPoint",
    "PopplerRectangle",
    "PopplerTextAttributes",
    "PopplerColor",
    "PopplerLinkMapping",
    "PopplerPageTransition",
    "PopplerImageMapping",
    "PopplerFormFieldMapping",
    "PopplerAnnotMapping",
    "PopplerQuadrilateral",
    // poppler-action.h
    "PopplerAction",
    "PopplerDest",
    "PopplerActionLayer",
    // poppler-attachment.h
    "PopplerAttachment",
    // poppler-movie.h
    "PopplerMovie",
    // poppler-media.cc
    "_PopplerMedia",
    "PopplerMedia",
    // poppler-annot.cc
    "_PopplerAnnotMarkup",
    "PopplerAnnotMarkup",
    "_PopplerAnnotText",
    "PopplerAnnotText",
    "_PopplerAnnotTextMarkup",
    "PopplerAnnotTextMarkup",
    "_PopplerAnnotFreeText",
    "PopplerAnnotFreeText",
    "_PopplerAnnotFileAttachment",
    "PopplerAnnotFileAttachment",
    "_PopplerAnnotMovie",
    "PopplerAnnotMovie",
    "_PopplerAnnotScreen",
    "PopplerAnnotScreen",
    "_PopplerAnnotLine",
    "PopplerAnnotLine",
    "_PopplerAnnotCircle",
    "PopplerAnnotCircle",
    "_PopplerAnnotSquare",
    "PopplerAnnotSquare",
    // poppler-annot.h
    "PopplerAnnotCalloutLine",
    // poppler-structure-element.cc
    "_PopplerStructureElementIter",
    "PopplerStructureElementIter",
    "_PopplerTextSpan",
    "PopplerTextSpan",
];
const WHITELIST_POPPLER_DOCUMENT: [&str; 4] = [
    "goffset",
    "gint64",
    //
    "_PopplerPageRange",
    "PopplerPageRange",
];
const WHITELIST_POPPLER_PAGE: [&str; 10] = [
    "_PopplerPoint",
    "_PopplerRectangle",
    "_PopplerTextAttributes",
    "_PopplerColor",
    "_PopplerLinkMapping",
    "_PopplerPageTransition",
    "_PopplerImageMapping",
    "_PopplerFormFieldMapping",
    "_PopplerAnnotMapping",
    "_PopplerQuadrilateral",
];
const WHITELIST_POPPLER_ACTION: [&str; 4] = [
    "guint8",
    "_PopplerAction",
    "_PopplerDest",
    "_PopplerActionLayer",
];
const WHITELIST_POPPLER_ANNOT: [&str; 1] = ["_PopplerAnnotCalloutLine"];
const WHITELIST_POPPLER_ATTACHMENT: [&str; 1] = ["_PopplerAttachment"];
const WHITELIST_POPPLER_FORM_FIELD: [&str; 0] = [
    // empty
];
const WHITELIST_POPPLER_LAYER: [&str; 0] = [
    // empty
];
const WHITELIST_POPPLER_MEDIA: [&str; 0] = [
    // empty
];
const WHITELIST_POPPLER_MOVIE: [&str; 2] = [
    "guint64",
    //
    "_PopplerMovie",
];
