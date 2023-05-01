use anyhow::Result;
use configparser::ini::Ini;
use dialoguer::{FuzzySelect, theme::ColorfulTheme};

fn get_profile_list() -> Result<Vec<String>> {
    let home_dir = std::env::var("HOME")?;
    let config = Ini::new()
        .load(format!("{}/.aws/config", home_dir))
        .expect("config file not found");
    let profiles = config
        .into_keys()
        .map(|s| s.replace("profile ", ""))
        .collect();

    Ok(profiles)
}

fn main() -> Result<()> {
    let items = get_profile_list()?;
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which?")
        .items(&items)
        .default(0)
        .interact()?;

    println!("{} selected.", items[selection]);

    for (key, value) in std::env::vars() {
        println!{"{}={}", key, value};
    }
    
    Ok(())
}
