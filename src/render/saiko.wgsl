//#import saiko_ui::style::{BorderStyle, FillStyle};

struct BorderStyle {
    border_color : vec4<f32>,
    border_radius : vec4<f32>,
    border_width : f32,
}

struct FillStyle {
    fill_color : vec4<f32>,
}

struct Bound {
    center : vec2<f32>,
    size : vec2<f32>,
    z_index : f32,
}

struct Rect {
    bound : Bound,
    border_style: BorderStyle,
    fill_style: FillStyle,
};

@group(0) @binding(0)
var<storage, read> rect : array<Rect>;
@group(0) @binding(1)
var<uniform> resolution : vec2<f32>;

@group(1) @binding(0)
var font_atlas : texture_2d_array<f32>;
@group(1) @binding(1)
var font_atlas_sampler : sampler;

@fragment
fn fragment( 
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    var normalized_uv = uv - 0.5;
    var point = resolution * uv;
    point = point - (resolution * 0.5);
    
    var current_z = 0.0;
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    
    for (var i = 0; i < i32(arrayLength(&rect)); i++) {
        var curr_rect = rect[i];
        var distance = rounded_box_sdf(point, curr_rect);
        final_color = select (
            curr_rect.fill_style.fill_color,
            final_color,
            distance > 0.0
        );
        final_color = select (
            final_color,
            curr_rect.border_style.border_color,
            abs(distance) < curr_rect.border_style.border_width / 2.0
        );
    }
        
    // return final_color;
    var color = textureSample(font_atlas, font_atlas_sampler, uv, 0);
    var distance = median(color.r, color.g, color.b);
    var screen_distance = 2.5 * (distance - 0.5);
    var opacity = clamp(screen_distance + 0.5, 0.0, 1.0);
    return vec4<f32>(1.0, 1.0, 1.0, opacity);
    
    
    // let s = smoothstep(0.0, 1.0, cos(distance));
    // return vec4<f32>(0.0, 0.0, s, 1.0);
    
    // return select(
    //     vec4<f32>(1.0, 1.0, 1.0, 1.0),
    //     vec4<f32>(0.0, 0.0, 0.0, 1.0),
    //     screen_distance < 0.0
    // );
}

fn median(r : f32, g : f32, b : f32) -> f32 {
    return max(min(r, g), min(max(r, g), b));
}

fn box_sdf(p : vec2<f32>, bounds : vec2<f32>) -> f32 {
    var d = abs(p)-bounds;
    return length(max(d, vec2<f32>(0.0, 0.0))) + min(max(d.x,d.y),0.0);
}

fn rounded_box_sdf(point : vec2<f32>, rect : Rect) -> f32 {
    var size = (rect.bound.size * 1.0);
    var p = point - (rect.bound.center * vec2<f32>(1.0, -1.0));
    var r = select(rect.border_style.border_radius.xy, rect.border_style.border_radius.zw, p.x > 0.0);
    r = select(r, r.yy, p.y > 0.0);
    r = min(r, size);
    var q = abs(p) - size + r.x;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - r.x;
}