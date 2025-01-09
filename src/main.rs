use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sea_orm::*;
use std::env;
use std::str::FromStr;
use time::macros::format_description;
use validator::Validate;

mod entities;
mod error;
use crate::error::ApiCustomError;
mod services;
use crate::services::dept_service::*;
use crate::services::emp_service::*;
mod state;
use crate::state::AppState;
mod middleware;
use crate::middleware::access_log;

/// hello
///
/// get: /
#[get("/")]
async fn hello() -> impl Responder {
    tracing::info!("logging: Hello World!");
    HttpResponse::Ok().body("Hello World!")
}

/// echo
///
/// post: /echo
///
/// * `req_body` - リクエストボディ
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// manual_hello
///
/// get /manual_hello
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello there!")
}

/// 構造体: パスパラメータ1 レスポンス
#[derive(serde::Serialize)]
struct Path1Res {
    value: u32,
}

/// get パスパラメータ1
///
/// get: get_path1
/// パラメータは u32（数値）のみ受け付る. 数値以外は404.
#[get("/path1/{param1}")]
async fn get_path1(
    param: Result<actix_web::web::Path<u32>, actix_web::Error>,
) -> Result<HttpResponse, ApiCustomError> {
    // let req_param = param.into_inner();
    // HttpResponse::Ok().body(format!("param: {}", req_param))

    Ok(HttpResponse::Ok().json(Path1Res {
        value: param?.into_inner(),
    }))
}

/// get パスパラメータ2
///
/// get: get_path2
/// パラメータは u32（数値）を2つ受付る. 数値以外は404になる
#[get("/path2/{param1}/{param2}")]
async fn get_path2(
    params: Result<actix_web::web::Path<(u32, u32)>, actix_web::Error>,
) -> Result<HttpResponse, ApiCustomError> {
    let req_param = params?.into_inner();
    Ok(HttpResponse::Ok().body(format!("param1: {}, param2: {}", req_param.0, req_param.1)))
}

/// get パスパラメータ3
///
/// get: get_path3
/// パラメータは 文字列を受け取る（スラッシュ含める）.
#[get("/path3/{param:.*}")]
async fn get_path3(param: actix_web::web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("param: {}", param.into_inner()))
}

/// 構造体: サンプルJSONリクエスト
#[derive(serde::Deserialize)]
struct JsonReq {
    val1: u32,
    val2: String,
}

/// 構造体: サンプルJSONレスポンス
#[derive(serde::Serialize)]
struct JsonRes {
    res_val1: u32,
    res_val2: String,
}

/// Post JSONリクエスト
///
/// post: /json
///
/// * `data` - リクエストボディ（JSON）
#[post("/json")]
async fn post_json(
    data: Result<actix_web::web::Json<JsonReq>, actix_web::Error>,
) -> Result<impl Responder, ApiCustomError> {
    // let req = data.into_inner();
    // HttpResponse::Ok().json(JsonRes {
    //     res_val1: req.val1,
    //     res_val2: req.val2,
    // })
    let req = match data {
        Ok(json_req) => json_req.into_inner(),
        Err(err) => return Err(ApiCustomError::ActixWebError(err)),
    };
    Ok(HttpResponse::Ok().json(JsonRes {
        res_val1: req.val1,
        res_val2: req.val2,
    }))
}

/// ルートアンマッチ
///
/// エラー（ApiCustomError::NotFound）を返却する
async fn route_unmatch() -> Result<HttpResponse, ApiCustomError> {
    Err(ApiCustomError::NotFound)
}

/// 構造体: バリデーション（get）
#[derive(serde::Deserialize, serde::Serialize, validator::Validate)]
struct ValidateGetStruct {
    #[validate(
        range(min = 1, max = 10, message = "1~10の値で入力してください."),
        required(message = "必須.")
    )]
    x: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 5, message = "2~5文字で入力してください."))]
    y: Option<String>,
}

/// バリデーション(get)
///
/// get: /validate
///
/// * `param` - クエリストリング
#[get("/validate")]
async fn get_validate(
    param: Result<actix_web::web::Query<ValidateGetStruct>, actix_web::Error>,
) -> Result<HttpResponse, ApiCustomError> {
    // 1. actix-web
    // let param = match param {
    //     Ok(param) => param,
    //     Err(err) => return Err(ApiCustomError::ActixWebError(err)),
    // };

    // 2. validate
    // match param.validate() {
    //     Ok(_) => Ok(HttpResponse::Ok().json(ValidateGetStruct {
    //         x: param.x,
    //         y: param.y.clone(),
    //     })),
    //     Err(err) => Err(ApiCustomError::ValidationError(err)),
    // }

    // 1. acticx-web
    let param = param?;

    // 2. validate
    param.validate()?;

    // 3. レスポンス
    Ok(HttpResponse::Ok().json(ValidateGetStruct {
        x: param.x,
        y: param.y.clone(),
    }))
}

