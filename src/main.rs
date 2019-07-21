mod config;
mod delta;
mod edits;
mod style;
use ansi_term;
use atty;
use syntect::highlighting::{Color, FontStyle, Style};
use crate::delta::delta;
    let config = cli::process_command_line_arguments(&assets, &opt);

    if opt.show_colors {
        show_colors(&config);
        process::exit(0);
    }
        OutputType::from_mode(PagingMode::QuitIfOneScreen, Some(config.pager)).unwrap();
        &config,
fn show_colors(config: &config::Config) {
    println!(
        "delta \
         --theme=\"{theme}\" \
         --minus-color=\"{minus_color}\" \
         --minus-emph-color=\"{minus_emph_color}\" \
         --plus-color=\"{plus_color}\" \
         --plus-emph-color=\"{plus_emph_color}\"",
        theme = config.theme_name,
        minus_color = color_to_hex(config.minus_style_modifier.background.unwrap()),
        minus_emph_color = color_to_hex(config.minus_emph_style_modifier.background.unwrap()),
        plus_color = color_to_hex(config.plus_style_modifier.background.unwrap()),
        plus_emph_color = color_to_hex(config.plus_emph_style_modifier.background.unwrap()),
    )
}

fn color_to_hex(color: Color) -> String {
    let mut string = String::new();
    let style = Style {
        foreground: style::NO_COLOR,
        background: color,
        font_style: FontStyle::empty(),
    };
    paint::paint_text(
        &format!("#{:x?}{:x?}{:x?}", color.r, color.g, color.b),
        style,
        &mut string,
    )
    .unwrap();
    string.push_str("\x1b[0m"); // reset
    string
}

    if atty::is(atty::Stream::Stdin) {
        input = "\
diff --git a/tests/data/hello.c b/tests/data/hello.c
index 541e930..e23bef1 100644
--- a/tests/data/hello.c
+++ b/tests/data/hello.c
@@ -1,5 +1,5 @@
 #include <stdio.h>

 int main(int argc, char **argv) {
-    printf(\"Hello!\\n\");
+    printf(\"Hello world!\\n\");
 }"
        .to_string()
    } else {
        io::stdin().read_to_string(&mut input)?;
    }
    let mut config: config::Config;
    let style = ansi_term::Style::new().bold();
        if opt.light && !style::is_light_theme(theme) || opt.dark && style::is_light_theme(theme) {
        writeln!(stdout, "\nTheme: {}\n", style.paint(theme))?;
        config = cli::process_command_line_arguments(&assets, &opt);
            OutputType::from_mode(PagingMode::QuitIfOneScreen, Some(config.pager)).unwrap();
            &config,
        if style::is_light_theme(theme) {
        if !style::is_light_theme(theme) {