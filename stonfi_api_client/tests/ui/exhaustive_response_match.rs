use stonfi_api_client::v1::V1Response;

fn check_response(response: V1Response) {
    match response {
        V1Response::Assets(_) => {}
        V1Response::AssetsQuery(_) => {}
        V1Response::Asset(_) => {}
        V1Response::Farms(_) => {}
        V1Response::Farm(_) => {}
        V1Response::Pools(_) => {}
        V1Response::Pool(_) => {}
        V1Response::Routers(_) => {}
        V1Response::Router(_) => {}
        V1Response::SwapSimulate(_) => {}
        V1Response::TransactionActionTree(_) => {}
        V1Response::TransactionQuery(_) => {}
    }
}

fn main() {}
