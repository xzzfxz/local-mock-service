use actix_web::web::{scope, ServiceConfig};

mod fund;

// 基金配置
pub fn fund_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/control-center/api/v1/public/product")
            .service(fund::strategy_board::sync_product)
            .service(fund::strategy_board::parse_file)
            .service(fund::strategy_board::save_basic_info)
            .service(fund::strategy_board::show_strategy),
    )
    .service(
        scope("/control-center/api/v1/public/strategy")
            .service(fund::strategy_board::load_version)
            .service(fund::strategy_board::load_category),
    )
    .service(scope("/auth-center/api/v1/auth").service(fund::user::login));
}
