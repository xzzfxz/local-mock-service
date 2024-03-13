use crate::model::ErrorResult;
use crate::service::fund;
use actix_web::{get, post, HttpResponse, Responder};

#[post("/product/sync")]
pub async fn sync_product() -> impl Responder {
    match fund::sync_product() {
        Ok(res) => {
            println!("/product/sync 成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/product/sync失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: 200,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}

#[get("/product/show_strategy")]
pub async fn show_strategy() -> impl Responder {
    match fund::show_strategy() {
        Ok(res) => {
            println!("/product/show_strategy 成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/product/show_strategy失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: 200,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}
