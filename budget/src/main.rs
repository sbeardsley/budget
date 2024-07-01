mod app;
mod cli;

use clap::Parser;
use inquire::{
    error::InquireResult,
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
};

fn main() -> InquireResult<()> {
    inquire::set_global_render_config(get_render_config());

    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Budget => {
            println!("Running budget command");
        }
    }
    Ok(())
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightRed);
    render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::LightYellow);
    render_config.selected_checkbox = Styled::new("☑").with_fg(Color::LightGreen);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}
