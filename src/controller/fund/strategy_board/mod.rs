use crate::model::ErrorResult;
use crate::service::fund;
use actix_web::{get, post, HttpResponse, Responder};

/// 获取产品列表
#[post("/sync")]
pub async fn sync_product() -> impl Responder {
    match fund::strategy_board::sync_product() {
        Ok(res) => {
            println!("/product/sync 成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/product/sync失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: -1,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}

/// 获取策略列表
#[get("/show_strategy")]
pub async fn show_strategy() -> impl Responder {
    match fund::strategy_board::show_strategy() {
        Ok(res) => {
            println!("/product/show_strategy 成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/product/show_strategy失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: -1,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}

/// 查询策略类型版本
#[post("/load/names")]
pub async fn load_category() -> impl Responder {
    match fund::strategy_board::load_category() {
        Ok(res) => {
            println!("/load/names 成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/load/names失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: -1,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}

/// 查询版本
#[post("/load/versions")]
pub async fn load_version() -> impl Responder {
    match fund::strategy_board::load_version() {
        Ok(res) => {
            println!("/load/versions 成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/load/versions失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: -1,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}

/// 批量上传
#[post("/strategy/position/parse_file")]
pub async fn parse_file() -> impl Responder {
    match fund::strategy_board::parse_file() {
        Ok(res) => {
            println!("/position/parse_file成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/position/parse_file失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: -1,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}

/// 保存状态
#[post("/strategy/update/basic/info")]
pub async fn save_basic_info() -> impl Responder {
    match fund::strategy_board::save_basic_info() {
        Ok(res) => {
            println!("/strategy/update/basic/info成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/strategy/update/basic/info失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: -1,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}
