## About
A simple yet feature rich zoom utility for Hyprland.

## Showcase
[![demo](https://img.youtube.com/vi/RzgMqkkmSwg/0.jpg)](https://youtube.com/watch?v=RzgMqkkmSwg)

## Installation
Currently, the only installation method is building from source:
```
cargo install --git https://github.com/nouritsu/hyprzoom
```
> Or manually `git clone https://github.com/nouritsu/hyprzoom.git hyprzoom && cargo install --path hyprzoom`.

Note that this requires you to have `cargo` installed and its bin directory in `$PATH`. You may also `cargo build --release` and copy the binary at `target/release/hyprzoom` to a directory in your `$PATH`.


## Usage
### Help
The following commands print the help message:
```
hyprzoom help
```
```
hyprzoom --help
```
```
hyprzoom -h
```

Help for subcommands `zoom`/`z` and `inout`/`in_out`/`io`:
```
hyprzoom zoom --help # can use z instead of zoom
```
```
hyprzoom inout -h # can use in_out or io instead of inout
```
### Common
Both subcommands (`zoom` and `inout`) support the following options:
```
-s, --steps <steps>           Number of steps for the zoom animation      [default: 15]
-d, --duration <duration>     Duration of the zoom animation              [default: 250ms]
```
Steps is an integer greater than 0. A value of 1 would be an instant zoom, without animations.

Duration can be a human-readable string. For example `250ms` for 250 milliseconds or `2s` for 2 seconds. Obvious footgun: `1d`.

### Zoom To
The `zoom` (or `z`) subcommand zooms to a provided `ztarget`.
```
hyprzoom zoom <ztarget>
```

In addition to steps + duration, you can also specify the ease function using
```
hyprzoom zoom <target> --ease <ease_function>
```

The ease function is a string in format (case-insensitive) `fn:qualifier` where
- `fn` denotes the specific function, such as `quad`, `lin`, `elastic` etc.
- `qualifier` denotes the specific qualifier, such as `in` (or `i`), `out` (or `o`), `inout` (or `io`)

By default, it will
- use the `quad:in` ease function
- animate 15 frames over 0.25 seconds (60 fps, I think)

### Zoom In/Out
The `inout` (or `in_out`, `io`) zooms to a provided `ztarget` and zooms out to the initial zoom level.
```
hyprzoom inout <ztarget>
```

As with the zoom function, you can specify the ease function(s, this time).
```
hyprzoom inout <ztarget> --in-ease <ease_function_in> --out-ease <ease_function_out>
```
> these options are intentionally named so, because `ease-in` and `ease-out` is misleading

The ease functions follow the same format as described in the previous section.

In addition to steps + duration and the ease functions, you can also specify the duration spent zoomed in as

```
hyprzoom inout <ztarget> --zduration <zduration>
```

It follows the same format as described in the previous section.

By default, it will
- use the `quad:in` ease function for zooming in
- use the `quad:out` ease function for zooming out
- animate 15 frames over 0.25 seconds of zooming in
- wait 1 second zoomed in
- animate 15 frames over 0.25 seconds of zooming out

## Notice
A majority of this tool's functionality can be achieved using hyprland configuration.
Rather than scrapping this project, it would be better to extend it to other compositors and add more configuration options.

A configuration for `animations` like below can be used (highlighted by u/SOA-determined on Reddit)
```
animations {
    enabled = true
    bezier = easeOut, 0.16, 1, 0.3, 1
    animation = zoomFactor, 1, 6, easeOut  # 6ds = 600ms (not 0.6!)
  }
  # Zoom binds
  bind = Ctrl+Super, mouse:274, exec, hyprctl keyword cursor:zoom_factor 3.0
  bindr = Ctrl+Super, mouse:274, exec, hyprctl keyword cursor:zoom_factor 1.0
}
```

## Acknowledgements
- A similar tool [hypr-zoom](https://github.com/FShou/hypr-zoom) does exist but it seems to be abandoned (unmerged PRs)
- The demo showcases a waybar setup adapted from [mechabar](github.com/sejjy/mechabar)

## Plans
- animation caching (idek if this is worth it)
- nix flake
- toggle zoom (perhaps with an environment variable)
