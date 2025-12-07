use std::{
    fs::File,
    io::Write,
    ptr::null_mut,
    time::{Duration, Instant},
};

use amffi::{
    amf_init,
    components::video_encoder_vce::{
        AMF_VIDEO_ENCODER_FRAMERATE, AMF_VIDEO_ENCODER_FRAMESIZE, AMF_VIDEO_ENCODER_TARGET_BITRATE,
        AMF_VIDEO_ENCODER_USAGE, AMF_VIDEO_ENCODER_VCE_AVC, AMFVideoEncoderUsage,
    },
    core::{
        buffer::AMFBuffer,
        context::AMFContext,
        interface::Interface,
        platform::{AMFRate, AMFSize},
        result::AMFError,
        surface::AMFSurface,
        trace::{AMFTraceWriterConsole, AMFTraceWriterDebugOutput},
    },
};
use widestring::widecstr;
use windows::{
    Win32::Graphics::Direct3D11::{D3D11_BOX, ID3D11Device, ID3D11Texture2D},
    core::Interface as _,
};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const RECT_SIZE: u32 = 50;

fn main() {
    let library = amf_init().unwrap();
    let version = library.query_version().unwrap();
    let factory = library.init_factory(version).unwrap();
    let trace = factory.get_trace().unwrap();
    trace.set_writer_enabled::<AMFTraceWriterConsole>(true);
    trace.set_writer_enabled::<AMFTraceWriterDebugOutput>(true);

    let context = factory.create_context().unwrap();
    unsafe {
        context
            .init_dx11_raw(null_mut(), amffi::core::data::AMFDXVersion::DX11_0)
            .unwrap()
    };
    #[allow(unused)]
    let (surface_1, surface_2) = prepare_dx11_surface(&context).unwrap();

    let encoder = factory
        .create_component(&context, AMF_VIDEO_ENCODER_VCE_AVC)
        .unwrap();

    encoder
        .set_property(
            AMF_VIDEO_ENCODER_USAGE,
            AMFVideoEncoderUsage::Transcoding as i64,
        )
        .unwrap();
    encoder
        .set_property(AMF_VIDEO_ENCODER_TARGET_BITRATE, 5000000i64)
        .unwrap();
    let size = AMFSize::new(WIDTH, HEIGHT);
    encoder
        .set_property(AMF_VIDEO_ENCODER_FRAMESIZE, size)
        .unwrap();
    let rate = AMFRate::new(30, 1);
    encoder
        .set_property(AMF_VIDEO_ENCODER_FRAMERATE, rate)
        .unwrap();

    encoder
        .init(amffi::core::surface::AMFSurfaceFormat::Nv12, WIDTH, HEIGHT)
        .unwrap();

    let handle = std::thread::spawn({
        let encoder = encoder.clone();
        move || {
            let mut file = File::create("./output.h264").unwrap();
            loop {
                let output = encoder.query_output();
                match output {
                    Ok(data) => {
                        let buffer: AMFBuffer = data.cast().unwrap();
                        file.write_all(unsafe {
                            std::slice::from_raw_parts(
                                buffer.get_native() as _,
                                buffer.get_size() as usize,
                            )
                        })
                        .unwrap();
                    }
                    Err(AMFError::Eof) => {
                        break;
                    }
                    Err(AMFError::Repeat) => {}
                    Err(e) => panic!("{e:?}"),
                }
            }
        }
    });

    let mut submitted = 0;
    let mut surface_in = None;

    let mut x_pos = 0;
    let mut y_pox = 0;

    while submitted < 600 {
        if surface_in.is_none() {
            surface_in = Some(
                context
                    .alloc_surface(
                        amffi::core::data::AMFMemoryType::DX11,
                        amffi::core::surface::AMFSurfaceFormat::Nv12,
                        WIDTH,
                        HEIGHT,
                    )
                    .unwrap(),
            );

            fill_surface_dx11(
                &context,
                &surface_in.clone().unwrap(),
                &surface_1,
                &surface_2,
                &mut x_pos,
                &mut y_pox,
            );
        }

        let instant = Instant::now();
        surface_in = if let Some(surface) = &surface_in {
            surface
                .set_property(
                    widecstr!("StartTimeProperty"),
                    Instant::now().duration_since(instant).as_nanos() as i64,
                )
                .unwrap();
            let result = encoder.submit_input(surface);
            match result {
                Ok(()) => {
                    submitted += 1;
                    None
                }
                Err(AMFError::InputFull) => {
                    std::thread::sleep(Duration::from_millis(1));
                    surface_in
                }
                Err(e) => panic!("{e:?}"),
            }
        } else {
            surface_in
        };
    }

    loop {
        let res = encoder.drain();
        if res == Ok(()) {
            break;
        }
    }

    let _ = handle.join();

    encoder.terminate().unwrap();
}

