
struct Rect {
    position: vec3<f32>,
    size: vec2<f32>,
    color: vec4<f32>,
    corners : vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    test : f32
};

@group(0) @binding(0)
var<storage, read> rect : array<Rect>;
@group(0) @binding(1)
var<uniform> resolution : vec2<f32>;

@fragment
fn fragment( 
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    var normalized_uv = uv - 0.5;
    var point = resolution * uv;
    
    var current_z = 0.0;
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    
    for (var i = 0; i < i32(arrayLength( &rect )); i++) {
        var curr_rect = rect[i];
        var distance = rounded_box_sdf(point, curr_rect);
        final_color = select (
            curr_rect.color,
            final_color,
            distance > 0.0
        );
    }
    
    // return vec4<f32>((point.x % 10.0) / 10.0, (point.y % 10.0) / 10.0, 0.0, 1.0);
    
    return final_color;
}

fn box_sdf(p : vec2<f32>, bounds : vec2<f32>) -> f32 {
    var d = abs(p)-bounds;
    return length(max(d, vec2<f32>(0.0, 0.0))) + min(max(d.x,d.y),0.0);
}

fn rounded_box_sdf(p : vec2<f32>, rect : Rect) -> f32 {
    var r = rect.corners.xy;
    r = select(rect.corners.xy, rect.corners.zw, p.x > 0.0);
    r = select(r, r.yy, p.y > 0.0);
    var q = abs(p) - rect.size + r.x;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - r.x;
}

// float sdRoundedBox( in vec2 p, in vec2 b, in vec4 r )
// {
//     r.xy = (p.x>0.0)?r.xy : r.zw;
//     r.x  = (p.y>0.0)?r.x  : r.y;
//     vec2 q = abs(p)-b+r.x;
//     return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
// }