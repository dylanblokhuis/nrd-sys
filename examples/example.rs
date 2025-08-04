fn main() {
    let descs = [nrd_sys::nrd_DenoiserDesc {
        denoiser: nrd_sys::nrd_Denoiser_REBLUR_DIFFUSE,
        identifier: 0,
    }];

    let instance_creation_desc = nrd_sys::nrd_InstanceCreationDesc {
        denoisers: &descs as *const _ as *const nrd_sys::nrd_DenoiserDesc,
        denoisersNum: descs.len() as u32,
        allocationCallbacks: nrd_sys::nrd_AllocationCallbacks {
            Allocate: None,
            Reallocate: None,
            Free: None,
            userArg: std::ptr::null_mut(),
        },
    };

    let desc = unsafe { &*nrd_sys::nrd_GetLibraryDesc() };
    println!("NRD Library Description: {:#?}", desc);

    let mut instance = std::ptr::null_mut();
    let instance_res =
        unsafe { nrd_sys::nrd_CreateInstance(&instance_creation_desc as _, &mut instance) };
    if instance_res != nrd_sys::nrd_Result_SUCCESS {
        panic!("Failed to create NRD instance: {:?}", instance_res);
    }

    let instance_desc = unsafe { &*nrd_sys::nrd_GetInstanceDesc(instance) };
    println!("NRD Instance Description: {:#?}", instance_desc);

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
