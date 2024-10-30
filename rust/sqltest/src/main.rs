mod signal;
use time::macros::{offset, time};
use chrono::format::format;
use chrono::NaiveDateTime; // 处理 数据库中 datetime 类型
use dotenvy::dotenv;
use futures::executor::block_on;
use rust_decimal::Decimal; // 处理 数据库中 decimal 类型
use serde::{Deserialize, Serialize};
use serde_json::Value; // 处理 数据库中 json 类型
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{types::time::OffsetDateTime, Type};
use sqlx::{types::Json, FromRow};
use sqlx::{Execute, MySql, QueryBuilder, Row}; // get 方法的 trait
use std::{env, result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("i come here");
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("举报系统发生了点小问题，请稍后再试")]
    Database(#[from] sqlx::Error),

    #[error("最多只能上传 3 张截图")]
    TooManyScreenshots,

    #[error("举报内容最多 200 字")]
    TooLongComment,

    #[error("截图链接不合法")]
    InvalidScreenshotUrl,

    #[error("您今天已经举报过该玩家了")]
    AlreadyReported,

    #[error("您今天已经举报过 3 个玩家了")]
    TooManyReports,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Screenshots(Vec<String>);

#[derive(Deserialize, Serialize, Debug, sqlx::Encode, sqlx::Decode)]
#[sqlx(type_name = "reason", rename_all = "snake_case")]
pub enum Reason {
    /// 辱骂
    VerbalAbuse,
    /// 挂机
    AwayFromKeyboard,
    /// 作弊
    Cheating,
    /// 干扰正常游戏
    DisruptiveBehavior,
    /// 非法昵称
    IllegalName,
    /// 其他
    Other,
}

impl Type<sqlx::MySql> for Reason {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <str as Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <str as Type<sqlx::MySql>>::compatible(ty)
    }
}

/// 举报记录状态
#[derive(Deserialize, Serialize, Debug, sqlx::Encode, sqlx::Decode)]
#[sqlx(type_name = "state")]
pub enum ReportState {
    // 初始状态
    Init,
    // 处理中
    Handling,
    // 处理完成
    Complete,
    // 无效举报（无需处理）
    Invalid,
}

