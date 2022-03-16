use std::sync::Arc;

use ahash::AHashMap as HashMap;
use db::{common::ctx::ApiInfo, db_conn, system::entities::sys_role_api, DB};
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};
use tokio::sync::Mutex;
use tracing::info;

use crate::apps::system;

pub static ALL_APIS: Lazy<Arc<Mutex<HashMap<String, ApiInfo>>>> = Lazy::new(|| {
    let apis: HashMap<String, ApiInfo> = HashMap::new();
    Arc::new(Mutex::new(apis))
});

pub async fn init_all_api() {
    api_init_func().await;
}

pub async fn re_init_all_api() {
    let mut apis = ALL_APIS.lock().await;
    apis.clear();
    drop(apis);
    api_init_func().await;
}

async fn api_init_func() {
    let db = DB.get_or_init(db_conn).await;
    let res = system::get_all_sys_menu(db, false, true).await;
    match res {
        Ok(menus) => {
            for menu in menus {
                self::add_api(db, &menu.id, &menu.api, &menu.menu_name, &menu.is_db_cache, &menu.is_log).await;
            }
            let apis = ALL_APIS.lock().await;
            info!("初始化时获取路由API成功:{:#?}", apis);
            drop(apis);
        }
        Err(e) => {
            info!("初始化时获取路由API失败:{:#?}", e)
        }
    }
}

pub async fn add_api<C>(db: &C, api_id: &str, api: &str, menu_name: &str, is_db_cache: &str, is_log: &str)
where
    C: TransactionTrait + ConnectionTrait,
{
    let related_api = match system::get_related_api_by_db_name(db, api_id).await {
        Ok(x) => Some(x),
        Err(e) => {
            info!("{}", e);
            None
        }
    };

    let api_info = ApiInfo {
        name: menu_name.to_string(),
        related_api,
        is_db_cache: is_db_cache == "1",
        is_log: is_log == "1",
    };
    let mut apis = ALL_APIS.lock().await;
    apis.entry(api.to_string())
        .and_modify(|x| {
            *x = api_info.clone();
        })
        .or_insert(api_info);
    drop(apis)
}

pub async fn remove_api(api: &str) {
    let mut apis = ALL_APIS.lock().await;
    apis.remove(api);
    drop(apis)
}

pub async fn is_in(api: &str) -> bool {
    let apis = ALL_APIS.lock().await;
    let res = apis.get(api).is_some();
    drop(apis);
    res
}

pub async fn check_api_permission(api: &str, method: &str) -> bool {
    let db = DB.get_or_init(db_conn).await;
    match sys_role_api::Entity::find()
        .filter(sys_role_api::Column::Api.eq(api))
        .filter(sys_role_api::Column::Method.eq(method))
        .one(db)
        .await
    {
        Ok(x) => x.is_some(),
        Err(e) => {
            info!("检查API权限出现错误:{:#?}", e);
            false
        }
    }
}