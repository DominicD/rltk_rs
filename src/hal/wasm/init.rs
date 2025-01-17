pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
) -> super::super::super::Rltk {
    use super::super::super::Rltk;
    use super::super::*;
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width_pixels);
    canvas.set_height(height_pixels);

    super::bind_wasm_events(&canvas);

    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    webgl2_context
        .get_extension("EXT_color_buffer_float")
        .expect("Unable to add extensions");

    let gl = glow::Context::from_webgl2_context(webgl2_context);

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_WITH_BG_VS,
        shader_strings::CONSOLE_WITH_BG_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_NO_BG_VS,
        shader_strings::CONSOLE_NO_BG_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::BACKING_VS,
        shader_strings::BACKING_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SCANLINES_VS,
        shader_strings::SCANLINES_FS,
    ));

    // Build the backing frame-buffer
    let backing_fbo = Framebuffer::build_fbo(&gl, width_pixels as i32, height_pixels as i32);

    // Build a simple quad rendering vao
    let quad_vao = setup_quad(&gl);

    Rltk {
        backend: RltkPlatform {
            gl,
            platform: PlatformGL {
                quad_vao,
                context_wrapper: Some(WrappedContext {}),
                backing_buffer: backing_fbo,
            },
        },
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders,
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        alt: false,
        control: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    }
}
