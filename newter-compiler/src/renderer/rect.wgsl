struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) rect_pixels: vec4<f32>,
    @location(3) rect_radius: f32,
    @location(4) stroke_color: vec4<f32>,
    @location(5) stroke_width: f32,
    @location(6) viewport: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) rect_pixels: vec4<f32>,
    @location(2) @interpolate(flat) rect_radius: f32,
    @location(3) @interpolate(flat) stroke_color: vec4<f32>,
    @location(4) @interpolate(flat) stroke_width: f32,
    @location(5) @interpolate(flat) viewport: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    out.rect_pixels = in.rect_pixels;
    out.rect_radius = in.rect_radius;
    out.stroke_color = in.stroke_color;
    out.stroke_width = in.stroke_width;
    out.viewport = in.viewport;
    return out;
}

fn rounded_rect_sdf(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let d = abs(p) - b + vec2<f32>(r, r);
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0, 0.0))) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let rect = in.rect_pixels;
    let center = vec2<f32>(rect.x + rect.z * 0.5, rect.y + rect.w * 0.5);
    let half_size = vec2<f32>(rect.z * 0.5, rect.w * 0.5);
    // Convert NDC (clip_position) to pixel space using viewport from vertex
    let px = (in.clip_position.x + 1.0) * 0.5 * in.viewport.x;
    let py = (1.0 - in.clip_position.y) * 0.5 * in.viewport.y;
    let p = vec2<f32>(px, py) - center;
    let d = rounded_rect_sdf(p, half_size, in.rect_radius);
    let edge = 1.0;
    let fill_alpha = 1.0 - smoothstep(-edge, edge, d);
    var color = in.color;
    var alpha = fill_alpha;
    if in.stroke_width > 0.0 {
        let stroke_alpha = smoothstep(-edge, edge, d) * (1.0 - smoothstep(in.stroke_width - edge, in.stroke_width + edge, d));
        let total = fill_alpha + stroke_alpha + 1e-6;
        color = vec4<f32>(
            (in.color.rgb * fill_alpha + in.stroke_color.rgb * stroke_alpha) / total,
            (in.color.a * fill_alpha + in.stroke_color.a * stroke_alpha) / total
        );
        alpha = max(fill_alpha, stroke_alpha);
    }
    return vec4<f32>(color.rgb, color.a * alpha);
}
