fn main() {
    slint_build::compile_with_config(
        "ui/main.slint",
        slint_build::CompilerConfiguration::new()
            .with_library_paths(std::collections::HashMap::from([
                ("material".into(), 
                 std::path::PathBuf::from("material-1.0/material.slint"))
            ]))
    ).unwrap();
}
