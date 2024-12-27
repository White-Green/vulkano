use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::shader::spirv::bytes_to_words;
use vulkano::shader::{ShaderModule, ShaderModuleCreateInfo};
use vulkano::VulkanLibrary;

fn main() {
    let library = VulkanLibrary::new().unwrap();
    let instance = Instance::new(library, InstanceCreateInfo::default()).unwrap();
    let physical_device = instance
        .enumerate_physical_devices()
        .unwrap()
        .next()
        .unwrap();
    let (device, _) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo::default()],
            ..DeviceCreateInfo::default()
        },
    )
    .unwrap();

    // shader.wgsl: hand-written
    // shader.spv: convert from shader.wgsl by naga-cli `naga shader.wgsl shader.spv`
    // shader0.spvasm: convert from shader.spv by spirv-tools `spirv-dis shader.spv -o shader0.spvasm`
    // shader1.spvasm: copied from shader0.spvasm `cp shader0.spvasm shader1.spvasm` and edit
    // shader0.spv: convert from shader0.spvasm by spirv-tools `spirv-as --target-env spv1.0 shader0.spvasm -o shader0.spv`
    // shader1.spv: convert from shader1.spvasm by spirv-tools `spirv-as --target-env spv1.0 shader1.spvasm -o shader1.spv`
    // shader0.wgsl: convert from shader0.spv by naga-cli `naga shader0.spv shader0.wgsl`
    // shader1.wgsl: convert from shader1.spv by naga-cli `naga shader1.spv shader1.wgsl`
    let shader_module0 = unsafe {
        ShaderModule::new(
            device.clone(),
            ShaderModuleCreateInfo::new(&bytes_to_words(include_bytes!("shader0.spv")).unwrap()),
        )
        .unwrap()
    };
    let vert0 = shader_module0.entry_point("vert_main").unwrap();
    dbg!(vert0.info()); // contains @binding(0) @group(0)

    let shader_module1 = unsafe {
        ShaderModule::new(
            device.clone(),
            ShaderModuleCreateInfo::new(&bytes_to_words(include_bytes!("shader1.spv")).unwrap()),
        )
        .unwrap()
    };
    let vert1 = shader_module1.entry_point("vert_main").unwrap();
    dbg!(vert1.info()); // empty...
}
