mod cfg;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rvi rust_file_name");
        std::process::exit(1);
    }
    let file_name = args[1].clone();
    let config = cfg::Config::loads()?;
    config.write_config(file_name.as_str())?;
    let file_full_path = config.get_full_file_path(file_name.as_str());
    std::process::Command::new(config.get_editor())
        .arg(file_full_path)
        .status()?;
    Ok(())
}
