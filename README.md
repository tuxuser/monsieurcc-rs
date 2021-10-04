# Monsieur Cuisine connect instrumentation

[![Crates.io](https://img.shields.io/crates/v/monsieurcc.svg)](https://crates.io/crates/monsieurcc)
[![Docs.rs](https://docs.rs/monsieurcc/badge.svg)](https://docs.rs/monsieurcc)
[![CI](https://github.com/tuxuser/monsieurcc-rs/workflows/CI/badge.svg)](https://github.com/tuxuser/monsieurcc-rs/actions)

## MCC CLI

### Usage
```
Monsieur Cuisine Connect - Command Line interface 0.1.0
Handle recipes and APK downloads

USAGE:
    mcc_cli [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    apk        Fetch download links for MC2 APK
    help       Prints this message or the help of the given subcommand(s)
    recipes    Download recipes for various languages
```

### Fetch recipes

Usage

```
$ ./mcc_cli -o recipes_new.json recipes -l de -t live
Downloading recipes (RecipeOptions { language: "de", recipe_type: Live })...
Saving recipes to "recipes_new.json"
```

If ommitting argument `-o` the output filename is auto-generated from language and recipe type args.

## Disclaimer

This project is in no way endorsed by or affiliated with Lidl, Silvercrest, Silpion or any associated subsidiaries, logos or trademarks.
For any possible matters in that regard, please get in touch by opening an issue on this repository, thank you.
