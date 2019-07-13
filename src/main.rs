mod bat;
mod cli;
mod draw;
mod parse;
use crate::bat::assets::{list_languages, HighlightingAssets};
use crate::bat::output::{OutputType, PagingMode};
use crate::parse::delta;

    let opt = cli::Opt::from_args();
    let paint_config = cli::process_command_line_arguments(&assets, &opt);

    let mut output_type =
        OutputType::from_mode(PagingMode::QuitIfOneScreen, Some(paint_config.pager)).unwrap();
    let mut writer = output_type.handle().unwrap();
        &mut writer,
    let mut opt = cli::Opt::from_args();

        paint_config = cli::process_command_line_arguments(&assets, &opt);
        let mut output_type =
            OutputType::from_mode(PagingMode::QuitIfOneScreen, Some(paint_config.pager)).unwrap();
        let mut writer = output_type.handle().unwrap();

        delta(
            input.split("\n").map(String::from),
            &paint_config,
            &assets,
            &mut writer,
        )?;

#[cfg(test)]
mod tests {
    use super::*;

    use console::strip_ansi_codes;

    #[test]
    #[ignore]
    fn test_added_file() {
        let input = "\
commit d28dc1ac57e53432567ec5bf19ad49ff90f0f7a5
Author: Dan Davison <dandavison7@gmail.com>
Date:   Thu Jul 11 10:41:11 2019 -0400

    .

diff --git a/a.py b/a.py
new file mode 100644
index 0000000..8c55b7d
--- /dev/null
+++ b/a.py
@@ -0,0 +1,3 @@
+# hello
+class X:
+    pass";

        let expected_output = "\
commit d28dc1ac57e53432567ec5bf19ad49ff90f0f7a5
Author: Dan Davison <dandavison7@gmail.com>
Date:   Thu Jul 11 10:41:11 2019 -0400

    .

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
modified: a.py
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────────────────────────────────────

────────────────────────────────────────────────────────────────────────────────
 # hello
 class X:
     pass
";

        let mut opt = cli::Opt::from_args();
        opt.width = Some("variable".to_string());
        let assets = HighlightingAssets::new();
        let paint_config = cli::process_command_line_arguments(&assets, &opt);
        let mut writer: Vec<u8> = Vec::new();
        delta(
            input.split("\n").map(String::from),
            &paint_config,
            &assets,
            &mut writer,
        )
        .unwrap();
        let output = strip_ansi_codes(&String::from_utf8(writer).unwrap()).to_string();
        assert!(output.contains("\nadded: a.py\n"));
        if false {
            // TODO: hline width
            assert_eq!(output, expected_output);
        }
    }

}