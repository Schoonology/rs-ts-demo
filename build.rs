use std::process::Command;

fn main() -> std::io::Result<()> {
    compile_css()?;
    compile_ts()?;

    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

fn compile_css() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=src/frontend/style.css");

    assert!(Command::new("npx")
        .args(&[
            "tailwindcss",
            "--input",
            "src/frontend/style.css",
            "--output",
            "public/style.css"
        ])
        .status()
        .unwrap()
        .success(),);

    Ok(())
}

fn compile_ts() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=src/frontend/main.tsx");
    println!("cargo::rerun-if-changed=src/frontend/state.ts");
    println!("cargo::rerun-if-changed=src/frontend/components/form.tsx");
    println!("cargo::rerun-if-changed=src/frontend/components/list.tsx");

    assert!(Command::new("npx")
        .args(&[
            "esbuild",
            "src/frontend/main.tsx",
            "--outfile=public/main.js",
            "--bundle",
            "--loader:.tsx=tsx",
        ])
        .status()
        .unwrap()
        .success(),);

    Ok(())
}
