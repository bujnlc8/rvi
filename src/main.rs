mod cfg;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rvi file_name.rs");
        std::process::exit(1);
    }
    let file_name = args[1].clone();
    let config = cfg::Config::loads()?;
    config.write_config(file_name.as_str())?;
    let file_full_path = config.get_full_file_path(file_name.as_str());
    if std::path::PathBuf::from(&file_full_path).exists() {
        std::process::Command::new(config.get_editor())
            .arg(&file_full_path)
            .status()?;
    } else {
        std::process::Command::new(config.get_editor())
            .arg(file_full_path)
            .args(vec![
                "-c",
                r#"call setline(1, "// Created Time: ".strftime("%Y-%m-%d %H:%M:%S"))"#,
                "-c",
                "call setline(2, '')",
                "-c",
                "call setline(3, 'fn main(){')",
                "-c",
                "call setline(4, '')",
                "-c",
                "call setline(5, '')",
                "-c",
                "call setline(6, '}')",
                "-c",
                "call setline(7, '')",
                "-c",
                "call setline(8, '#[cfg(test)]')",
                "-c",
                "call setline(9, 'mod test{}')",
            ])
            .status()?;
    }
    Ok(())
}
