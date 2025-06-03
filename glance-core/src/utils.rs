/// Alpha blends a given foreground and background pixel color
pub fn alpha_blend(fg: [u8; 4], bg: [u8; 4]) -> [u8; 4] {
    let (rf, gf, bf, af) = (
        fg[0] as f32,
        fg[1] as f32,
        fg[2] as f32,
        fg[3] as f32 / 255.0,
    );
    let (rb, gb, bb, ab) = (
        bg[0] as f32,
        bg[1] as f32,
        bg[2] as f32,
        bg[3] as f32 / 255.0,
    );

    let a_out = af + ab * (1.0 - af);
    if a_out == 0.0 {
        return [0, 0, 0, 0];
    }

    let r_out = (rf * af + rb * ab * (1.0 - af)) / a_out;
    let g_out = (gf * af + gb * ab * (1.0 - af)) / a_out;
    let b_out = (bf * af + bb * ab * (1.0 - af)) / a_out;

    [
        r_out.round() as u8,
        g_out.round() as u8,
        b_out.round() as u8,
        (a_out * 255.0).round() as u8,
    ]
}
