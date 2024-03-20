
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

@fragment
fn fragment( 
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    var resolution = pos.xy / uv;
    var fragment_pos = 
    var distance = box_sdf(uv, vec2<f32>(0.1, 0.1));
    // let distance = box_sdf();
    if (distance < 0.0) {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }
    
    return rect[0].color * distance;
    // return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}

fn box_sdf(p : vec2<f32>, bounds : vec2<f32>) -> f32 {
    var d = abs(p)-bounds;
    return length(max(d, vec2<f32>(0.0, 0.0))) + min(max(d.x,d.y),0.0);
}

// float sdRoundedBox( in vec2 p, in vec2 b, in vec4 r )
// {
//     r.xy = (p.x>0.0)?r.xy : r.zw;
//     r.x  = (p.y>0.0)?r.x  : r.y;
//     vec2 q = abs(p)-b+r.x;
//     return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
// }