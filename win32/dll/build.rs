#[cfg(target_family = "unix")]
fn main() {
    println!("cargo:rerun-if-env-changed=XWIN");
    if let Ok(xwin) = std::env::var("XWIN") {
        for dir in ["crt/lib/x86", "sdk/lib/ucrt/x86", "sdk/lib/um/x86"] {
            println!(r"cargo:rustc-link-search={xwin}/{dir}");
        }
    }

    println!("cargo::rustc-link-arg=/Brepro");
    println!("cargo::rustc-link-arg=/noentry");
    println!("cargo::rustc-link-arg=/nodefaultlib");
    // This doesn't help, we need -Zmerge-functions=disabled
    // println!("cargo::rustc-link-arg=/OPT:NOICF");
    // This doesn't disable debug info, turns out /Brepro causes it
    // println!("cargo::rustc-link-arg=/DEBUG:NONE");
}

#[cfg(target_family = "windows")]
fn main() {}
