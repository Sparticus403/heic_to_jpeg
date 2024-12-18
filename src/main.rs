use libheif_rs::{
    Channel, RgbChroma, ColorSpace, HeifContext, Result,
    ItemId, LibHeif
};

fn main() -> Result<()> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file("/home/todd/Downloads/sample1.heif")?;
    let handle = ctx.primary_image_handle()?;
    //assert_eq!(handle.width(), 1652);
    //assert_eq!(handle.height(), 1791);
    println!("image height: {}\nimage width: {}", handle.height(), handle.width());

    // Get Exif
    let mut meta_ids: Vec<ItemId> = vec![0;1];
    let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    // assert_eq!(count, 1);
    if count == 1 {
        println!("Exif exists! Woo!");
    } else { println!("No Exif exists!"); }

    let _exif: Vec<u8> = handle.metadata(meta_ids[0])?;

    // Decode the image
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    assert_eq!(
        image.color_space(),
        Some(ColorSpace::Rgb(RgbChroma::Rgb)),
    );
    // assert_eq!(image.width(), 1652);
    // assert_eq!(image.height(), 1791);

    // Scale the image
    // let small_img = image.scale(1024, 800, None)?;
    // assert_eq!(small_img.width(), 1024);
//    assert_eq!(small_img.height(), 800);
//    println!("Image scaled!\n  width: {}\n  heigh: {}", small_img.width(), small_img.height());

    // Get "pixels"
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.unwrap();
    assert_eq!(interleaved_plane.width, 1024);
    assert_eq!(interleaved_plane.height, 800);
    assert!(!interleaved_plane.data.is_empty());
    assert!(interleaved_plane.stride > 0);

    Ok(())
}
