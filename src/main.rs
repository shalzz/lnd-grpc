extern crate futures;

mod protos;

use std::sync::{Arc};
use std::thread;
use std::net::SocketAddr;

use grpc::ClientStub;

use protos::rpc_grpc::{Lightning, LightningClient};
use protos::rpc::{GetInfoRequest};

use tls_api::{ TlsConnector, TlsConnectorBuilder};

fn test_tls_connector() -> tls_api_native_tls::TlsConnector {
    let root_ca = include_bytes!("../tls.cert");
    let root_ca = tls_api::Certificate::from_der(root_ca.to_vec());

    let mut builder = tls_api_native_tls::TlsConnector::builder().unwrap();
    builder
        .add_root_certificate(root_ca)
        .expect("add_root_certificate");
    builder.build().unwrap()
}

fn main() {
    let port = 10009;

    let client_conf = Default::default();

    let mut tls_option = httpbis::ClientTlsOption::Tls("127.0.0.1".to_owned(),
                Arc::new(test_tls_connector()));
    let addr = SocketAddr::new("::1".parse().unwrap(), port);

    let grpc_client =
            Arc::new(grpc::Client::new_expl(&addr, "127.0.0.1", tls_option, client_conf).unwrap());

    let client = LightningClient::with_client(grpc_client);

    let mut req = GetInfoRequest::new();

    let resp = client.get_info(grpc::RequestOptions::new(), req);

    let result = resp.wait();
    println!("{:?}", result.unwrap());

    loop {
        thread::park();
    }
}