fn fill_surface_dx11(
    context: &AMFContext,
    surface: &AMFSurface,
    color_1: &AMFSurface,
    color_2: &AMFSurface,
    x_pos: &mut u32,
    y_pox: &mut u32,
) {
    // The device returned does not inc the reference count
    let ptr = context.get_dx11_device(amffi::core::data::AMFDXVersion::DX11_0);
    let device = unsafe { ID3D11Device::from_raw_borrowed(&ptr).unwrap() };
    let plane = surface.get_plane_at(0);
    let ptr = plane.get_native();
    let surface_dx11 = unsafe { ID3D11Texture2D::from_raw_borrowed(&ptr).unwrap() };

    let context = unsafe { device.GetImmediateContext() }.unwrap();
    let ptr = color_1.get_plane_at(0).get_native();
    let surface_dx11_color_1 = unsafe { ID3D11Texture2D::from_raw_borrowed(&ptr).unwrap() };
    unsafe { context.CopyResource(surface_dx11, surface_dx11_color_1) };

    if *x_pos + RECT_SIZE > WIDTH as u32 {
        *x_pos = 0;
    }

    if *y_pox + RECT_SIZE > HEIGHT as u32 {
        *y_pox = 0;
    }

    let rect = D3D11_BOX {
        left: 0,
        top: 0,
        front: 0,
        right: RECT_SIZE,
        bottom: RECT_SIZE,
        back: 1,
    };

    let ptr = color_2.get_plane_at(0).get_native();
    let surface_dx11_color_2 = unsafe { ID3D11Texture2D::from_raw_borrowed(&ptr).unwrap() };
    unsafe { context.CopyResource(surface_dx11, surface_dx11_color_2) };

    unsafe {
        context.CopySubresourceRegion(
            surface_dx11,
            0,
            *x_pos,
            *y_pox,
            0,
            surface_dx11_color_2,
            0,
            Some(&rect),
        )
    };
    unsafe {
        context.Flush();
    }
    *x_pos += 2;
    *y_pox += 2;
}

fn prepare_dx11_surface(context: &AMFContext) -> Result<(AMFSurface, AMFSurface), AMFError> {
    let surface_1 = context.alloc_surface(
        amffi::core::data::AMFMemoryType::Host,
        amffi::core::surface::AMFSurfaceFormat::Nv12,
        WIDTH,
        HEIGHT,
    )?;
    let surface_2 = context.alloc_surface(
        amffi::core::data::AMFMemoryType::Host,
        amffi::core::surface::AMFSurfaceFormat::Nv12,
        RECT_SIZE as i32,
        RECT_SIZE as i32,
    )?;

    unsafe {
        fill_nv12_surface_with_color(&surface_1, 128, 255, 128);
    }
    unsafe {
        fill_nv12_surface_with_color(&surface_2, 128, 0, 128);
    }

    surface_1.convert(amffi::core::data::AMFMemoryType::DX11)?;
    surface_2.convert(amffi::core::data::AMFMemoryType::DX11)?;

    Ok((surface_1, surface_2))
}

unsafe fn fill_nv12_surface_with_color(surface: &AMFSurface, nv_y: u8, u: u8, v: u8) {
    let plane_y = surface.get_plane_at(0);
    let plane_uv = surface.get_plane_at(1);

    let width_y = plane_y.get_width();
    let height_y = plane_y.get_height();
    let line_y = plane_y.get_h_pitch();

    let data_y = plane_y.get_native() as *mut u8;

    for y in 0..height_y {
        let data_line = unsafe { data_y.add((y * line_y) as usize) };
        unsafe {
            data_line.write_bytes(nv_y, width_y as usize);
        }
    }

    let width_uv = plane_uv.get_width();
    let height_uv = plane_uv.get_height();
    let line_uv = plane_uv.get_h_pitch();

    let data_uv = plane_uv.get_native() as *mut u8;

    for y in 0..height_uv {
        let mut data_line = unsafe { data_uv.add((y * line_uv) as usize) };
        for _ in 0..width_uv {
            unsafe { *data_line = u }
            unsafe { *data_line.add(1) = v }
            data_line = unsafe { data_line.add(2) };
        }
    }
}
