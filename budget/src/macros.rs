// macro_rules! display {
//     ($($arg:tt)*) => {{
//         use owo_colors::OwoColorize;
//         println!(
//             "{}",
//             format!($($arg)*)
//                 .if_supports_color(owo_colors::Stream::Stdout, |text| text.bold())
//         );
//     }};
// }

// macro_rules! critical {
//     ($($arg:tt)*) => {{
//         use owo_colors::OwoColorize;
//         eprintln!(
//             "{}",
//             format!($($arg)*)
//                 .if_supports_color(owo_colors::Stream::Stderr, |text| text.bright_red())
//         );
//     }};
// }

// macro_rules! define_display_macro {
//     ($name:ident, $level:ident, $style:ident, $d:tt) => (
//         macro_rules! $name {
//             ($d($d arg:tt)*) => {{
//                 use owo_colors::OwoColorize;
//                 if tracing::Level::$level <= *$crate::app::verbosity() {
//                     eprintln!(
//                         "{}",
//                         format!($d($d arg)*)
//                             .if_supports_color(owo_colors::Stream::Stderr, |text| text.$style())
//                     );
//                 }
//             }};
//         }
//     );
// }

// define_display_macro!(trace, TRACE, underline, $);
// define_display_macro!(debug, DEBUG, italic, $);
// define_display_macro!(info, INFO, bold, $);
// define_display_macro!(success, INFO, bright_cyan, $);
// define_display_macro!(waiting, INFO, bright_magenta, $);
// define_display_macro!(warn, WARN, bright_yellow, $);
// define_display_macro!(error, ERROR, bright_red, $);
