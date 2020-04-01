use rusqlite::{Connection, Result, NO_PARAMS};

#[derive(Debug)]
pub struct Asset {
	pub album_name: String,
	pub dir: String,
	pub filename: String,
	pub original_filename: String,
}

impl Clone for Asset {
    fn clone(&self) -> Asset {
        Asset {
            album_name: self.album_name.clone(),
            dir: self.dir.clone(),
            filename: self.filename.clone(),
            original_filename: self.original_filename.clone()
        }
    }
}

pub fn get_db_assets(filename: &str, mut ret: Vec<Asset>) -> Result<Vec<Asset>> {
    let filename = format!("{}/database/Photos.sqlite", filename);
    let conn = Connection::open(filename)?;
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
    for asset in backup_table {
		let asset = &asset?;
        ret.push(asset.clone());
    }
    Ok(ret)
}