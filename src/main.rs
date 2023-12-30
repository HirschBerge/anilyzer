use clap::Parser;
use colored::*;

use std::fs;

#[derive(Parser, Debug)]
#[command(
    author = "HirschBerge",
    version = "v1.0.1",
    about = "Parses Plex or Jellyfin Libraries."
)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "/mnt/NAS/Anime/")]
    path: String,
}
#[derive(Debug, Clone)]
struct Show {
    title: String,
    season_cnt: u8,
    season_names: Vec<Season>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)] // Add Clone here
struct Season {
    season_of: String,
    season_title: String,
    epi_count: u16,
    // titles: Vec<String>, //Not currently useful.
}

impl Show {
    fn push_season(&mut self, szn: Season) {
        self.season_names.push(szn);
    }
    fn print(self) {
        println!("{}", self.title.purple());
        let mut szn_cnt_string = "".to_string();
        if self.season_cnt == 1 {
            szn_cnt_string = format!("  {} Season", &self.season_cnt);
        } else {
            szn_cnt_string = format!("  {} Seasons", &self.season_cnt);
        }
        println!("{}", szn_cnt_string.bright_red());
        for seaz in self.season_names {
            let e_count = format!("{} Episodes", seaz.epi_count).bright_green();
            println!("    {}: {}", seaz.season_title.bright_cyan(), e_count);
        }
    }
    fn sorted_by_season_title(&self) -> Show {
        let mut sorted_seasons = self.season_names.clone();
        sorted_seasons.sort_by(|a, b| a.season_title.cmp(&b.season_title));

        Show {
            title: self.title.clone(),
            season_cnt: self.season_cnt,
            season_names: sorted_seasons,
        }
    }
}

fn main() {
    let args = Args::parse();
    let root_path = args.path; // Change this to your root directory

    // Create a nested HashMap to store show information
    // let mut tv_shows: HashMap<String , HashMap<String, u32>> = HashMap::new();

    // Call the function to build the TV show data
    let mut results = build_tv_show_data(root_path);
    results.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    let mut count = 0;
    for show in results {
        let sorted_show = show.sorted_by_season_title();
        sorted_show.print();
        count += 1;
    }
    let totals = format!("Total Shows parsed: {count}").bright_blue();
    println!("{totals}");
}

fn build_tv_show_data(root_path: String) -> Vec<Show> {
    let mut all_shows: Vec<Show> = vec![];
    // Iterate over the TV show directories
    for show_entry in fs::read_dir(root_path).expect("Failed to read root directory") {
        if let Ok(show_dir) = show_entry {
            if show_dir.file_type().map_or(false, |ft| ft.is_dir()) {
                if let Ok(show_name) = show_dir.file_name().into_string() {
                    let mut show = Show {
                        title: show_name.clone(),
                        season_cnt: fs::read_dir(show_dir.path())
                            .expect("Failed to read show directory")
                            .filter_map(|entry| entry.ok())
                            .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_dir()))
                            .count() as u8,
                        season_names: vec![],
                    };
                    // let mut seasons: HashMap<String, u32> = HashMap::new();
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
                                    as u16;
                                let season_number =
                                    season_dir.file_name().to_string_lossy().to_string();
                                // Add season information to the HashMap
                                let szn = Season {
                                    season_of: show_name.clone(),
                                    season_title: season_number,
                                    epi_count: episode_count,
                                };
                                show.push_season(szn);
                            }
                        }
                    }
                    // dbg!(&show_name, &seasons);
                    // dbg!(show);
                    // Add show information to the HashMap
                    // tv_shows.insert(show_name, seasons);
                    all_shows.push(show)
                }
            }
        }
    }
    all_shows
    // tv_shows.clone()
}
