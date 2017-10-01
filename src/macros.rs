
#[macro_export]
macro_rules! lv2_descriptor {

    (@desc $DESC:ident { $uri:expr => $Plug:ty }) => {
        static mut $DESC: $crate::ffi::LV2_Descriptor = $crate::ffi::LV2_Descriptor {
            URI: b"\0" as *const u8 as _,
            instantiate: Some($crate::instantiate::<$Plug>),
            connect_port: Some($crate::connect_port::<$Plug>),
            activate: Some($crate::activate::<$Plug>),
            run: Some($crate::run::<$Plug>),
            deactivate: Some($crate::deactivate::<$Plug>),
            cleanup: Some($crate::cleanup::<$Plug>),
            extension_data: None,
        };
    };


    ( $( $idx:expr => $DESC:ident { $uri:expr => $Plug:ty } ),+ ) => {

        $(
            lv2_descriptor!{ @desc $DESC { $uri => $Plug } }
        )+

        #[no_mangle]
        pub unsafe extern "C" fn lv2_descriptor (index: u32) -> *const $crate::ffi::LV2_Descriptor
        {
            match index {
                $(
                    $idx => {
                        $DESC.URI = concat!($uri, "\0").as_ptr() as _;
                        &$DESC
                    },
                )+
                _ => {
                    use std;
                    std::ptr::null()
                }
            }
        }

    };

}

#[macro_export]
macro_rules! lv2_ports {
    ( $Plug:ty => { $( $idx:expr => $name:ident : $Meta:ty ),+ } ) => {

        #[derive(Copy, Clone)]
        pub struct PortsRaw<'h> {
            $(
                pub $name: <$Meta as $crate::meta::Port<'h>>::FieldRaw
            ),+
        }

        pub struct Ports<'h> {
            $(
                pub $name: <$Meta as $crate::meta::Port<'h>>::Field
            ),+
        }

        unsafe impl<'h> $crate::Ported<'h> for $Plug {
            type PortsRaw = PortsRaw<'h>;
            type Ports = Ports<'h>;
            fn new_ports_raw() -> Self::PortsRaw {
                Self::PortsRaw {
                    $(
                        $name: <$Meta as $crate::meta::Port<'h>>::new_raw()
                    ),+
                }
            }

            fn connect_port(port: usize, data: *mut (), ports_raw: &mut Self::PortsRaw) {
                match port {
                    $(
                        $idx => { ports_raw.$name = <$Meta as $crate::meta::Port<'h>>::cast_raw(data); }
                    ),+
                    _ => {},
                }
            }

            fn convert_ports(ports_raw: Self::PortsRaw, sample_count: usize) -> Self::Ports {
                Self::Ports {
                    $(
                        $name: <$Meta as $crate::meta::Port<'h>>::convert(ports_raw.$name, sample_count)
                    ),+
                }
            }
        }

    }
}

#[macro_export]
macro_rules! lv2_features_query {
    ($list:expr, $( ($name:ident <= $Feat:ty) ),+ ) => {
        let ($($name),+) = {
            $(let mut $name = None;)+
            for f in $list {
                $(
                    if f.uri() == <$Feat as $crate::Feature>::uri() {
                        $name = Some(unsafe {
                            <$Feat as $crate::Feature>::from_raw(&f)
                        });
                    }
                )+
            }
            ($($name),+)
        };
    };
}
