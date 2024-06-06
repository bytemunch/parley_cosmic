// PARLEY
//
// THINGS TO ASK THE GANG:
// - Obliques
// - Padding (could be this renderer specific tho?)
// -

// Copyright 2024 the Parley Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A simple example that lays out some text using Parley, rasterises the glyph using Swash
//! and and then renders it into a PNG using the `image` crate.

use image::codecs::png::PngEncoder;
use image::{self, Pixel, Rgba, RgbaImage};
use parley::layout::{Alignment, Glyph, GlyphRun, Layout};
use parley::style::{FontFamily, FontStack, FontStyle, FontWeight, StyleProperty};
use parley::{FontContext, LayoutContext};
use peniko::Color;
use std::fs::File;
use swash::scale::image::Content;
use swash::scale::{Render, ScaleContext, Scaler, Source, StrikeWith};
use swash::zeno;
use swash::FontRef;
use zeno::{Format, Vector};

fn main() {
    // The display scale for HiDPI rendering
    let display_scale = 1.0;

    // The width for line wrapping
    let max_advance = Some(1080.0 * display_scale);

    // Colours for rendering
    let text_color = Color::rgb8(255, 255, 255);
    let bg_color = Rgba([0x34, 0x34, 0x34, 0xFF]);

    // Padding around the output image
    let padding = 1;

    // Create a FontContext, LayoutContext and ScaleContext
    //
    // These are all intended to be constructed rarely (perhaps even once per app (or once per thread))
    // and provide caches and scratch space to avoid allocations
    let mut font_cx = FontContext::default();
    let mut layout_cx = LayoutContext::new();
    let mut scale_cx = ScaleContext::new();

    // Prep styles
    let bold = FontWeight::new(600.0);
    let bold_style = StyleProperty::FontWeight(bold);

    let italic = FontStyle::Italic;
    let italic_style = StyleProperty::FontStyle(italic);

    let oblique = FontStyle::Oblique(None);
    let oblique_style = StyleProperty::FontStyle(oblique);

    let serif = FontStack::Source("serif");
    let serif_style = StyleProperty::FontStack(serif);

    let mono = FontStack::Source("monospace");
    let mono_style = StyleProperty::FontStack(mono);

    let comic = FontStack::Single(FontFamily::Named("Comic Nueue"));
    let comic_style = StyleProperty::FontStack(comic);

    let red_style = StyleProperty::Brush(Color::RED);
    let orange_style = StyleProperty::Brush(Color::ORANGE);
    let yellow_style = StyleProperty::Brush(Color::YELLOW);
    let green_style = StyleProperty::Brush(Color::GREEN);
    let blue_style = StyleProperty::Brush(Color::BLUE);
    let indigo_style = StyleProperty::Brush(Color::INDIGO);
    let violet_style = StyleProperty::Brush(Color::VIOLET);

    // Build span list
    let span_list = vec![
        ("B", vec![&bold_style]),
        ("old ", vec![]),
        ("I", vec![&italic_style]),
        ("talic ", vec![]),
        ("f", vec![]),
        ("i ", vec![]),
        ("f", vec![&bold_style]),
        ("i ", vec![]),
        ("f", vec![&italic_style]),
        ("i \n", vec![]),
        ("Sans-Serif Normal ", vec![]),
        ("Sans-Serif Bold ", vec![&bold_style]),
        ("Sans-Serif Italic ", vec![&italic_style]),
        ("Sans-Serif Fake Italic ", vec![&oblique_style]),
        ("Sans-Serif Bold Italic\n", vec![&bold_style, &italic_style]),
        ("Serif Normal ", vec![&serif_style]),
        ("Serif Bold ", vec![&bold_style, &serif_style]),
        ("Serif Italic ", vec![&italic_style, &serif_style]),
        (
            "Serif Bold Italic\n",
            vec![&bold_style, &italic_style, &serif_style],
        ),
        ("Mono Normal ", vec![&mono_style]),
        ("Mono Bold ", vec![&bold_style, &mono_style]),
        ("Mono Italic ", vec![&italic_style, &mono_style]),
        (
            "Mono Bold Italic\n",
            vec![&bold_style, &italic_style, &mono_style],
        ),
        ("Comic Normal ", vec![&comic_style]),
        ("Comic Bold ", vec![&bold_style, &comic_style]),
        ("Comic Italic ", vec![&italic_style, &comic_style]),
        (
            "Comic Bold Italic\n",
            vec![&bold_style, &italic_style, &comic_style],
        ),
        ("R", vec![&red_style]),
        ("A", vec![&orange_style]),
        ("I", vec![&yellow_style]),
        ("N", vec![&green_style]),
        ("B", vec![&blue_style]),
        ("O", vec![&indigo_style]),
        ("W ", vec![&violet_style]),
        ("Red ", vec![&red_style]),
        ("Orange ", vec![&orange_style]),
        ("Yellow ", vec![&yellow_style]),
        ("Green ", vec![&green_style]),
        ("Blue ", vec![&blue_style]),
        ("Indigo ", vec![&indigo_style]),
        ("Violet ", vec![&violet_style]),
        ("U", vec![&violet_style]),
        ("N", vec![&indigo_style]),
        ("I", vec![&blue_style]),
        ("C", vec![&green_style]),
        ("O", vec![&yellow_style]),
        ("R", vec![&orange_style]),
        ("N\n", vec![&red_style]),
        ("生活,삶,जिंदगी 😀 FPS\n", vec![&red_style]),
    ];

    let mut style_queue = vec![];
    let mut full_text = String::new();

    // Use span list to create style queue
    for span in span_list {
        let s = span.0;
        let range_start = full_text.len();
        let range_end = range_start + s.len();
        full_text.insert_str(range_start, s);
        for style in span.1 {
            style_queue.push((style, range_start..range_end));
        }
    }

    // Create a RangedBuilder
    let mut builder = layout_cx.ranged_builder(&mut font_cx, &full_text, display_scale);

    // Set default text colour styles (set foreground text color)
    let brush_style = StyleProperty::Brush(text_color);
    builder.push_default(&brush_style);

    // Set default font family
    //let font_stack = FontStack::Source("system-ui");
    let font_stack = FontStack::Source("sans-serif");
    let font_stack_style = StyleProperty::FontStack(font_stack);
    builder.push_default(&font_stack_style);
    builder.push_default(&StyleProperty::LineHeight(1.0));
    builder.push_default(&StyleProperty::FontSize(32.0));

    // Set the first 4 characters to bold
    //builder.push(&bold_style, 0..4);

    // Apply style queue
    for (style, range) in style_queue {
        builder.push(style, range);
    }

    // Build the builder into a Layout
    let mut layout: Layout<Color> = builder.build();

    // Perform layout (including bidi resolution and shaping) with start alignment
    layout.break_all_lines(max_advance, Alignment::Start);

    // Create image to render into
    //let width = layout.width().ceil() as u32 + (padding * 2);
    //let height = layout.height().ceil() as u32 + (padding * 2);
    let width = 1080;
    let height = 720;
    let mut img = RgbaImage::from_pixel(width, height, bg_color);

    // Iterate over laid out lines
    for line in layout.lines() {
        // Iterate over GlyphRun's within each line
        for glyph_run in line.glyph_runs() {
            render_glyph_run(&mut scale_cx, &glyph_run, &mut img, padding);
        }
    }

    // Write image to PNG file in examples/_output dir
    let output_path = {
        let path = std::path::PathBuf::from(file!());
        let mut path = std::fs::canonicalize(path).unwrap();
        path.pop();
        path.pop();
        path.push("_output");
        let _ = std::fs::create_dir(path.clone());
        path.push("parley.png");
        path
    };
    let output_file = File::create(output_path).unwrap();
    let png_encoder = PngEncoder::new(output_file);
    img.write_with_encoder(png_encoder).unwrap();
}

