use rusqlite::{Connection, Result, NO_PARAMS};

#[derive(Debug)]
pub struct Asset {
	album_name: String,
	dir: String,
	filename: String,
	original_filename: String,
}

pub fn get_db_assets(filename: &str) -> Result<()> {
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
        println!("Found asset {:?}", asset);
    }
    Ok(())
}