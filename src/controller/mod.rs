use actix_web::web::{scope, ServiceConfig};

mod fund;

// 基金配置
pub fn fund_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/control-center/api/v1/public")
            .service(fund::sync_product)
            .service(fund::show_strategy),
    );
}
