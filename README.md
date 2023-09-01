# Celeste Berry Indicator
This is a simple command-line utility to take a Celeste save file and convert it into an image containing information about the player's progress.

Basic usage is: `celeste-berry-indicator savefile.celeste output_image.bmp`

To avoid potential spoilers, use the `--no-spoilers` flag (or `/no-spoilers` on Windows).

### __WARNING__: potential spoilers below

## Advanced usage

This utility supports commands to customize the generated output, and to select save files to render more easily. Below are all the currently supported commands:

```
--help: prints a list of available commands.
--deaths: renders the save file's death count (off by default).
--id={0..2}: render the save with the specified ID. The ID range is not validated, so mods which allow extended save files can still be used with this command.
--no-hearts: only render golden berries, even if a chapter has already been completed.
--no-spoilers: hide uncollected items from incomplete chapters.
--output={file}: output to a specified file (only .bmp and .qoi image files are supported currently).
--spacing: add spacing between adjacent hearts.
```

On Windows, replace all double-dashes (`--`) with a forward slash (`/`).