pub mod price;
pub mod simulate;
pub mod forecast;
pub mod assets;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(price::get_price)
            .service(simulate::simulate)
            .service(forecast::forecast)
            .service(assets::create_asset)
            // You can register update and delete handlers here.
    );
}
