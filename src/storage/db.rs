use rusqlite::{params, Connection};
use std::path::PathBuf;

use directories::ProjectDirs;

use super::models::TestResult;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open() -> Self {
        let path = Self::db_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(&path).expect("veritabanı açılamadı");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS test_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tarih TEXT NOT NULL,
                mod TEXT NOT NULL,
                sure_sn INTEGER NOT NULL,
                hedef_kelime_sayisi INTEGER,
                wpm REAL NOT NULL,
                cpm REAL NOT NULL,
                dogruluk_yuzde REAL NOT NULL,
                dogru_tus INTEGER NOT NULL,
                yanlis_tus INTEGER NOT NULL,
                zorluk TEXT NOT NULL
            );",
        )
        .expect("tablo oluşturulamadı");

        Self { conn }
    }

    pub fn kaydet(&self, result: &TestResult) {
        self.conn
            .execute(
                "INSERT INTO test_results (tarih, mod, sure_sn, hedef_kelime_sayisi, wpm, cpm, dogruluk_yuzde, dogru_tus, yanlis_tus, zorluk) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    result.tarih,
                    result.modu,
                    result.sure_sn,
                    result.hedef_kelime_sayisi,
                    result.wpm,
                    result.cpm,
                    result.dogruluk_yuzde,
                    result.dogru_tus,
                    result.yanlis_tus,
                    result.zorluk,
                ],
            )
            .expect("kayıt başarısız");
    }

    pub fn getir_tum(&self) -> Vec<TestResult> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, tarih, mod, sure_sn, hedef_kelime_sayisi, wpm, cpm, dogruluk_yuzde, dogru_tus, yanlis_tus, zorluk FROM test_results ORDER BY id DESC LIMIT 50")
            .unwrap();

        let rows = stmt
            .query_map([], |row| {
                Ok(TestResult {
                    id: row.get(0)?,
                    tarih: row.get(1)?,
                    modu: row.get(2)?,
                    sure_sn: row.get(3)?,
                    hedef_kelime_sayisi: row.get(4)?,
                    wpm: row.get(5)?,
                    cpm: row.get(6)?,
                    dogruluk_yuzde: row.get(7)?,
                    dogru_tus: row.get(8)?,
                    yanlis_tus: row.get(9)?,
                    zorluk: row.get(10)?,
                })
            })
            .unwrap();

        rows.filter_map(|r| r.ok()).collect()
    }

    fn db_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "typing_trainer") {
            proj_dirs.data_dir().join("stats.sqlite")
        } else {
            PathBuf::from("stats.sqlite")
        }
    }
}
