use actix_web::{delete, get, patch, post, web, HttpResponse};
use sea_orm::*;
use serde_json::json;
use validator::Validate;

use crate::entities::prelude::{Dept, Emp};
use crate::entities::{dept, emp};
use crate::error::ApiCustomError;
use crate::state::AppState;

/// 構造体: dept リクエストJson
#[derive(validator::Validate, serde::Deserialize, serde::Serialize, Debug)]
struct DeptRequestJson {
    // dname
    #[validate(length(min = 1, max = 14, message = "1~14文字で入力してください."))]
    pub dname: String,

    // loc
    #[validate(length(min = 1, max = 13, message = "1~13文字で入力してください."))]
    pub loc: String,
}

/// dept 全件取得
#[get("/dept")]
async fn get_dept_all(data: web::Data<AppState>) -> Result<HttpResponse, ApiCustomError> {
    // レコード取得（全件）
    let depts = Dept::find()
        .order_by_asc(dept::Column::Deptno)
        .all(&data.conn)
        .await?;

    // レスポンス
    if depts.is_empty() {
        return Ok(HttpResponse::NoContent().finish());
    }
    Ok(HttpResponse::Ok().json(depts))
}

/// dept キー取得
#[get("/dept/{deptno}")]
async fn get_dept_by_key(
    path: Result<actix_web::web::Path<u32>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // クエリストリング取得（＋型チェックと型変換）
    let deptno: i32 = path?.into_inner().try_into().unwrap();

    // キーに該当するレコードを取得する
    let dept = Dept::find_by_id(deptno).one(&data.conn).await?;

    // レスポンス
    match dept {
        Some(dept) => Ok(HttpResponse::Ok().json(dept)),
        _ => Err(ApiCustomError::NotFound),
    }
}

/// dept 登録
#[post("/dept")]
async fn post_dept(
    form: Result<actix_web::web::Json<DeptRequestJson>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // バリデート
    let form = form?.into_inner();
    form.validate()?;

    // DB登録
    let dept = dept::ActiveModel::from_json(json!(form))?
        .insert(&data.conn)
        .await?;

    // レスポンス
    Ok(HttpResponse::Created().json(dept))
}

/// dept 変更
#[patch("/dept/{deptno}")]
async fn patch_dept(
    path: Result<actix_web::web::Path<u32>, actix_web::Error>,
    form: Result<actix_web::web::Json<DeptRequestJson>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // クエリストリング取得（型チェック込）
    let deptno: i32 = path?.into_inner().try_into().unwrap();

    // バリデート
    let form = form?;
    form.validate()?;

    // 更新
    let dept = Dept::find_by_id(deptno).one(&data.conn).await?;
    let updated_dept = match dept {
        Some(dept) => {
            let mut dept_active_model = dept.into_active_model();
            dept_active_model.set_from_json(json!(form))?;
            dept_active_model.update(&data.conn).await?
        }
        _ => return Err(ApiCustomError::NotFound),
    };

    // レスポンス
    Ok(HttpResponse::Ok().json(updated_dept))
}

/// dept 削除
#[delete("/dept/{deptno}")]
async fn delete_dept(
    path: Result<actix_web::web::Path<u32>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // クエリストリング取得（型チェック込）
    let deptno: i32 = path?.into_inner().try_into().unwrap();

    // 参照制約チェック
    if exists_references(&data.conn, deptno).await? {
        return Err(ApiCustomError::UnporcessibleEntity(format!(
            "deptno [{}] can not delete.",
            deptno
        )));
    }

    // キーに該当するレコードを削除する
    let result = Dept::delete_by_id(deptno).exec(&data.conn).await?;

    // レスポンス
    if result.rows_affected == 0 {
        return Err(ApiCustomError::NotFound);
    }
    Ok(HttpResponse::NoContent().finish())
}

/// 参照制約チェック
///
/// 引数のdeptnoが設定されたempテーブルの有無を返却
async fn exists_references(conn: &DatabaseConnection, deptno: i32) -> Result<bool, DbErr> {
    Ok(Emp::find()
        .filter(emp::Column::Deptno.eq(deptno))
        .one(conn)
        .await?
        .is_some())
}
