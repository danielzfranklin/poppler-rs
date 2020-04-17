# poppler-sys

Low level bindings to [poppler](https://gitlab.freedesktop.org/poppler/poppler).

## Vendored Binding

The vendored bindings were generated on a system which had headers from:
- poppler-glib 0.87.0-1
- glib-2.0 2.64.2-1
- cairo 1.17.2+17+g52a7c79fd-2

Please continue reading if you intend to generate your own bindings and ignore the vendored ones.

## Documentation

To open the `poppler-sys` binding documentation:
```bash
$ cargo doc --no-deps --open --package poppler-sys
```

## Features

- `generate-bindings`
    If unset (default), the bindings from `src/vendored_bindings` will be used.  
    if set, new bindings will be generated into `OUTPUT_DIR` and they will be used as actual bindings. Also, they will be copied into (overwriting) `src/vendored_bindings`.
- `glib-api` (implicit, TODO)
- `qt5-api` (TODO)
- `cpp-api` (TODO)

- `dynamic-link` (implicit, TODO)
- `static-link` (TODO)

For now, only glib api is available; and `pkgconfig` emits the linking instructions for it (dynamic by default).

## Bindings Generation

Assuming the `generate-binding` is set, the bindings will be automatically generated according to the library `poppler-glib` (as searched by `pkgconfig`).  
The bindings are separated in various modules, so `bindgen` will be called multiple times - so there are a lot of discarded redudancy and so this process is slow (it takes ~5 minutes on my machine).  
Bindgen invokes `clang`; the `build.rs` script includes, into `clang`,  depedencies like `cairo` and `glib` that should result from the `pkgconfig` search.

Links to:
- `poppler-glib` (by `pkgconfig`, defaults to dynamic)

Build-depends on:
- `pkg-config`
- `poppler-glib` (and it's depedencies)
    - `cairo`
    - `glib-2.0`

After the new bindings have been generated, you should run the tests on both the `poppler` and `poppler-sys` packages, and also format the files on `poppler-sys/src/vendored_bindings/` (I use `rustfmt *<TAB>` to format all the files).
