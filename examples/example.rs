fn shader_bytes(desc: &nrd_sys::ComputeShaderDesc) -> &[u8] {
    if desc.bytecode.is_null() || desc.size == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(desc.bytecode as *const u8, desc.size as usize) }
}

fn main() {
    let _lib = nrd_sys::LibraryInfo::query().expect(
        "linked libNRD major.minor must match this crate's headers; update Include/libNRD or regenerate ffi",
    );

    let mut instance = nrd_sys::Instance::try_new_denoisers(&[nrd_sys::DenoiserSlot {
        identifier: nrd_sys::Identifier(0),
        denoiser: nrd_sys::Denoiser::ReblurDiffuse,
    }])
    .expect("Create NRD instance");

    let inst_desc = instance.description().expect("instance description");
    for pipeline in inst_desc.pipelines() {
        let spirv = shader_bytes(&pipeline.computeShaderSPIRV);
        let metal = shader_bytes(&pipeline.computeShaderMetal);
        println!("SPIRV size: {} bytes", spirv.len());
        println!("Metal metallib size: {} bytes", metal.len());
        println!("{:#?}", pipeline);
    }
    println!("{:#?}", inst_desc.raw());

    let mut common_settings = nrd_sys::default_common_settings();
    common_settings.resourceSize = [1920, 1080];
    common_settings.rectSize = [1920, 1080];

    let reblur_settings = nrd_sys::default_reblur_settings();

    instance
        .set_common_settings(&common_settings)
        .expect("SetCommonSettings");
    instance
        .set_reblur_settings(nrd_sys::Identifier(0), &reblur_settings)
        .expect("SetDenoiserSettings (REBLUR)");

    let dispatches = instance
        .compute_dispatches(&[nrd_sys::Identifier(0)])
        .expect("GetComputeDispatches");
    println!("{:#?}", dispatches);

    if let Some(name) = nrd_sys::denoiser_name(nrd_sys::Denoiser::ReblurDiffuse) {
        println!("denoiser: {}", name.to_string_lossy());
    }
}