fn render_glyph_run(
    context: &mut ScaleContext,
    glyph_run: &GlyphRun<Color>,
    img: &mut RgbaImage,
    padding: u32,
) {
    // Resolve properties of the GlyphRun
    let mut run_x = glyph_run.offset();
    let run_y = glyph_run.baseline();
    let style = glyph_run.style();
    let color = style.brush;

    // Get the "Run" from the "GlyphRun"
    let run = glyph_run.run();

    // Resolve properties of the Run
    let font = run.font();
    let font_size = run.font_size();
    let normalized_coords = run.normalized_coords();

    // Convert from parley::Font to swash::FontRef
    let font_ref = FontRef::from_index(font.data.as_ref(), font.index as usize).unwrap();

    // Build a scaler. As the font properties are constant across an entire run of glyphs
    // we can build one scaler for the run and reuse it for each glyph.
    let mut scaler = context
        .builder(font_ref)
        .size(font_size)
        .hint(true)
        .normalized_coords(normalized_coords)
        .build();

    // Iterates over the glyphs in the GlyphRun
    for glyph in glyph_run.glyphs() {
        let glyph_x = run_x + glyph.x + (padding as f32);
        let glyph_y = run_y - glyph.y + (padding as f32);
        run_x += glyph.advance;

        render_glyph(img, &mut scaler, color, glyph, glyph_x, glyph_y);
    }
}

fn render_glyph(
    img: &mut RgbaImage,
    scaler: &mut Scaler,
    color: Color,
    glyph: Glyph,
    glyph_x: f32,
    glyph_y: f32,
) {
    // Compute the fractional offset
    // You'll likely want to quantize this in a real renderer
    let offset = Vector::new(glyph_x.fract(), glyph_y.fract());

    // Render the glyph using swash
    let rendered_glyph = Render::new(
        // Select our source order
        &[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ],
    )
    // Select the simple alpha (non-subpixel) format
    .format(Format::Alpha)
    // Apply the fractional offset
    .offset(offset)
    // Render the image
    .render(scaler, glyph.id)
    .unwrap();

    let glyph_width = rendered_glyph.placement.width;
    let glyph_height = rendered_glyph.placement.height;
    let glyph_x = (glyph_x.floor() as i32 + rendered_glyph.placement.left) as u32;
    let glyph_y = (glyph_y.floor() as i32 - rendered_glyph.placement.top) as u32;

    match rendered_glyph.content {
        Content::Mask => {
            let mut i = 0;
            for pixel_y in 0..glyph_height {
                for pixel_x in 0..glyph_width {
                    let x = glyph_x + pixel_x;
                    let y = glyph_y + pixel_y;
                    let alpha = rendered_glyph.data[i];
                    let color = Rgba([color.r, color.g, color.b, alpha]);
                    img.get_pixel_mut(x, y).blend(&color);
                    i += 1;
                }
            }
        }
        Content::SubpixelMask => unimplemented!(),
        Content::Color => {
            let row_size = glyph_width as usize * 4;
            for (pixel_y, row) in rendered_glyph.data.chunks_exact(row_size).enumerate() {
                for (pixel_x, pixel) in row.chunks_exact(4).enumerate() {
                    let x = glyph_x + pixel_x as u32;
                    let y = glyph_y + pixel_y as u32;
                    let color = Rgba(pixel.try_into().expect("Not RGBA"));
                    img.get_pixel_mut(x, y).blend(&color);
                }
            }
        }
    };
}
