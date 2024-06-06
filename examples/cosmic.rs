// COSMIC

// SPDX-License-Identifier: MIT OR Apache-2.0

use cosmic_text::BorrowedWithFontSystem;
use cosmic_text::CacheKeyFlags;
use cosmic_text::Color;
use cosmic_text::Shaping;
use cosmic_text::Style;
use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, SwashCache, Weight};
use image::codecs::png::PngEncoder;
use image::Pixel;
use std::fs::File;

fn set_buffer_text<'a>(buffer: &mut BorrowedWithFontSystem<'a, Buffer>) {
    let attrs = Attrs::new();
    let serif_attrs = attrs.family(Family::Serif);
    let mono_attrs = attrs.family(Family::Monospace);
    let comic_attrs = attrs.family(Family::Name("Comic Neue"));

    let spans: &[(&str, Attrs)] = &[
        ("B", attrs.weight(Weight::BOLD)),
        ("old ", attrs),
        ("I", attrs.style(Style::Italic)),
        ("talic ", attrs),
        ("f", attrs),
        ("i ", attrs),
        ("f", attrs.weight(Weight::BOLD)),
        ("i ", attrs),
        ("f", attrs.style(Style::Italic)),
        ("i \n", attrs),
        ("Sans-Serif Normal ", attrs),
        ("Sans-Serif Bold ", attrs.weight(Weight::BOLD)),
        ("Sans-Serif Italic ", attrs.style(Style::Italic)),
        (
            "Sans-Serif Fake Italic ",
            attrs.cache_key_flags(CacheKeyFlags::FAKE_ITALIC),
        ),
        (
            "Sans-Serif Bold Italic\n",
            attrs.weight(Weight::BOLD).style(Style::Italic),
        ),
        ("Serif Normal ", serif_attrs),
        ("Serif Bold ", serif_attrs.weight(Weight::BOLD)),
        ("Serif Italic ", serif_attrs.style(Style::Italic)),
        (
            "Serif Bold Italic\n",
            serif_attrs.weight(Weight::BOLD).style(Style::Italic),
        ),
        ("Mono Normal ", mono_attrs),
        ("Mono Bold ", mono_attrs.weight(Weight::BOLD)),
        ("Mono Italic ", mono_attrs.style(Style::Italic)),
        (
            "Mono Bold Italic\n",
            mono_attrs.weight(Weight::BOLD).style(Style::Italic),
        ),
        ("Comic Normal ", comic_attrs),
        ("Comic Bold ", comic_attrs.weight(Weight::BOLD)),
        ("Comic Italic ", comic_attrs.style(Style::Italic)),
        (
            "Comic Bold Italic\n",
            comic_attrs.weight(Weight::BOLD).style(Style::Italic),
        ),
        ("R", attrs.color(Color::rgb(0xFF, 0x00, 0x00))),
        ("A", attrs.color(Color::rgb(0xFF, 0x7F, 0x00))),
        ("I", attrs.color(Color::rgb(0xFF, 0xFF, 0x00))),
        ("N", attrs.color(Color::rgb(0x00, 0xFF, 0x00))),
        ("B", attrs.color(Color::rgb(0x00, 0x00, 0xFF))),
        ("O", attrs.color(Color::rgb(0x4B, 0x00, 0x82))),
        ("W ", attrs.color(Color::rgb(0x94, 0x00, 0xD3))),
        ("Red ", attrs.color(Color::rgb(0xFF, 0x00, 0x00))),
        ("Orange ", attrs.color(Color::rgb(0xFF, 0x7F, 0x00))),
        ("Yellow ", attrs.color(Color::rgb(0xFF, 0xFF, 0x00))),
        ("Green ", attrs.color(Color::rgb(0x00, 0xFF, 0x00))),
        ("Blue ", attrs.color(Color::rgb(0x00, 0x00, 0xFF))),
        ("Indigo ", attrs.color(Color::rgb(0x4B, 0x00, 0x82))),
        ("Violet ", attrs.color(Color::rgb(0x94, 0x00, 0xD3))),
        ("U", attrs.color(Color::rgb(0x94, 0x00, 0xD3))),
        ("N", attrs.color(Color::rgb(0x4B, 0x00, 0x82))),
        ("I", attrs.color(Color::rgb(0x00, 0x00, 0xFF))),
        ("C", attrs.color(Color::rgb(0x00, 0xFF, 0x00))),
        ("O", attrs.color(Color::rgb(0xFF, 0xFF, 0x00))),
        ("R", attrs.color(Color::rgb(0xFF, 0x7F, 0x00))),
        ("N\n", attrs.color(Color::rgb(0xFF, 0x00, 0x00))),
        (
            "生活,삶,जिंदगी 😀 FPS\n",
            attrs.color(Color::rgb(0xFF, 0x00, 0x00)),
        ),
    ];

    buffer.set_rich_text(spans.iter().copied(), attrs, Shaping::Advanced);
}

fn main() {
    let mut font_system = FontSystem::new();
    let mut swash_cache = SwashCache::new();

    let display_scale = 1.0;
    let metrics = Metrics::new(32.0, 44.0);
    let mut buffer = Buffer::new_empty(metrics.scale(display_scale));
    let mut buffer = buffer.borrow_with(&mut font_system);
    buffer.set_size(1080.0, 720.0);
    set_buffer_text(&mut buffer);

    let width = 1080;
    let height = 720;
    let bg_color = image::Rgba([0x34, 0x34, 0x34, 0xFF]);
    let font_color = Color::rgb(0xFF, 0xFF, 0xFF);

    // Create image
    let mut img = image::RgbaImage::from_pixel(width, height, bg_color);

    buffer.draw(&mut swash_cache, font_color, |x, y, w, h, color| {
        let color = [color.r(), color.g(), color.b(), color.a()].into();
        for j in 0..h {
            for i in 0..w {
                let px = (x as u32 + i).min(width - 1);
                let py = (y as u32 + j).min(height - 1);

                img.get_pixel_mut(px, py).blend(&color);
            }
        }
    });

    // Write image to PNG file in _output dir
    let output_path = {
        let path = std::path::PathBuf::from(file!());
        let mut path = std::fs::canonicalize(path).unwrap();
        path.pop();
        path.pop();
        path.push("_output");
        let _ = std::fs::create_dir(path.clone());
        path.push("cosmic.png");
        path
    };
    let output_file = File::create(output_path).unwrap();
    let png_encoder = PngEncoder::new(output_file);
    img.write_with_encoder(png_encoder).unwrap();
}
