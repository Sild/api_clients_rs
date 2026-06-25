use bidask_api_client::api::{PageInfo, PaginatedResponse, PoolInfo, TokenInfo};

#[test]
fn test_response_models_support_default_setter_construction() {
    let page = PageInfo::default().with_total(10);
    let token = TokenInfo::default().with_symbol("TON".to_string());
    let pool = PoolInfo::default().with_address("pool".to_string());
    let response = PaginatedResponse::default().with_page(page.clone()).with_result(vec![pool.clone()]);

    assert_eq!(page.total, 10);
    assert_eq!(token.symbol, "TON");
    assert_eq!(response.page, page);
    assert_eq!(response.result, vec![pool]);
}
