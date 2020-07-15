extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();

    cc::Build::new()
        .file("src/interceptor.asm")
        .compile("interceptor");
    // println!("cargo:rerun-if-changed=interceptor.asm");

    res.compile().unwrap();

}
