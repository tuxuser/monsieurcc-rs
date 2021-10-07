use std::path::PathBuf;

use monsieurcc::{
    api::Api,
    serde_json,
    schemas::RecipeType,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    /// Download recipes for various languages
    Recipes(RecipeOptions),
    /// Fetch download links for MC2 APK
    Apk,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Monsieur Cuisine Connect - Command Line interface",
    about = "Handle recipes and APK downloads"
)]
struct Opt {
    /// Subcommand
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
struct RecipeOptions {
    /// Optional output file
    #[structopt(short = "o", long = "output")]
    pub output_file: Option<PathBuf>,
    /// Desired language (e.g. "de", "en", "fr", "it", "es", "pl")
    #[structopt(short = "l", long = "lang")]
    language: String,
    /// Recipe type (e.g. "default", "live", "beta")
    #[structopt(short = "t", long = "type", default_value = "default")]
    recipe_type: RecipeType,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let api = Api::new("");

    match args.cmd {
        Command::Recipes(opts) => {
            let output_path = match opts.output_file.clone() {
                Some(fpath) => fpath,
                None => {
                    let filename = format!(
                        "recipes_{}_{}.json",
                        opts.language,
                        opts.recipe_type.to_string()
                    );
                    PathBuf::from(filename)
                }
            };

            println!("Downloading recipes ({:?})...", &opts);
            match api
                .get_recipes(Some(&opts.language), Some(opts.recipe_type.clone()))
                .await
            {
                Ok(recipes) => match serde_json::ser::to_string(&recipes) {
                    Ok(serialized) => {
                        println!("Saving recipes to {:?}", &output_path);
                        let res = std::fs::write(&output_path, serialized);
                        if res.is_err() {
                            return Err(format!(
                                "Failed to write Recipes to path: {:?}",
                                output_path
                            )
                            .into());
                        }
                    }
                    Err(e) => {
                        return Err(format!(
                            "Failed to serialize Recipes back into JSON, err: {}",
                            e
                        )
                        .into());
                    }
                },
                Err(_) => {
                    return Err(format!("Failed to fetch recipes with options: {:?}", opts).into());
                }
            }
        }
        Command::Apk => {
            println!("Fetching list of MC2 filenames...");
            let apks = api
                .get_apk_updates()
                .await
                .expect("Failed to download APK updates");

            println!("== MC2 APK links ==");
            apks.into_iter().enumerate().for_each(|(idx, name)| {
                println!("{}) {}", idx, name);
            });
        }
    }

    Ok(())
}
