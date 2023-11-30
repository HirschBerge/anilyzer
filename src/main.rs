use colored::*;
use std::collections::HashMap;
use std::fs;
fn main() {
    let root_path = "/mnt/NAS/Anime"; // Change this to your root directory

    // Create a nested HashMap to store show information
    let mut tv_shows: HashMap<String, HashMap<String, u32>> = HashMap::new();

    // Call the function to build the TV show data
    let results = build_tv_show_data(root_path, &mut tv_shows);

    // Convert HashMap to a vector of key-value pairs and sort by show name
    let mut sorted_results: Vec<_> = results.into_iter().collect();
    sorted_results.sort_by_key(|(show_name, _)| show_name.clone());

    // Print the constructed data structure
    for (show_name, show_info) in &sorted_results {
        let season_cnt = show_info.len() as i32;
        println!("{}:", show_name.purple());
        if &season_cnt <= &1 {
            println!("{}", format!("  {} Season:", season_cnt).bright_red());
        } else {
            println!("{}", format!("  {} Seasons:", season_cnt).bright_red());
        }
        // Convert HashMap to a vector of key-value pairs and sort by season name
        let mut sorted_seasons: Vec<_> = show_info.into_iter().collect();
        sorted_seasons.sort_by_key(|(season, _)| (*season).clone());

        for (season, episodes) in &sorted_seasons {
            println!(
                "    {}: {}",
                season.bright_cyan(),
                format!("{} episodes", episodes).to_string().bright_green()
            );
        }
    }
    println!(
        "{}",
        format!("Total Shows Parsed: {}", sorted_results.len()).bright_blue()
    )
}

fn build_tv_show_data(
    root_path: &str,
    tv_shows: &mut HashMap<String, HashMap<String, u32>>,
) -> HashMap<String, HashMap<String, u32>> {
    // Iterate over the TV show directories
    for show_entry in fs::read_dir(root_path).expect("Failed to read root directory") {
        if let Ok(show_dir) = show_entry {
            if show_dir.file_type().map_or(false, |ft| ft.is_dir()) {
                if let Ok(show_name) = show_dir.file_name().into_string() {
                    let mut seasons: HashMap<String, u32> = HashMap::new();
                    // Iterate over the seasons (subdirectories) of the current show
                    for season_entry in
                        fs::read_dir(show_dir.path()).expect("Failed to read show directory")
                    {
                        if let Ok(season_dir) = season_entry {
                            // Check if it's a directory and not a file
                            if season_dir.file_type().map_or(false, |ft| ft.is_dir()) {
                                let episode_count = season_dir
                                    .path()
                                    .read_dir()
                                    .expect("Failed to read season directory")
                                    .filter_map(|entry| entry.ok())
                                    .filter(|entry| {
                                        entry.file_type().map_or(false, |ft| ft.is_file())
                                    })
                                    .count()
                                    as u32;
                                let season_number =
                                    season_dir.file_name().to_string_lossy().to_string();
                                // Add season information to the HashMap
                                seasons.insert(season_number, episode_count);
                            }
                        }
                    }
                    // dbg!(&show_name, &seasons);
                    // Add show information to the HashMap
                    tv_shows.insert(show_name, seasons);
                }
            }
        }
    }

    tv_shows.clone()
}
