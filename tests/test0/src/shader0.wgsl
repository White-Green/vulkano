struct type_4 {
    member: array<vec4<f32>>,
}

@group(0) @binding(0) 
var<storage> global: type_4;
var<private> global_1: u32;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    let _e5 = global_1;
    let _e8 = global.member[_e5];
    global_2 = _e8;
    let _e10 = global_2[1u];
    global_2[1u] = -(_e10);
    return;
}

@vertex 
fn vert_main(@builtin(vertex_index) param: u32) -> @builtin(position) vec4<f32> {
    global_1 = param;
    function();
    let _e4 = global_2.y;
    global_2.y = -(_e4);
    let _e6 = global_2;
    return _e6;
}
