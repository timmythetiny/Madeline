use crate::{
    control,
    image::Image,
    plugin::{self, *},
    utils::{Value, Vec2I, Vec2U},
};
use rayon::prelude::*;

enum Parameters {
    TranslateX,
    TranslateY,
}

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("tx", Value::Integer(0)),
        control::Desc::new("ty", Value::Integer(0)),
    ];
    let desc = plugin::Desc::new("merge", &["bg", "fg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => bg,
        None => return Err(String::from("Invalid background input")),
    };

    let mut out = bg.clone();

    let fg = match inputs[1] {
        Some(fg) => fg,
        // Pass through background input
        None => return Ok(out),
    };

    let tx = controls[Parameters::TranslateX as usize].as_int();
    let ty = controls[Parameters::TranslateY as usize].as_int();
    let translate = Vec2I::new(tx, ty);

    if fg.channel_count() == 4 {
        // Alpha blended
        fg.par_channels()
            .take(3)
            // Would it be better to parallelize over lines?
            .zip(out.par_channels_mut())
            .for_each(|(fg_chan, out_chan)| {
                for (y, (fg_line, alpha_line)) in fg_chan.lines().zip(fg[3].lines()).enumerate() {
                    for (x, (fg_e, alpha_e)) in fg_line.iter().zip(alpha_line.iter()).enumerate() {
                        let pos = translate + Vec2U::new(x, y).into();
                        if let Some(index) = out_chan.index_of(pos) {
                            let bg_e = out_chan[index];
                            let value = *fg_e * alpha_e + bg_e * (1.0 - *alpha_e);
                            out_chan[index] = value;
                        }
                    }
                }
            });

        let out_a = &mut out[3];
        for (y, fg_line) in fg[3].lines().enumerate() {
            for (x, fg_e) in fg_line.iter().enumerate() {
                let pos = translate + Vec2U::new(x, y).into();
                if let Some(index) = out_a.index_of(pos) {
                    let bg_e = out_a[index];
                    let value = 1.0 - (1.0 - fg_e) * (1.0 - bg_e);
                    out_a[index] = value;
                }
            }
        }
    } else {
        // Straight copy
        fg.par_channels()
            .zip(out.par_channels_mut())
            .for_each(|(fg_c, out_c)| {
                for (y, fg_line) in fg_c.lines().enumerate() {
                    for (x, fg_e) in fg_line.iter().enumerate() {
                        let pos = translate + Vec2U::new(x, y).into();
                        if let Some(index) = out_c.index_of(pos) {
                            out_c[index] = *fg_e;
                        }
                    }
                }
            })
    }

    Ok(out)
}
