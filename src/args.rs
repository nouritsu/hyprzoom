use clap::{Args, Parser, Subcommand};
use easer::functions::{self, Easing};
use hyprland::keyword::{Keyword, OptionValue};
use log::LevelFilter;
use std::{path::PathBuf, process::exit, time::Duration};

const MIN_DURATION: Duration = Duration::from_millis(1);
const MIN_DURATION_UNIT: &str = "ms";

const MIN_STEPS: usize = 1; /* instant, no animation */

pub type EaseFn = fn(f64, f64, f64, f64) -> f64;

macro_rules! easefn {
    ($fn_type:ident, $qualifier:expr) => {
        match $qualifier {
            "i" | "in" => Ok(functions::$fn_type::ease_in as EaseFn),
            "o" | "out" => Ok(functions::$fn_type::ease_out as EaseFn),
            "io" | "inout" | "in_out" => Ok(functions::$fn_type::ease_in_out as EaseFn),
            q => Err(format!(
                "Invalid easing qualifier: '{}'. Expected 'in', 'out', or 'inout'",
                q
            )),
        }
    };
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Log level for the application
    #[arg(long, default_value = "info", value_parser = parse_log_level)]
    pub log_level: LevelFilter,

    /// Path to the log file
    #[arg(long)]
    pub log_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct CommonArgs {
    /// Number of steps for the zoom animation
    #[arg(short, long, default_value_t = 15, value_parser = parse_steps)]
    pub steps: usize,

    /// Duration of the zoom animation
    #[arg(short, long, default_value = "250ms", value_parser = parse_duration)]
    pub duration: Duration,

    /// Target Zoom (cursor:zoom_factor)
    #[arg(required = true)]
    pub ztarget: f64,
}

#[derive(Subcommand)]
pub enum Command {
    /// Zoom to a specific zoom level
    #[command(visible_alias = "z")]
    Zoom(ZoomToArgs),

    /// Zoom in and out
    #[command(visible_alias = "in_out", visible_alias = "io")]
    Inout(ZoomInOutArgs),
}

#[derive(Args)]
pub struct ZoomToArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Ease Function
    #[arg(long, default_value = "quad:in", value_parser = parse_ease)]
    pub ease: EaseFn,
}

#[derive(Args)]
pub struct ZoomInOutArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Initial Zoom (cursor:zoom_factor)
    #[arg(long, default_value_t = default_init())]
    pub zinit: f64,

    /// Zoom In Ease Function
    #[arg(long, default_value = "quad:in", value_parser = parse_ease)]
    pub in_ease: EaseFn,

    /// Zoom Out Ease Function
    #[arg(long, default_value = "quad:out", value_parser = parse_ease)]
    pub out_ease: EaseFn,

    /// Zoomed-In Duration
    #[arg(long, default_value = "1s", value_parser = parse_duration)]
    pub zduration: Duration,
}

/* Argument Parsers */

fn parse_log_level(s: &str) -> Result<LevelFilter, String> {
    s.parse::<LevelFilter>()
        .map_err(|_| format!("invalid log level: {}", s))
}

fn parse_steps(s: &str) -> Result<usize, String> {
    s.parse::<usize>()
        .map_err(|_| format!("invalid steps, must be number: {}", s))
        .and_then(|v| {
            if v < MIN_STEPS {
                Err(format!(
                    "invalid steps, must be at least {}: {}",
                    MIN_STEPS, s
                ))
            } else {
                Ok(v)
            }
        })
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let duration =
        parse_duration::parse(s).map_err(|err| format!("Invalid duration format: {}", err))?;

    if duration < MIN_DURATION {
        return Err(format!(
            "Duration must be at least {}{}, got {}",
            MIN_DURATION.as_millis(),
            MIN_DURATION_UNIT,
            s
        ));
    }

    Ok(duration)
}

fn parse_ease(s: &str) -> Result<EaseFn, String> {
    let s = s.trim().to_lowercase();
    let (ease_fn_str, qualifier) = s.split_once(':').ok_or_else(|| {
        format!(
            "Invalid format. Expected 'easefn:qualifier' (e.g., 'lin:in'), got '{}'",
            s
        )
    })?;

    let result = match ease_fn_str {
        // Physical
        "back" => easefn!(Back, qualifier),
        "ela" | "elastic" => easefn!(Elastic, qualifier),
        "bounce" => easefn!(Bounce, qualifier),

        // Polynomial
        "lin" | "linear" => easefn!(Linear, qualifier),
        "quad" | "quadratic" => easefn!(Quad, qualifier),
        "cube" | "cubic" => easefn!(Cubic, qualifier),
        "quart" | "quartic" => easefn!(Quart, qualifier),
        "quint" | "quintic" => easefn!(Quint, qualifier),

        // Other Math
        "exp" | "expo" | "exponential" => easefn!(Expo, qualifier),
        "sin" | "sine" => easefn!(Sine, qualifier),
        "circ" | "circle" | "circular" => easefn!(Circ, qualifier),

        _ => Err(format!("Invalid easing function: '{}'", ease_fn_str)),
    };

    result
}

/* Default Values */

fn default_init() -> f64 {
    match Keyword::get("cursor:zoom_factor") {
        Ok(Keyword {
            value: OptionValue::Float(v),
            ..
        }) => v,
        Ok(Keyword { value, .. }) => {
            eprintln!(
                "invalid value for 'cursor:zoom_factor' (must be float): {}",
                value
            );
            exit(1)
        }
        Err(e) => {
            eprintln!("error getting 'cursor:zoom_factor': {}", e);
            exit(1)
        }
    }
}
