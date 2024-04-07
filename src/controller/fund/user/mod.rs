use crate::model::ErrorResult;
use crate::service::fund;
use actix_web::{post, HttpResponse, Responder};

/// 登录
#[post("/login")]
pub async fn login() -> impl Responder {
    match fund::user::login() {
        Ok(res) => {
            println!("/login成功");
            HttpResponse::Ok().json(res)
        }
        Err(info) => {
            println!("/login失败: {:#?}", info);
            HttpResponse::Ok().json(ErrorResult {
                code: -1,
                msg: format!("{:?}", info),
                result: {},
            })
        }
    }
}
