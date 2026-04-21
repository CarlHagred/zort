# Zort

Zort is a small script for sorting files within a directory based on a configuration file.

When run the first time zort will create its config file in:

Windows:

C:\Users\USER\AppData\Roaming\carlhagred\zort\

Linux/Mac:

/home/USER/.config/zort/

### Config File

The config file is a normal json file.
Add the extension as the key and then the target directory as the value.

The default config file looks like this:

```
{
  "jpg": "Images",
  "png": "Images",
  "gif": "Images",
  "pdf": "Documents",
  "doc": "Documents",
  "txt": "Documents",
  "docx": "Documents",
  "csv": "Sheets",
  "xlsx": "Sheets",
  "xlx": "Sheets",
  "ods": "Sheets",
  "mp3": "Music",
  "wav": "Music",
  "flac": "Music",
  "aac": "Music",
  "mp4": "Video",
  "mov": "Video",
  "avi": "Video",
  "mkv": "Video",
  "wmv": "Video",
  "zip": "Zipped",
  "rar": "Zipped",
  "7z": "Zipped",
  "gz": "Zipped"
}
```

## Get started

Get started by running:

```
cargo run -- _file_path_
```
