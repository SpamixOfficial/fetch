# General

## Example general section in config file

```toml
[general]
# Define the "default" art-file location
default_art = "~/.config/bluefetch/art/default"
art_directory = "./art" # Something custom!
...
```

## Parameters

| Key           | Type         | Optional           | Description                                                                                          |
| ------------- | ------------ | ------------------ | ---------------------------------------------------------------------------------------------------- |
| default_art   | String(Path) | :white_check_mark: | The default art file to use, optional. Default is your OS                                            |
| art_directory | String(Path) | :white_check_mark: | The directory to use for art, optional. Default is $HOME/.config/bluefetch/art or /etc/bluefetch/art |
