# iPhotoSync

iPhotoSync is a tool to migrate from iPhoto/Photos OSx application or to backup your library keeping organisational structure


## Examples

Backup the whole library into specified directory:
`iPhotoSync --backup_directory=/tmp/iPhotoSync --database=/Users/mlappo/Pictures/Photos Library.photoslibrary/`

Show the actions which will be done during backup:
`iPhotoSync --backup_directory=/tmp/iPhotoSync --database=/Users/mlappo/Pictures/Photos Library.photoslibrary/ --dry-run`

## Usage

```
iPhotoSync 0.1.0
Mikhail Lappo <miklelappo@gmail.com>
A iPhoto/Photo sync utility

USAGE:
    iPhotoSync [FLAGS] --backup_directory <backup_directory> --database <database>

FLAGS:
    -n, --dry_run    show what would have been transferred
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --backup_directory <backup_directory>    Select backup directory for iPhoto events
    -f, --database <database>                    Select iPhoto database
```
