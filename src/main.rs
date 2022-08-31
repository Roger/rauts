mod handler;
mod request;
mod router;

use ocpp::v16::authorize::Authorize;
use request::{Call, Request, Source, CGW, TPBE};

use router::Router;

fn call_authorize_cgw(Call(Authorize { id_tag }, ..): Call<Authorize, CGW>) {
    println!("CGW: Authorize id_tag: {}", id_tag);
}

fn call_authorize_tpb(Call(authorize, ..): Call<Authorize, TPBE>) {
    println!("TPBE: {:?}", authorize);
}

fn main() {
    let router = Router::new()
        .route(call_authorize_cgw)
        .route(call_authorize_tpb);

    let ocpp_message =
        ocpp::unpack(r#"[2, "424242", "Authorize",  {"idTag": "454564564"}]"#).unwrap();
    let request = Request(ocpp_message, Source::TPBE);
    router.call(&request);

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "Authorize",  {"idTag": "1333333337"}]"#).unwrap();
    let request = Request(ocpp_message, Source::CGW);
    router.call(&request);

    let ocpp_message =
        ocpp::unpack(r#"[2, "6666666", "BootNotification",  {"chargePointModel": "optimus prime", "chargePointVendor": ""}]"#).unwrap();
    let request = Request(ocpp_message, Source::CGW);
    router.call(&request);
}
