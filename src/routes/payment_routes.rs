use actix_web::web;
use crate::handlers::payment_handlers;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
      web::resource("/payment")
      .route(web::post().to(payment_handlers::create_payment))
    )
}



// use crate::handlers::b2c::pix_get_refund_by_e2eid_handle::pix_get_refund_by_e2eid;
// use crate::handlers::b2c::pix_immediate_handler::create_pix_immediate_payment;
// use crate::handlers::b2c::pix_immediate_list_received::pix_immediate_list_received;
// use crate::handlers::b2c::pix_immediate_list_received_by_txid::pix_immediate_list_received_by_txid;
// use crate::handlers::b2c::pix_refund::pix_refund;
// use crate::handlers::b2c::pix_webhook_create::create_pix_webhook;
// use crate::handlers::b2c::pix_with_due_date_handler::create_pix_with_due_date_payment;
// use actix_web::web;

// pub fn b2c_pix_routes(cfg: &mut web::ServiceConfig) {
//     cfg.route(
//         "b2c/pix_immediate_payment",
//         web::post().to(create_pix_immediate_payment),
//     );
//     cfg.route(
//         "b2c/pix_with_due_date_payment",
//         web::post().to(create_pix_with_due_date_payment),
//     );
//     cfg.route("b2c/pix_webhook_create", web::post().to(create_pix_webhook));
//     cfg.route(
//         "b2c/pix_immediate_list_received",
//         web::get().to(pix_immediate_list_received),
//     );
//     cfg.route(
//         "b2c/pix_immediate_list_received_by_txid/{txid}",
//         web::get().to(pix_immediate_list_received_by_txid),
//     );
//     cfg.route("b2c/pix_refund", web::post().to(pix_refund));

//     cfg.route(
//         "b2c/pix_get_refund_by_e2eid/{e2eid}",
//         web::get().to(pix_get_refund_by_e2eid),
//     );
// }
