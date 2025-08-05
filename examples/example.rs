fn main() {
    let desc = nrd_sys::LibraryDesc::new();

    let instance = desc.create_instance(&nrd_sys::ffi::nrd_InstanceCreationDesc {
        allocationCallbacks: nrd_sys::ffi::nrd_AllocationCallbacks {
            Allocate: None,
            Free: None,
            Reallocate: None,
            userArg: std::ptr::null_mut(),
        },
        denoisers: &nrd_sys::ffi::nrd_DenoiserDesc {
            identifier: 0,
            denoiser: nrd_sys::Denoiser::ReblurDiffuse as u32,
        },
        denoisersNum: 1,
    });

    let desc = instance.get_desc();
    let common_settings = instance.get_default_common_settings();
    let reblur_settings = instance.get_default_reblur_settings();

    instance.set_common_settings(&common_settings);
    instance.set_reblur_settings(0, &reblur_settings);

    let dispatches = instance.get_dispatches(&[0]);
    println!("{:#?}", dispatches);

    // println!("{:#?}", desc.roughness_encoding());
    // let lib_desc = nrd_sys::Instance::library_desc();
    // println!("{:#?}", lib_desc);
    // let id1 = nrd_sys::Identifier(0);
    // let mut instance = nrd_sys::Instance::new(&[nrd_sys::DenoiserDesc {
    //     identifier: id1,
    //     denoiser: nrd_sys::Denoiser::ReblurDiffuse,
    // }])
    // .unwrap();
    // let desc = instance.desc();
    // println!("{:#?}", desc);

    // instance
    //     .set_common_settings(&nrd_sys::CommonSettings {
    //         resource_size: [1920, 1080],
    //         rect_size: [1920, 1080],
    //         ..Default::default()
    //     })
    //     .unwrap();
    // instance
    //     .set_denoiser_settings(id1, &nrd_sys::ReferenceSettings::default())
    //     .unwrap();

    // let dispatches = instance.get_compute_dispatches(&[id1]).unwrap();

    // println!("{:#?}", dispatches);
}
