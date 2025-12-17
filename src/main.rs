mod args;

use crate::args::{Cli, Command, CommonArgs, EaseFn};
use clap::Parser;
use color_eyre::eyre;
use directories::ProjectDirs;
use hyprland::{
    error::HyprError,
    keyword::{Keyword, OptionValue},
};
use log::{debug, error, info, warn};
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::{
    fs::{self, OpenOptions},
    process::exit,
    thread,
    time::Duration,
};

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    let log_file = match ProjectDirs::from("com", "nouritsu", "hyprzoom") {
        Some(dirs) => match dirs.state_dir() {
            Some(state_dir) => {
                let state_dir = state_dir.to_path_buf();
                if let Err(e) = fs::create_dir_all(&state_dir) {
                    eprintln!(
                        "failed to create state directory at {}: {}",
                        state_dir.display(),
                        e
                    );
                    exit(1);
                }
                state_dir.join("hyprzoom.log")
            }
            None => {
                eprintln!("failed to determine state directory path");
                exit(1);
            }
        },
        None => {
            eprintln!("failed to locate project directories (unsupported platform?)");
            exit(1);
        }
    };

    let log_file_handle = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .unwrap_or_else(|e| {
            eprintln!("failed to open log file at {}: {}", log_file.display(), e);
            exit(1);
        });

    CombinedLogger::init(vec![WriteLogger::new(
        cli.log_level,
        Config::default(),
        log_file_handle,
    )])
    .unwrap_or_else(|e| {
        eprintln!("failed to initialize logger: {}", e);
        exit(1);
    });
    info!("log file: {}", log_file.display());

    info!("hyprzoom started");

    if let Err(e) = handle_command(cli.command) {
        error!("hyprzoom exited with error: {}", e);
        exit(1);
    }

    info!("hyprzoom exited successfully");
    Ok(())
}

fn handle_command(command: Command) -> Result<(), HyprError> {
    match command {
        Command::Zoom(args) => {
            info!("executing 'zoom' command");
            let CommonArgs {
                steps,
                duration,
                ztarget: target,
            } = args.common;

            debug!(
                "zoom parameters: steps={}, duration={:?}, target={}",
                steps, duration, target
            );

            let zout = ease_range(1.0, target, steps, args.ease);
            debug!("generated {} zoom steps from 1.0 to {}", zout.len(), target);

            apply_zooms(&zout, duration)?;
            info!("'zoom' command completed successfully");
        }
        Command::Inout(args) => {
            info!("executing 'inout' command (zoom in and out)");
            let CommonArgs {
                steps,
                duration,
                ztarget: target,
            } = args.common;
            let init = args.zinit;

            debug!(
                "inout parameters: steps={}, duration={:?}, init={}, target={}, zduration={:?}",
                steps, duration, init, target, args.zduration
            );

            let zin = ease_range(init, target, steps, args.in_ease);
            debug!(
                "generated {} zoom-in steps from {} to {}",
                zin.len(),
                init,
                target
            );

            let zdur = args.zduration;
            let zout = ease_range(target, init, steps, args.out_ease);
            debug!(
                "generated {} zoom-out steps from {} to {}",
                zout.len(),
                target,
                init
            );

            apply_zooms(&zin, duration)?;
            info!("zoom-in phase completed, sleeping for {:?}", zdur);
            thread::sleep(zdur);
            info!("starting zoom-out phase");
            apply_zooms(&zout, duration)?;
            info!("'inout' command completed successfully");
        }
    };
    Ok(())
}

fn apply_zooms(zs: &[f64], duration: Duration) -> Result<(), HyprError> {
    debug_assert!(!zs.is_empty()); /* cli parser ensures this */
    info!("applying {} zoom steps over {:?}", zs.len(), duration);
    let interval = Duration::from_secs_f64(duration.as_secs_f64() / zs.len() as f64);
    debug!("interval per step: {:?}", interval);

    for (i, &z) in zs.iter().enumerate() {
        debug!(
            "step {}/{}: setting zoom factor to {:.4}",
            i + 1,
            zs.len(),
            z
        );
        Keyword::set("cursor:zoom_factor", OptionValue::Float(z))?;
        thread::sleep(interval);
    }

    info!("all zoom steps applied successfully");
    Ok(())
}

fn ease_range(start: f64, end: f64, n: usize, ease_fn: EaseFn) -> Vec<f64> {
    debug!(
        "calculating ease range: start={}, end={}, steps={}",
        start, end, n
    );

    match n {
        0 => {
            warn!("ease_range called with 0 steps, returning empty vector");
            vec![]
        }
        1 => {
            debug!("ease_range called with 1 step, returning start value only");
            vec![start]
        }
        n => {
            let result: Vec<f64> = (0..n)
                .map(|i| i as f64)
                .map(|i| ease_fn(i, start, end - start, (n - 1) as f64))
                .collect();
            debug!("generated {} eased values", result.len());
            result
        }
    }
}
