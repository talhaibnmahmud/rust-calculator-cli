use image::GenericImageView;

mod args;
mod calculator;

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let capacity = width * height * 4;
        println!("Creating buffer with capacity: {}", capacity);
        let buffer = Vec::with_capacity(capacity.try_into().unwrap());

        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        println!("Updating placeholder image buffer with image data");

        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}

fn find_image_from_path(path: String) -> (image::DynamicImage, image::ImageFormat) {
    let image_reader = image::ImageReader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();

    (image, image_format)
}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    if dim_1.0 * dim_1.1 < dim_2.0 * dim_2.1 {
        dim_1
    } else {
        dim_2
    }
}

fn standardise_size(
    image_1: image::DynamicImage,
    image_2: image::DynamicImage,
) -> (image::DynamicImage, image::DynamicImage) {
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
    println!("Width: {}, Height: {}\n", width, height);
    if image_1.dimensions() == image_2.dimensions() {
        println!("Images are or same size, returning as is ...");
        return (image_1, image_2);
    }
    if image_2.dimensions() == (width, height) {
        println!("Resizing image 1 ...");
        return (
            image_1.resize_exact(width, height, image::imageops::FilterType::Triangle),
            image_2,
        );
    }

    println!("Resizing image 2 ...");
    (
        image_1,
        image_2.resize_exact(width, height, image::imageops::FilterType::Triangle),
    )
}

fn set_rgba(vec: &[u8], start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for index in start..=end {
        let value = match vec.get(index) {
            Some(d) => *d,
            None => panic!("Index out of bound!"),
        };

        rgba.push(value);
    }

    rgba
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];
    println!(
        "Created combined image buffer with size: {}",
        combined_data.len()
    );

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..i + 3, set_rgba(&vec_1, 1, 1 + 3));
        } else {
            combined_data.splice(i..i + 3, set_rgba(&vec_2, 1, 1 + 3));
        }
        i += 4;
    }

    combined_data
}

fn combine_images(image_1: image::DynamicImage, image_2: image::DynamicImage) -> Vec<u8> {
    println!("Combining images together");
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    println!("Merging images with alternating columns ...");
    alternate_pixels(vec_1, vec_2)
}

fn main() -> Result<(), ImageDataErrors> {
    let args = args::Args::new();
    println!("Args: {:#?}", args);

    let (image_1, image_format_1) = find_image_from_path(args.image_1);
    let (image_2, image_format_2) = find_image_from_path(args.image_2);
    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    };

    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?;
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_format_1,
    )
    .unwrap();

    Ok(())
}
