use dirs::home_dir;
use rusqlite::{Connection, NO_PARAMS};
use crate::models::{CookieDataRaw, Result, GetChromeCookieError};

const COOKIES_FILE: &str = "Library/Application Support/Google/Chrome/Default/Cookies";

pub fn get_cookie(name: &Option<Vec<&str>>, host: &Option<&str>, limit: u32) -> Result<Vec<CookieDataRaw>> {
    // TODO: osごとにパス変える
    // Cookiesファイルの場所をHomeディレクトリから算出
    let mut cookies_file_path_buf = home_dir().unwrap();
    cookies_file_path_buf.push(&COOKIES_FILE);
    let cookies_file_path = cookies_file_path_buf.clone().into_os_string().into_string().unwrap();

    // ファイルの存在確認
    if !cookies_file_path_buf.as_path().exists() {
        return Err(GetChromeCookieError::new(format!("Cookie databese file `{}` is not found.", &cookies_file_path)))
    }

    // build sql
    let default_sql = "SELECT name,host_key,encrypted_value FROM cookies where 1 = 1";
    let where_name_statement = match name {
        None => "".to_string(),
        Some(n) => {
            let names = n.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(",");
            format!("and name in ({})", names).to_string()
        },
    };
    let where_host_key_statement = match host {
        None => "".to_string(),
        Some(h) => format!("and host_key like '%{}%'", h).to_string(),
    };
    let limit_statement = format!("limit {}", limit);
    let sql = format!("{} {} {} {};", default_sql, where_name_statement, where_host_key_statement, limit_statement);

    // DB接続
    let conn = Connection::open(cookies_file_path).map_err(|err| {
        GetChromeCookieError::new(format!("Fail to open database connection: {}", err.to_string()))
    })?;

    // クエリ発行
    let mut result = conn.prepare(sql.as_str()).map_err(|err| {
        GetChromeCookieError::new(format!("An Error occurred at executing queries: {}", err.to_string()))
    })?;

    // マッピング
    let values = result.query_map(NO_PARAMS, |row| {
        Ok(CookieDataRaw {
            name: row.get(0)?,
            host_key: row.get(1)?,
            encrypted_value: row.get(2)?,
        })
    }).map_err(|err| {
        GetChromeCookieError::new(format!("An Error occurred at mapping row data to object: {}", err.to_string()))
    })?;

    values.map(|v| {
        let raw = v.map_err(|err| {
            GetChromeCookieError::new(format!("Fail, an error has occurred at unwrapping cookie row data: {}", err.to_string()))
        })?;
        Ok(raw)
    }).collect::<Result<Vec<CookieDataRaw>>>()
}