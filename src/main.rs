use clap::Parser;
use colored::*;
use std::fs;

/// Represents the command line arguments.
#[derive(Parser, Debug)]
#[command(
    author = "HirschBerge",
    version = "v1.0.1",
    about = "Parses Plex or Jellyfin Libraries."
)]
struct Args {
    #[arg(short, long, default_value = "/mnt/NAS/Anime/")]
    path: String,
    #[arg(short, long, default_value = "")]
    title: String,
}

/// Represents a TV show.
#[derive(Debug, Clone)]
struct Show {
    title: String,
    season_cnt: u8,
    season_names: Vec<Season>,
}

/// Represents a season of a TV show.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Season {
    season_of: String,
    season_title: String,
    epi_count: u16,
}

#[allow(unused_assignments)]
impl Show {
    /**
    Adds a season to the Show.

    # Example
    ```
    let mut show = Show { title: "Example Show".to_string(), season_cnt: 0, season_names: vec![] };
    let season = Season { season_of: "Example Show".to_string(), season_title: "Season 1".to_string(), epi_count: 10 };
    show.add_season(season);
    ```
    */
    fn add_season(&mut self, szn: Season) {
        self.season_names.push(szn);
    }

    /**
    Prints the show details.

    # Example
    ```
    let show = Show { title: "Example Show".to_string(), season_cnt: 2, season_names: vec![] };
    show.print();
    ```
    */
    fn print(self) {
        println!("{}", self.title.purple());
        let mut szn_cnt_string: String = "".to_string();
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

    /**
    Returns a new struct of type Show with seasons sorted by title.

    # Example
    ```
    let show = Show { title: "Example Show".to_string(), season_cnt: 2, season_names: vec![] };
    let sorted_show = show.sort();
    ```
    */
    fn sort(&self) -> Show {
        let mut sorted_seasons = self.season_names.clone();
        sorted_seasons.sort_by(|a, b| a.season_title.cmp(&b.season_title));
        Show {
            title: self.title.clone(),
            season_cnt: self.season_cnt,
            season_names: sorted_seasons,
        }
    }
}

/**
Builds TV show data from a given path.

# Example
```
let root_path = "/path/to/your/library/".to_string();
let results = build_tv_show_data(root_path);
```
*/
fn build_tv_show_data(root_path: String, search: String) -> Vec<Show> {
    let mut all_shows: Vec<Show> = vec![];
    fs::read_dir(root_path)
        .expect("Failed to read root directory")
        .filter_map(|show_entry| show_entry.ok())
        .filter(|show_dir| show_dir.file_type().map_or(false, |ft| ft.is_dir()))
        .filter_map(|show_dir| {
            show_dir.file_name().into_string().ok().map(|show_name| {
                let mut show = Show {
                    title: show_name.clone(),
                    season_cnt: fs::read_dir(show_dir.path())
                        .expect("Failed to read show directory")
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_dir()))
                        .count() as u8,
                    season_names: vec![],
                };
                fs::read_dir(show_dir.path())
                    .expect("Failed to read show directory")
                    .filter_map(|season_entry| season_entry.ok())
                    .filter(|season_dir| season_dir.file_type().map_or(false, |ft| ft.is_dir()))
                    .map(|season_dir| {
                        let episode_count = season_dir
                            .path()
                            .read_dir()
                            .expect("Failed to read season directory")
                            .filter_map(|entry| entry.ok())
                            .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
                            .count() as u16;
                        let season_number = season_dir.file_name().to_string_lossy().to_string();
                        let szn = Season {
                            season_of: show_name.clone(),
                            season_title: season_number,
                            epi_count: episode_count,
                        };
                        show.add_season(szn);
                    })
                    .for_each(|_| ()); // Using map, so we need to consume the iterator
                if show.title.as_str().contains(&search) {
                    Some(show)
                } else {
                    None
                }
            })
        })
        .for_each(|show| {
            if let Some(s) = show {
                all_shows.push(s)
            }
        });
    all_shows
}

fn main() {
    let args = Args::parse();
    let root_path = args.path; // Change this to your root directory
    let mut results = build_tv_show_data(root_path, args.title);
    results.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    let mut count = 0;
    for show in results {
        let sorted_show = show.sort();
        sorted_show.print();
        count += 1;
    }
    let totals = format!("Total Shows parsed: {count}").bright_blue();
    println!("{totals}");
}
