use rusqlite::{params, Connection};
use std::path::PathBuf;

use directories::ProjectDirs;

use super::models::TestResult;

const HISTORY_LIMIT: i64 = 50;

const CREATE_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS test_results (
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
);";

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
        conn.execute_batch(CREATE_TABLE_SQL)
            .expect("tablo oluşturulamadı");
        temizle_gecmis(&conn);

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
        sinirla_gecmis(&self.conn);
    }

    pub fn getir_tum(&self) -> Vec<TestResult> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, tarih, mod, sure_sn, hedef_kelime_sayisi, wpm, cpm, dogruluk_yuzde, dogru_tus, yanlis_tus, zorluk FROM test_results ORDER BY id DESC LIMIT ?1")
            .unwrap();

        let rows = stmt
            .query_map(params![HISTORY_LIMIT], |row| {
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

fn temizle_gecmis(conn: &Connection) {
    // Aynı testin her karede tekrar kaydedilmiş kopyalarını temizle.
    let _ = conn.execute(
        "DELETE FROM test_results WHERE id NOT IN (
            SELECT MIN(id) FROM test_results
            GROUP BY tarih, mod, sure_sn, hedef_kelime_sayisi, wpm, cpm,
                     dogruluk_yuzde, dogru_tus, yanlis_tus, zorluk
        )",
        [],
    );

    // Hiç tuşa basılmadan oluşan anlamsız kayıtları kaldır.
    let _ = conn.execute(
        "DELETE FROM test_results WHERE dogru_tus = 0 AND yanlis_tus = 0",
        [],
    );

    sinirla_gecmis(conn);
}

fn sinirla_gecmis(conn: &Connection) {
    let _ = conn.execute(
        "DELETE FROM test_results WHERE id NOT IN (
            SELECT id FROM test_results ORDER BY id DESC LIMIT ?1
        )",
        params![HISTORY_LIMIT],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kayit_ekle(conn: &Connection, tarih: &str, wpm: f64, dogru_tus: i64) {
        conn.execute(
            "INSERT INTO test_results (tarih, mod, sure_sn, hedef_kelime_sayisi, wpm, cpm, dogruluk_yuzde, dogru_tus, yanlis_tus, zorluk)
             VALUES (?1, 'iki_el', 60, NULL, ?2, ?3, 100.0, ?4, 0, 'orta')",
            params![tarih, wpm, wpm * 5.0, dogru_tus],
        )
        .unwrap();
    }

    #[test]
    fn gecmis_tekrarlari_temizlenir_ve_sinirlanir() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(CREATE_TABLE_SQL).unwrap();

        kayit_ekle(&conn, "2026-01-01 10:00:00", 10.0, 10);
        kayit_ekle(&conn, "2026-01-01 10:00:00", 10.0, 10);
        kayit_ekle(&conn, "2026-01-01 10:00:00", 10.0, 10);
        kayit_ekle(&conn, "2026-01-01 10:01:00", 0.0, 0);

        for i in 0..HISTORY_LIMIT + 10 {
            kayit_ekle(
                &conn,
                &format!("2026-01-02 10:{:02}:00", i % 60),
                i as f64 + 1.0,
                10,
            );
        }

        temizle_gecmis(&conn);

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_results", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, HISTORY_LIMIT);

        let bos: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM test_results WHERE dogru_tus = 0 AND yanlis_tus = 0",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(bos, 0);

        let tekil: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM (
                    SELECT DISTINCT tarih, mod, sure_sn, hedef_kelime_sayisi, wpm, cpm,
                                    dogruluk_yuzde, dogru_tus, yanlis_tus, zorluk
                    FROM test_results
                )",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(tekil, count);
    }
}
