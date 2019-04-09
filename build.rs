use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

//use bindgen;
use num_cpus;

fn artefacts_built(build_dir: &Path) -> bool {
    build_dir.join("tools/lkl/lib/lkl.o").exists() && build_dir.join("liblinux.a").exists()
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = PathBuf::from(out_dir.clone());

    println!("OUT_DIR {:?}", out_dir);
    let libs_built = artefacts_built(out_dir_path.as_path());

    if !libs_built {
        println!("RMDIR {:?}", out_dir);
        Command::new(format!("rm",))
            .args(&["-rf", out_dir.as_str()])
            .status()
            .unwrap();

        println!("MKDIR {:?}", out_dir);
        Command::new(format!("mkdir",))
            .args(&["-p", out_dir.as_str()])
            .status()
            .unwrap();

        println!("CLONE {:?}", out_dir);
        let options = vec![
            "clone",
            "--depth",
            "1",
            "https://github.com/lkl/linux.git",
            out_dir.as_str(),
        ];
        Command::new("git")
            .args(options.as_slice())
            .status()
            .unwrap();

        println!("BUILD {:?}", out_dir);
        let cpus = format!("{}", num_cpus::get());
        let options = vec!["-C", "tools/lkl", "-j", cpus.as_str()];

        Command::new("make")
            .args(options.as_slice())
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();

        let options = vec!["rcs", "liblinux.a", "tools/lkl/lib/lkl.o"];
        Command::new("ar")
            .args(options.as_slice())
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();

        println!("OUT_DIR {:?}", out_dir);
    }
    assert!(artefacts_built(out_dir_path.as_path()));
    println!("cargo:rustc-link-search=native={}", out_dir);

    /*
    // This currently prevents running bindgen as part of the build.rs script
    // https://github.com/rust-lang/cargo/issues/2589

    let bindings = bindgen::Builder::default()
        .header("arch/lkl/include/uapi/asm/host_ops.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    */
}
