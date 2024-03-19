
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
var<storage, read> rect : Rect;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return rect.color;
}

// fn box_sdf(p : vec2<f32>, bounds : vec2<f32>, radius : vec4<f32>) -> f32 {
//     r.xy = if p.x > 0.0 { r.xy } else { r.zw };
//     r.x = if p.y > 0.0 { r.x } else { r.y };
//     let q = abs(p) - bounds + r.x;
//     return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r.x;
// }

// float sdRoundedBox( in vec2 p, in vec2 b, in vec4 r )
// {
//     r.xy = (p.x>0.0)?r.xy : r.zw;
//     r.x  = (p.y>0.0)?r.x  : r.y;
//     vec2 q = abs(p)-b+r.x;
//     return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
// }