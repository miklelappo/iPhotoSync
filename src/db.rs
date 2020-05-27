use rusqlite::{Connection, Result, NO_PARAMS};
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Asset {
    pub album_name: String,
    pub dir: String,
    pub filename: String,
    pub original_filename: String,
}

pub fn get_db_assets(filename: &Path) -> Result<Vec<Asset>> {
    let filename = filename.join("database").join("Photos.sqlite");
    let conn = Connection::open(filename)?;
    // Get all assets
    let mut backup_table_statement = conn.prepare("
        select ZGENERICALBUM.ZTITLE, ZDIRECTORY, ZFILENAME, ZORIGINALFILENAME
        from Z_26ASSETS 
        inner join ZGENERICASSET ON Z_26ASSETS.Z_34ASSETS = ZGENERICASSET.Z_PK
        inner join ZGENERICALBUM ON Z_26ASSETS.Z_26ALBUMS = ZGENERICALBUM.Z_PK
        inner join ZADDITIONALASSETATTRIBUTES ON Z_26ASSETS.Z_34ASSETS = ZADDITIONALASSETATTRIBUTES.ZASSET"
    )?;
    let backup_table = backup_table_statement.query_map(NO_PARAMS, |row| {
        Ok(Asset {
            album_name: row.get(0)?,
            dir: row.get(1)?,
            filename: row.get(2)?,
            original_filename: row.get(3)?,
        })
    })?;

    // Extract iPhotoEvents album paths (folders)
    let mut albums_statement = conn.prepare("
        select a.Z_PK as \"PrivateKey\", a.ZTITLE as \"AlbumName\", b.ZTITLE as \"AlbumParent\" from ZGENERICALBUM a
        inner join ZGENERICALBUM b on a.ZPARENTFOLDER = b.Z_PK where AlbumName NOTNULL
    ")?;
    let mut albums_hash: HashMap<String, String> = HashMap::new();
    let mut albums_rows = albums_statement.query(NO_PARAMS)?;
    while let Some(row) = albums_rows.next()? {
        albums_hash.insert(
            row.get(1)?,                    //album name
            row.get(2).unwrap_or_default(), //parent folder name if exists
        );
    }

    let mut ret = Vec::new();
    for asset in backup_table {
        let asset = &mut asset?;
        loop {
            if let Some(parent) = albums_hash.get(&asset.album_name) {
                if parent.is_empty() {
                    break;
                }
                asset.album_name = format!("{}/{}", &parent, asset.album_name);
            }
        }
        ret.push(asset.clone());
    }
    Ok(ret)
}
