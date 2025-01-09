use actix_web::{delete, get, patch, post, web, HttpResponse};
use chrono::NaiveDate;
use sea_orm::*;
use serde_json::json;
use validator::Validate;

use crate::entities::emp;
use crate::entities::prelude::{Dept, Emp};
use crate::error::ApiCustomError;
use crate::state::AppState;

/// 構造体: emp リクエストJson
#[derive(validator::Validate, serde::Deserialize, serde::Serialize, Debug)]
struct EmpRequestJson {
    // ename
    #[validate(length(min = 1, max = 10, message = "1~10文字で入力してください."))]
    ename: String,

    // job
    #[validate(length(min = 1, max = 9, message = "1~9文字で入力してください."))]
    job: String,

    // mgr
    mgr: Option<i32>,

    // hiredate
    hiredate: NaiveDate,

    // sal
    #[validate(range(
        min = 0.01,
        max = 99999.99,
        message = "0.01 ~ 99999.99の範囲で入力してください."
    ))]
    sal: f32,

    // comm
    #[validate(range(
        min = 0.01,
        max = 99999.0,
        message = "0.01 ~ 99999.99の範囲で入力してください."
    ))]
    comm: Option<f32>,

    // deptno
    deptno: i32,
}

/// emp 全件取得
#[get("/emp")]
async fn get_emp_all(data: web::Data<AppState>) -> Result<HttpResponse, ApiCustomError> {
    // レコード全件取得
    let emps = Emp::find()
        .order_by_asc(emp::Column::Empno)
        .all(&data.conn)
        .await?;

    // レスポンス
    if emps.is_empty() {
        return Ok(HttpResponse::NoContent().finish());
    }
    Ok(HttpResponse::Ok().json(emps))
}

/// emp キー取得
#[get("/emp/{empno}")]
async fn get_emp_by_key(
    path: Result<actix_web::web::Path<u32>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // クエリストリング取得（＋型チェックと型変換）
    let empno: i32 = path?.into_inner().try_into().unwrap();

    // キーに該当するレコードを取得する
    let emp = Emp::find_by_id(empno).one(&data.conn).await?;

    // レスポンス
    match emp {
        Some(emp) => Ok(HttpResponse::Ok().json(emp)),
        _ => Err(ApiCustomError::NotFound),
    }
}

/// emp 登録
#[post("/emp")]
async fn post_emp(
    form: Result<actix_web::web::Json<EmpRequestJson>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // バリデート
    let form = form?;
    form.validate()?;

    // 親レコードチェック
    if !exists_dept(&data.conn, form.deptno).await? {
        return Err(ApiCustomError::UnporcessibleEntity(format!(
            "deptno [{}] is not exists.",
            form.deptno
        )));
    }

    // mgrチェック
    // ... mgr（上司の社員コード） はempテーブル上存在していること
    if form.mgr.is_some() && !exists_emp(&data.conn, form.mgr.unwrap()).await? {
        return Err(ApiCustomError::UnporcessibleEntity(format!(
            "mgr(empno) [{}] is not exists.",
            form.mgr.unwrap()
        )));
    }

    // DB登録
    let emp = emp::ActiveModel::from_json(json!(form))?
        .insert(&data.conn)
        .await?;

    // レスポンス
    Ok(HttpResponse::Created().json(emp))
}

/// emp 変更
#[patch("/emp/{empno}")]
async fn patch_emp(
    path: Result<actix_web::web::Path<u32>, actix_web::Error>,
    form: Result<actix_web::web::Json<EmpRequestJson>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // クエリストリング取得（＋型チェックと型変換）
    let empno: i32 = path?.into_inner().try_into().unwrap();

    // バリデート
    let form = form?;
    form.validate()?;

    // 親レコードチェック
    if !exists_dept(&data.conn, form.deptno).await? {
        return Err(ApiCustomError::UnporcessibleEntity(format!(
            "deptno [{}] is not exists.",
            form.deptno
        )));
    }

    // 更新
    let emp = Emp::find_by_id(empno).one(&data.conn).await?;
    let updated_emp = match emp {
        Some(emp) => {
            let mut emp_active_model = emp.into_active_model();
            emp_active_model.set_from_json(json!(form))?;
            emp_active_model.update(&data.conn).await?
        }
        _ => return Err(ApiCustomError::NotFound),
    };

    // レスポンス
    Ok(HttpResponse::Ok().json(updated_emp))
}

#[delete("/emp/{empno}")]
async fn delete_emp(
    path: Result<actix_web::web::Path<u32>, actix_web::Error>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ApiCustomError> {
    // クエリストリング取得（型チェック込）
    let empno: i32 = path?.into_inner().try_into().unwrap();

    // 削除対象empnoと同じ値のmgrのレコードが存在する場合は削除不可
    if exists_emp_mgr(&data.conn, empno).await? {
        return Err(ApiCustomError::UnporcessibleEntity(format!(
            "empno [{}] can not delete.",
            empno
        )));
    }

    // キーに該当するレコードを削除する
    let result = Emp::delete_by_id(empno).exec(&data.conn).await?;

    // レスポンス
    if result.rows_affected == 0 {
        return Err(ApiCustomError::NotFound);
    }
    Ok(HttpResponse::NoContent().finish())
}

/// 参照制約チェック（親レコード有無）
///
/// 引数のdeptnoが設定されたdeptテーブルの有無を返却
async fn exists_dept(conn: &DatabaseConnection, deptno: i32) -> Result<bool, DbErr> {
    Ok(Dept::find_by_id(deptno).one(conn).await?.is_some())
}

/// empno存在チェック
///
/// 引数のempnoに紐づくempテーブルレコードの有無を返却
async fn exists_emp(conn: &DatabaseConnection, empno: i32) -> Result<bool, DbErr> {
    Ok(Emp::find_by_id(empno).one(conn).await?.is_some())
}

/// empno mgr存在チェック
///
/// 引数のempnoと同じ値が設定されているempテーブル.mgr のレコードの有無を返却
async fn exists_emp_mgr(conn: &DatabaseConnection, empno: i32) -> Result<bool, DbErr> {
    Ok(Emp::find()
        .filter(emp::Column::Mgr.eq(empno))
        .one(conn)
        .await?
        .is_some())
}
