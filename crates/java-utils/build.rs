use std::{env, path::PathBuf, process::Command};

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=../java/src");

    let java_dir = env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .with_context(|| "CARGO_MANIFEST_DIR not set")?
        .join("java");
    #[cfg(windows)]
    let gradlew = java_dir.join("gradlew.bat");

    let build = Command::new(gradlew)
        .arg("build")
        .current_dir(&java_dir)
        .spawn()?
        .wait()?;
    if !build.success() {
        return Err(anyhow!(format!("Could not build Java {:?}", build)));
    }
    println!(
        "cargo:rustc-env=JAVA_CHECK={}",
        java_dir
            .join("build")
            .join("libs")
            .join("JavaUtilsCheck.jar")
            .to_string_lossy()
    );

    Ok(())
}