impl Type<sqlx::MySql> for ReportState {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <str as Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <str as Type<sqlx::MySql>>::compatible(ty)
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewReport {
    /// 举报者
    pub reporter_id: String,
    /// 被举报者
    pub reported_id: String,
    /// 举报原因
    pub reason: Reason,
    /// 举报说明
    pub comment: String,

    /// 举报截图，可能包含多张图片，最多 3 张
    pub screenshots: Vec<String>,

    /// 业务自定义数据
    pub data: Option<Value>,

    // 状态
    pub state: ReportState,
    pub game_id: String,
    pub scene: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Report {
    pub id: i32,
    /// 举报者
    pub reporter_id: String,
    /// 被举报者
    pub reported_id: String,
    /// 游戏id
    pub game_id: String,
    /// 举报原因
    pub reason: Reason,
    /// 举报说明
    pub comment: String,
    // 来源场景
    pub scene: String,
    // 举报记录状态
    pub state: ReportState,
    /// 举报截图，可能包含多张图片
    pub screenshots: Json<Screenshots>,
    /// 举报时间
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    // 更新时间
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

pub const FIELDS: &'static str =
    "reporter_id,reported_id,game_id,scene,reason,comment,screenshots,state";
pub const TABLE_NAME: &'static str = "reports";

async fn testInsert(db: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let new: NewReport = NewReport {
        reporter_id: "128".to_string(),
        reported_id: "1305".to_string(),
        reason: Reason::AwayFromKeyboard,
        comment: "".to_string(),
        screenshots: vec![],
        data: None,
        state: ReportState::Init,
        game_id: "124".to_string(),
        scene: "".to_string(),
    };

    let sql = format!("INSERT INTO reports ({}) VALUES(?,?,?,?,?,?,?,?)", FIELDS);

    let new_id = sqlx::query(&sql)
        .bind(new.reporter_id)
        .bind(new.reported_id)
        .bind(new.game_id)
        .bind(new.scene)
        .bind(new.reason)
        .bind(new.comment)
        .bind(Json(new.screenshots))
        .bind(new.state)
        .execute(db)
        .await?
        .last_insert_id();

    println!("{}", new_id);
    Ok(())
}

async fn testUpdate(db: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let new: NewReport = NewReport {
        reporter_id: "129".to_string(),
        reported_id: "1305".to_string(),
        reason: Reason::VerbalAbuse,
        comment: "".to_string(),
        screenshots: vec![],
        data: None,
        state: ReportState::Complete,
        game_id: "124".to_string(),
        scene: "".to_string(),
    };

    let sql = format!(
        "UPDATE {} SET reporter_id = ?, reported_id = ?, game_id = ?, scene = ?, reason = ?, comment = ?, screenshots = ?, state = ? WHERE id = ?",
            TABLE_NAME
        );

    let data = sqlx::query(&sql)
        .bind(new.reporter_id)
        .bind(new.reported_id)
        .bind(new.game_id)
        .bind(new.scene)
        .bind(new.reason)
        .bind(new.comment)
        .bind(Json(new.screenshots))
        .bind(new.state)
        .bind(1)
        .execute(db)
        .await?
        .rows_affected();

    Ok(())
}

async fn testFind(db: &sqlx::MySqlPool) -> Result<Report, Error> {
    let field = "id,reporter_id,reported_id,game_id,scene,reason,comment,screenshots,state,created_at,updated_at";
    let sql = format!("SELECT {} from {} WHERE id= ?", field, TABLE_NAME);

    let data = sqlx::query_as::<_, Report>(&sql)
        .bind(1)
        .fetch_one(db)
        .await?;

    Ok(data)
}

#[derive(Default)]
pub struct ReportQuery {
    // 游戏id
    pub game_id: Option<String>,
    // 举报人
    pub reporter_id: Option<String>,
    // 被举报人
    pub reported_id: Option<String>,
    // 开始时间
    pub from_time: Option<OffsetDateTime>,
    // 结束时间
    pub to_time: Option<OffsetDateTime>,
    // id列表
    pub ids: Option<Vec<i32>>,
    // 状态列表
    pub states: Option<Vec<ReportState>>,
    // 举报原因列表
    pub reasons: Option<Vec<Reason>>,

    // 分页
    pub page: i32,

    pub per_page: i32,
}

async fn testQuery(db: &sqlx::MySqlPool) -> Result<Vec<Report>, Error> {
    let datetime_str = "2024-05-15 13:51:10";

    let naive_date_time = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S");    
    let datetime = match naive_date_time {
        Ok(time)=>{
            match time::OffsetDateTime::from_unix_timestamp(time.and_utc().timestamp()) {
                Ok(result) => {
                    let date = result.date();
                    println!("date:{}", date);
                    Some(result)
                },
                Err(_)=>None,
            }
        },
        Err(error)=> {
            None
        }
    };



    let query = ReportQuery {
        game_id: None,
        reporter_id: None,
        reported_id: None,
        from_time: None,
        to_time: Some(time::OffsetDateTime::now_utc()),
        ids: None,
        ..Default::default()
    };

    let field = "id,reporter_id,reported_id,game_id,scene,reason,comment,screenshots,state,created_at,updated_at";
    let sql = format!("SELECT {} from {} WHERE 1 = 1", field, TABLE_NAME);

    let mut query_builder: QueryBuilder<MySql> = sqlx::QueryBuilder::new(&sql);
    let mut condition = false;
    if let Some(gameid) = query.game_id {
        query_builder.push(" AND game_id = ");
        query_builder.push_bind(gameid);
        condition = true;
    }

    if let Some(reporter) = query.reporter_id {
        query_builder.push(" AND reporter_id = ");
        query_builder.push_bind(reporter);
        condition = true;
    }

    if let Some(reported) = query.reported_id {
        query_builder.push(" AND reported_id = ");
        query_builder.push_bind(reported);
        condition = true;
    }

    if let Some(fromtime) = query.from_time {
        query_builder.push(" AND created_at >= ");
        query_builder.push_bind(fromtime);
        condition = true;
    }

    if let Some(totime) = query.to_time {
        query_builder.push(" AND created_at < ");
        query_builder.push_bind(totime);
        condition = true;
    }

    let ids = query.ids.unwrap_or(vec![]);
    if ids.len() != 0 {
        query_builder.push(" AND id in ");

        query_builder.push_tuples(ids.iter(), |mut b, item| {
            b.push_bind(item);
        });
    }

    let states = query.states.unwrap_or(vec![]);
    if states.len() != 0 {
        query_builder.push(" AND state in ");
        query_builder.push_tuples(states.iter(), |mut b, item| {
            b.push_bind(item);
        });
    }

    let reasons = query.reasons.unwrap_or(vec![]);
    if reasons.len() != 0 {
        query_builder.push(" AND reason in ");
        query_builder.push_tuples(reasons.iter(), |mut b, item| {
            b.push_bind(item);
        });
    }

    let querybuild = query_builder.build();
    let temp = querybuild.sql();

    println!("sql:{}", temp);

    let data = querybuild
        .try_map(|row| Report::from_row(&row))
        .fetch_all(db)
        .await?;
    Ok(data)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let offset_date_time = OffsetDateTime::now_utc()
    .to_offset(offset!(+8))
    .replace_time(time!(00:00));
    println!("{}", time::OffsetDateTime::now_utc());
    println!("OffsetDateTime: {}", offset_date_time);


    let url = "mysql://root:root@localhost:3306/risk-control";
    let pool = {
        let pool = sqlx::MySqlPool::connect(&url).await?;
        // run migrations
        sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
        pool
    };

    match block_on(testQuery(&pool)) {
        Ok(_) => {}
        Err(error) => {
            println!("error:{:?}", error);
        }
    };
    let pool = sqlx::MySqlPool::connect(&url).await?;

    let rows: Vec<MySqlRow> = sqlx::query("select idstudent,name,age from student").fetch_all(&pool).await?;

    let mut data: Vec<RawRawData> = vec![];

    for row in rows.iter() {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        data.push(RawRawData{
            id,
            name,
            age
        });
    }

    signal::shutdown().await;
    Ok(())
}
