use std::cmp;

// merges rgba pixels source onto target (has transparency)
pub fn merge_rgba(source: [u8; 4], target: [u8; 4]) -> [u8; 4] {
    let mut merged: [u8; 4] = [0,0,0,0];

    let sr = source[0] as f32;
    let sg = source[1] as f32;
    let sb = source[2] as f32;
    let sa = source[3] as f32;

    let tr = target[0] as f32;
    let tg = target[1] as f32;
    let tb = target[2] as f32;
    let ta = target[3] as f32;

    let max = u8::MAX as f32;

    let a_out = sa + (ta * (max - sa) / max);
    let r_out = (sr * sa + tr * ta * (max - sa) / max) / a_out;
    let g_out = (sg * sa + tg * ta * (max - sa) / max) / a_out;
    let b_out = (sb * sa + tb * ta * (max - sa) / max) / a_out;

    merged[0] = cmp::min(r_out as u32, 255) as u8;
    merged[1] = cmp::min(g_out as u32, 255) as u8;
    merged[2] = cmp::min(b_out as u32, 255) as u8;
    merged[3] = cmp::min(a_out as u32, 255) as u8;

    merged
}