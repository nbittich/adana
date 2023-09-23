use adana_script_core::primitive::Primitive;

#[test]
fn load_plugin_test() {
    unsafe {
        let lib = libloading::Library::new("/home/nbittich/toyprograms/plugin_example/target/release/libplugin_example.so").unwrap();
        let plugin: libloading::Symbol<unsafe extern "C" fn() -> Primitive> =
            lib.get(b"plugin").unwrap();
        let struc = plugin();

        if let Primitive::Struct(s) = struc {
            if let Some(Primitive::NativeFunction { function }) =
                s.get("say_hello")
            {
                assert_eq!(
                    Primitive::String("Hello Nordine".into()),
                    function(vec![Primitive::String("Nordine".into())])
                );
            } else {
                dbg!(s);
            }
        } else {
            panic!("not a struc");
        }
    }
}
