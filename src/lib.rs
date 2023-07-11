use ext_php_rs::{prelude::*, types::ZendObject};

#[php_function]
pub fn caesium_compress(input: String, output: String, params: &mut ZendObject) -> bool {
    let mut parameters = caesium::initialize_parameters();
    parameters.jpeg.quality = params.get_property("jpeg_quality").unwrap_or(80);
    parameters.png.quality = params.get_property("png_quality").unwrap_or(80);
    parameters.optimize = params.get_property("optimize").unwrap_or(false);
    parameters.keep_metadata = params.get_property("keep_metadata").unwrap_or(false);
    parameters.png.force_zopfli = params.get_property("png_force_zopfli").unwrap_or(false);
    parameters.gif.quality = params.get_property("gif_quality").unwrap_or(80);
    parameters.webp.quality = params.get_property("webp_quality").unwrap_or(80);
    parameters.width = params.get_property("width").unwrap_or(0);
    parameters.height = params.get_property("height").unwrap_or(0);

    caesium::compress(input, output, &parameters).is_ok()
}

#[php_function]
pub fn caesium_compress_to_size(input: String, output: String, params: &mut ZendObject, max_output_size: usize) -> bool {
    let mut parameters = caesium::initialize_parameters();
    parameters.jpeg.quality = params.get_property("jpeg_quality").unwrap_or(80);
    parameters.png.quality = params.get_property("png_quality").unwrap_or(80);
    parameters.optimize = params.get_property("optimize").unwrap_or(false);
    parameters.keep_metadata = params.get_property("keep_metadata").unwrap_or(false);
    parameters.png.force_zopfli = params.get_property("png_force_zopfli").unwrap_or(false);
    parameters.gif.quality = params.get_property("gif_quality").unwrap_or(80);
    parameters.webp.quality = params.get_property("webp_quality").unwrap_or(80);
    parameters.width = params.get_property("width").unwrap_or(0);
    parameters.height = params.get_property("height").unwrap_or(0);

    caesium::compress_to_size(input, output, &mut parameters, max_output_size).is_ok()
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}