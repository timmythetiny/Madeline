use crate::control;
use crate::image::{self, Image};
use crate::plugin::{self, *};
use crate::utils::{Value, Vec2U};
use std::cmp::{min, max};

enum Parameters {
    SizeX,
    SizeY,
}

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("sx", Value::Integer(512)),
        control::Desc::new("sy", Value::Integer(512)),
    ];
    let desc = plugin::Desc::new("resize", &["bg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => bg,
        None => return Err(String::from("Invalid background input")),
    };

    let sx = controls[Parameters::SizeX as usize].as_uint();
    let sy = controls[Parameters::SizeY as usize].as_uint();

    let h_buf = resize_axis(&bg, sx);
    let v_buf = resize_axis(&h_buf, sy);

    Ok(v_buf)
}

fn resize_axis(src: &Image, dim: usize) -> Image {
    let buf_desc = image::Desc::new(Vec2U::new(src.desc().size.y, dim), src.channel_count());
    let mut buf = Image::from_desc(buf_desc);

    let filter_width = 2.0f32;
    let scale_factor_x = src.desc().size.x as f32 / dim as f32;
    let offset_x = filter_width * scale_factor_x;

    for (src_channel, dst_channel) in src.channels().zip(buf.channels_mut()) {
        // Starting with x, which is out-of-order. However, since
        // dst is flipped over y=x, this yields in-order access to
        // the src buffer.
        for (x, line) in dst_channel.lines_mut().enumerate() {
            for (y, px) in line.enumerate() {
                let out_pos = x as f32 * scale_factor_x;
                let lo = (out_pos - offset_x).round() as isize;
                let hi = (out_pos + offset_x).round() as isize;

                let mut acc = 0.0;
                for i in lo..hi {
                    let x_index = min(src.desc().size.x - 1, max(0, i) as usize);
                    let index = y * src.desc().size.x + x_index;
                    let value = src_channel[index];
                    let filter = sample((i - lo) as f32, offset_x);
                    acc += value * filter;
                }

                *px = acc;
            }
        }
    }

    buf
}

fn sample(x: f32, radius: f32) -> f32 {
    gauss(1.0 - (x - radius).abs() / radius) / radius
}

fn gauss(x: f32) -> f32 {
    let rcp = 1.0 - x;
    rcp * x * x + x * (1.0 - rcp * rcp)
}