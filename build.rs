use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main()
{
    // ./scripts/bootstrap
    let mut child = Command::new("./scripts/bootstrap").current_dir("xenomai").spawn().expect("Couldn't make bootstrap!");
    child.wait().expect("failed to wait on child");

    // configure --with-core=cobalt --enable-smp --enable-pshared
    let mut child = Command::new("./configure").current_dir("xenomai")
        .arg(&format!("--with-core=cobalt --enable-smp --enable-pshared"))
        .spawn().expect("Couldn't make configure!");
    child.wait().expect("failed to wait on child");

    // make
    let mut child = Command::new("make").current_dir("xenomai").spawn().expect("Couldn't make!");
    child.wait().expect("failed to wait on child");

    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:warning=project_dir: {}", project_dir);

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:warning=out_dir: {}", out_dir);

    let target_dir = PathBuf::from(format!("{}/../../../", out_dir)).to_str().unwrap().to_string();
    println!("cargo:warning=target_dir: {}", target_dir);

    let lib_path = format!("{}/xenomai/lib/alchemy/.libs", project_dir);
    println!("cargo:rustc-link-search=native={}", &lib_path);

    // Command::new("cp")
    //     .arg(&format!("{}/libalchemy.la", &lib_path))
    //     .arg(&target_dir)
    //     .status().unwrap();
    // println!("cargo:rustc-link-lib=static=alchemy");

    Command::new("cp")
        .arg(&format!("{}/libalchemy.so.0", &lib_path))
        .arg(&target_dir)
        .status().unwrap();
    println!("cargo:rustc-link-lib=alchemy");

    // bindgen ./xenomai/lib/alchemy/task.h -o ./binds.rs -- -I./xenomai/include -I./xenomai/include/alchemy -I./xenomai/include/cobalt

    let include_main = "-I./xenomai/include".to_string();
    let include_alchemy = "-I./xenomai/include/alchemy".to_string();
    let include_cobalt = "-I./xenomai/include/cobalt".to_string();

    let bindings = bindgen::Builder::default()
        .header("xenomai/lib/alchemy/task.h")
        .header("xenomai/lib/alchemy/queue.h")
        .clang_arg(include_main.as_str())
        .clang_arg(include_alchemy.as_str())
        .clang_arg(include_cobalt.as_str())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings_gen = bindings.generate().expect("Unable to generate bindings");
    let man_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    bindings_gen
        .write_to_file(man_dir_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}