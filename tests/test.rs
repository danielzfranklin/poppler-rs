use cairo::{Context, Format, ImageSurface, PdfSurface};
use poppler::{PopplerDocument, PopplerPage};
use std::{fs::File, io::Read};

#[test]
fn test1() -> Result<(), cairo::Error> {
    let filename = "tests/test.pdf";
    let doc = PopplerDocument::from_file(filename, "").unwrap();
    let num_pages = doc.n_pages();

    println!("Document has {} page(s)", num_pages);

    let mut surface = PdfSurface::new(420.0, 595.0, "tests/output.pdf").unwrap();
    let ctx = Context::new(&mut surface)?;

    for (page_num, page) in doc.into_iter().enumerate() {
        let (w, h) = page.size();
        println!("page {} has size {}, {}", page_num, w, h);
        surface.set_size(w, h)?;

        ctx.save()?;
        page.render(&ctx)?;

        println!("Text: {:?}", page.text().unwrap_or(""));

        ctx.restore()?;
        ctx.show_page()?;
    }
    // g_object_unref (page);
    //surface.write_to_png("file.png");
    surface.finish();
    Ok(())
}

#[test]
fn test2_from_file() -> Result<(), cairo::Error> {
    let path = "tests/test.pdf";
    let doc: PopplerDocument = PopplerDocument::from_file(path, "upw").unwrap();
    let num_pages = doc.n_pages();
    let title = doc.title().unwrap();
    let metadata = doc.metadata();
    let version_string = doc.pdf_version_string();
    let permissions = doc.permissions();
    let page: PopplerPage = doc.page(0).unwrap();
    let (w, h) = page.size();

    println!(
        "Document {} has {} page(s) and is {}x{}",
        title, num_pages, w, h
    );
    println!(
        "Version: {:?}, Permissions: {:x?}",
        version_string, permissions
    );

    assert!(metadata.is_some());
    assert_eq!(version_string, Some("PDF-1.3".to_string()));
    assert_eq!(permissions, 0xff);

    assert_eq!(title, "This is a test PDF file");

    let mut surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32).unwrap();
    let ctx = Context::new(&mut surface)?;

    ctx.save()?;
    page.render(&ctx)?;
    ctx.restore()?;
    ctx.show_page()?;

    let mut f: File = File::create("tests/out.png").unwrap();
    surface.write_to_png(&mut f).expect("Unable to write PNG");
    Ok(())
}

#[test]
fn test2_from_data() {
    let path = "tests/test.pdf";
    let mut file = File::open(path).unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let doc: PopplerDocument = PopplerDocument::from_data(&mut data[..], "upw").unwrap();
    let num_pages = doc.n_pages();
    let title = doc.title().unwrap();
    let metadata = doc.metadata();
    let version_string = doc.pdf_version_string();
    let permissions = doc.permissions();
    let page: PopplerPage = doc.page(0).unwrap();
    let (w, h) = page.size();

    println!(
        "Document {} has {} page(s) and is {}x{}",
        title, num_pages, w, h
    );
    println!(
        "Version: {:?}, Permissions: {:x?}",
        version_string, permissions
    );

    assert!(metadata.is_some());
    assert_eq!(version_string, Some("PDF-1.3".to_string()));
    assert_eq!(permissions, 0xff);
}

#[test]
fn test3() {
    let mut data = vec![];

    assert!(PopplerDocument::from_data(&mut data[..], "upw").is_err());
}
