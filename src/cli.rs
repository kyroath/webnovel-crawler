use clap::crate_version;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input};

use crate::info::*;

static LINE_EQ: &str =
    "================================================================================";
static LINE_DASH: &str =
    "--------------------------------------------------------------------------------";

pub fn init_console(url: Option<&str>) -> FictionInfo {
    let term = Term::stdout();
    term.set_title("Webnovel Crawler");

    let url = if let Some(url) = url {
        info!("URL inputted via arguments");
        url.to_string()
    } else {
        info!("URL argument not set");

        let width = LINE_EQ.len();
        term.write_line(LINE_EQ).expect("Error writing to terminal");

        let name = "Webnovel Crawler";
        let app_name_length = format!("{} #{}", name, crate_version!()).len();
        let app_name = format!(
            "{:01$}{2} {3}",
            " ",
            ((width - app_name_length) / 2),
            style(name).cyan(),
            style(format!("#{}", crate_version!())).yellow()
        );

        term.write_line(app_name.as_str())
            .expect("Error writing to terminal");
        term.write_line(LINE_DASH)
            .expect("Error writing to terminal");

        let url: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("-> Enter the fiction URL")
            .interact()
            .unwrap();

        url
    };

    FictionInfo::from_url(url)
}
