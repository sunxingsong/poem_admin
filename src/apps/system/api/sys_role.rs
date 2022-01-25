use crate::apps::{
    common::models::{ListData, Res},
    system::{entities::sys_role, service},
};
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query}, Result,
};

use crate::apps::common::models::{PageParams, RespData};
use serde_json::json;
use validator::Validate;

use crate::database::{db_conn, DB};

use super::super::models::sys_role::{
    AddReq, DataScopeReq, DeleteReq, EditReq, Resp, SearchReq, StatusReq,
};

/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<sys_role::Model>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>) -> Result<Json<RespData>> {
    req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::add(db, req).await?;
    Ok(Json(res))
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
    delete_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::delete(db, delete_req).await?;
    Ok(Json(res))
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Result<Json<RespData>> {
    //  数据验证
    edit_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::edit(db, edit_req).await?;
    Ok(Json(res))
}

// set_status 修改状态
#[handler]
pub async fn set_status(Json(status_req): Json<StatusReq>) -> Result<Json<RespData>> {
    //  数据验证
    status_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_status(db, status_req).await?;
    Ok(Json(res))
}
// set_data_scope 修改数据权限范围
#[handler]
pub async fn set_data_scope(Json(req): Json<DataScopeReq>) -> Json<Res<String>> {
    //  数据验证
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_data_scope(db, req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户   
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Json<Res<Resp>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_by_id(db, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_all 获取全部   
#[handler]
pub async fn get_all() -> Result<Json<RespData>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_all(db).await?;
    Ok(Json(RespData::with_data(json!(res))))
}

/// get_role_menu 获取角色授权菜单id数组   
#[handler]
pub async fn get_role_menu(Query(req): Query<SearchReq>) -> Json<Res<Vec<String>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    match req.role_id {
        None => Json(Res::with_msg("role_id不能为空")),
        Some(id) => {
            let res = service::sys_menu::get_permissions(vec![id]).await;
            Json(Res::with_data(res))
        }
    }
}

/// get_role_dept 获取角色授权部门id数组   
#[handler]
pub async fn get_role_dept(Query(req): Query<SearchReq>) -> Json<Res<Vec<String>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    match req.role_id {
        None => Json(Res::with_msg("role_id不能为空")),
        Some(id) => {
            let db = DB.get_or_init(db_conn).await;
            let res = service::sys_dept::get_dept_by_role_id(db, id).await;
            match res {
                Ok(x) => Json(Res::with_data(x)),
                Err(e) => Json(Res::with_err(&e.to_string())),
            }
        }
    }
}