/// 正規表現パターン（郵便番号）
static REGEX_POST_CODE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^[\d]{3}-?[\d]{4}").unwrap());

/// 構造体: バリデーション（post）
#[derive(serde::Deserialize, serde::Serialize, validator::Validate)]
struct ValidatePostStruct {
    #[validate(
        length(min = 1, max = 10, message = "名前を入力してください.[1~10文字]"),
        required(message = "名前は必須項目です.")
    )]
    name: Option<String>,

    #[validate(
        range(min = 1, max = 12, message = "誕生月を入力してください.[1~12]"),
        required(message = "誕生月は必須項目です.")
    )]
    birth_month: Option<u32>,

    #[validate(email(message = "メールアドレスの形式が正しくありません."))]
    email: Option<String>,

    #[validate(url(message = "URLの形式が正しくありません."))]
    hp_url: Option<String>,

    #[validate(regex(path = *REGEX_POST_CODE, message = "郵便番号形式が正しくありません."))]
    post_code: Option<String>,
}

#[post("/validate")]
async fn post_validate(
    form: Result<actix_web::web::Json<ValidatePostStruct>, actix_web::Error>,
) -> Result<HttpResponse, ApiCustomError> {
    // 1. actix-web
    let form = match form {
        Ok(form) => form,
        Err(err) => return Err(ApiCustomError::ActixWebError(err)),
    };

    // 2. validator
    // match form.validate() {
    //     Ok(_) => Ok(HttpResponse::Ok().json(form)),
    //     Err(err) => Err(ApiCustomError::ValidationError(err)),
    // }

    // 2. validator
    form.validate()?;

    // 3. response
    Ok(HttpResponse::Ok().json(form))
    // match form.validate() {
    //     Ok(_) => Ok(HttpResponse::Ok().json(form)),
    //     Err(err) => Err(ApiCustomError::ValidationError(err)),
    // }
}

/// main
///
/// actix-web entrypoint
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv
    dotenv().ok();

    // logging
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Layer.html
    tracing_subscriber::fmt()
        .json()
        // JST(+0900)変換
        .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
            time::UtcOffset::from_hms(9, 0, 0)
                .expect("should be JST(+0900) setting.(hour: 9, min: 0, second: 0)"),
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory][offset_minute]"),
        ))
        .with_max_level(
            tracing::Level::from_str(&env::var("LOG_LEVEL").unwrap_or(String::from("info")))
                .unwrap_or(tracing::Level::INFO),
        )
        // .with_current_span(true)
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE)
        // .with_thread_ids(true)
        // .with_thread_names(true)
        // .with_target(true)
        // .with_file(true)
        // .with_line_number(true)
        .init();

    // db connection. see: https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/
    let mut opt =
        ConnectOptions::new(env::var("DATABASE_URL").expect("DB Connection should be set."));
    opt.sqlx_logging(true).sqlx_logging_level(
        tracing::log::LevelFilter::from_str(
            &env::var("SQLX_LOG_LEVEL").unwrap_or(String::from("debug")),
        )
        .unwrap_or(tracing::log::LevelFilter::Debug),
    );

    let conn = Database::connect(opt).await.unwrap();
    let state = AppState { conn };

    // http
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::from_fn(access_log))
            .app_data(web::Data::new(state.clone()))
            .service(hello)
            .service(echo)
            .service(get_path1)
            .service(get_path2)
            .service(get_path3)
            .service(post_json)
            .service(get_validate)
            .service(post_validate)
            .service(post_dept)
            .service(get_dept_all)
            .service(get_dept_by_key)
            .service(patch_dept)
            .service(delete_dept)
            .service(get_emp_all)
            .service(get_emp_by_key)
            .service(post_emp)
            .service(patch_emp)
            .service(delete_emp)
            .route("/hey", web::get().to(manual_hello))
            .default_service(web::route().to(route_unmatch))
    })
    .bind((
        env::var("HOST").unwrap_or("127.0.0.1".to_string()),
        (env::var("PORT").unwrap_or("8080".to_string()))
            .parse::<u16>()
            .unwrap(),
    ))?
    .workers(
        (env::var("WORKER").unwrap_or("10".to_string()))
            .parse::<usize>()
            .unwrap(),
    )
    .run()
    .await
}
