//#import saiko_ui::style::{BorderStyle, FillStyle};

struct BorderStyle {
    border_color : vec4<f32>,
    border_radius : vec4<f32>,
    border_width : f32,
}

struct FillStyle {
    fill_color : vec4<f32>,
}

struct LineStyle {
    thickness : f32,
}

struct Bound {
    center : vec2<f32>,
    size : vec2<f32>,
    z_index : i32,
}

struct SimpleShape {
    bound : Bound,
    border_style: BorderStyle,
    fill_style: FillStyle,
};

struct Line {
    // a : vec2<f32>,
    // b : vec2<f32>,
    bound : Bound,
    line_style : LineStyle,
    border_style: BorderStyle,
    fill_style: FillStyle,
}

@group(0) @binding(0)
var<storage, read> rects : array<SimpleShape>;
@group(0) @binding(1)
var<storage, read> circles : array<SimpleShape>;
@group(0) @binding(2)
var<storage, read> lines : array<Line>;
@group(0) @binding(3)
var<uniform> resolution : vec2<f32>;

@group(1) @binding(0)
var font_atlas : texture_2d_array<f32>;
@group(1) @binding(1)
var font_atlas_sampler : sampler;

@fragment
fn fragment( 
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    var point = (resolution * uv);
    point = point - (resolution * 0.5);
    
    var current_z = 0;
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    
    //Check this pixel for all rectangles
    for (var i = 0; i < i32(arrayLength(&rects)); i++) {
        var curr_rect = rects[i];
        var distance = rect_sdf(point, curr_rect);
        var is_solid = final_color.a == 1.0;
        var is_over = curr_rect.bound.z_index > current_z;
        var is_fill = distance < 0.0;
        var is_border = abs(distance) < curr_rect.border_style.border_width / 2.0;
        
        if !is_fill && !is_border || (!is_over && is_solid) {
            continue;
        }
        
        var color = select (
            curr_rect.fill_style.fill_color,
            curr_rect.border_style.border_color,
            is_border
        );
        
        final_color = select (
            alpha_blend(final_color, color),
            alpha_blend(color, final_color),
            is_over
        );
        
        current_z = select(
            current_z, 
            curr_rect.bound.z_index, 
            curr_rect.bound.z_index > current_z
        );
    }
    
    for (var i = 0; i < i32(arrayLength(&rects)); i++) {
        var curr_circle = circles[i];
        var distance = circle_sdf(point, curr_circle);
        var is_solid = final_color.a == 1.0;
        var is_over = curr_circle.bound.z_index > current_z;
        var is_fill = distance < 0.0;
        var is_border = abs(distance) < curr_circle.border_style.border_width / 2.0;
        
        if !is_fill && !is_border || (!is_over && is_solid) {
            continue;
        }
        
        var color = select (
            curr_circle.fill_style.fill_color,
            curr_circle.border_style.border_color,
            is_border
        );
        
        final_color = select (
            alpha_blend(final_color, color),
            alpha_blend(color, final_color),
            is_over
        );
        
        current_z = select(
            current_z, 
            curr_circle.bound.z_index, 
            curr_circle.bound.z_index > current_z
        );
    }
    
    
    for (var i = 0; i < i32(arrayLength(&lines)); i++) {
        var curr_line = lines[i];
        var distance = line_sdf(point, curr_line);
        var is_solid = final_color.a == 1.0;
        var is_over = curr_line.bound.z_index > current_z;
        var is_fill = distance < curr_line.line_style.thickness;
        var is_border = abs(distance - curr_line.line_style.thickness) < (curr_line.border_style.border_width / 2.0);
        if !is_fill && !is_border || (!is_over && is_solid) {
            continue;
        }
        
        var color = select (
            curr_line.fill_style.fill_color,
            curr_line.border_style.border_color,
            is_border
        );
        
        final_color = select (
            alpha_blend(final_color, color),
            alpha_blend(color, final_color),
            is_over
        );
        
        current_z = select(
            current_z, 
            curr_line.bound.z_index, 
            curr_line.bound.z_index > current_z
        );
    }
    
    return final_color;
}


// Mix colors with respect to their alpha's.
fn alpha_blend(foreground_color : vec4<f32>, background_color : vec4<f32>) -> vec4<f32> {
    var alpha = clamp(foreground_color.a + background_color.a, 0.0, 1.0);
    var rgb = (1.0 - foreground_color.a) * background_color.rgb + foreground_color.a * foreground_color.rgb;
    return vec4<f32>(rgb, alpha);
}

fn median(r : f32, g : f32, b : f32) -> f32 {
    return max(min(r, g), min(max(r, g), b));
}

fn line_sdf(point : vec2<f32>, line : Line) -> f32 {
    var a = line.bound.center * vec2<f32>(1.0, -1.0);
    var b = line.bound.size * vec2<f32>(1.0, -1.0);
    var pa = point - a;
    var ba = b - a;
    var h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length( pa - h * ba );
}

fn rect_sdf(point : vec2<f32>, rect : SimpleShape) -> f32 {
    var size = (rect.bound.size * 1.0);
    var p = point - (rect.bound.center * vec2<f32>(1.0, -1.0));
    var r = select(rect.border_style.border_radius.xy, rect.border_style.border_radius.zw, p.x > 0.0);
    r = select(r, r.yy, p.y > 0.0);
    r = min(r, size);
    var q = abs(p) - size + r.x;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - r.x;
}

fn circle_sdf(point : vec2<f32>, circle : SimpleShape) -> f32 {
    var p = point - circle.bound.center * vec2<f32>(1.0, -1.0);
    return length(p) - circle.bound.size.x;
}