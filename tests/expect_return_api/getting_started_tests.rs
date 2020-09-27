extern crate httpmock;

use isahc::{get, get_async};

use httpmock::Method::GET;
use httpmock::{Mock, MockServer};
use httpmock_macros::httpmock_example_test;

#[test]
#[httpmock_example_test] // Internal macro to make testing easier. Ignore it.
fn getting_started_test() {
    let _ = env_logger::try_init();

    // Start a local mock server for exclusive use by this test function.
    let server = MockServer::start();

    // Create a mock on the mock server. The mock will return HTTP status code 200 whenever
    // the mock server receives a GET-request with path "/hello".
    let hello_mock = Mock::new()
        .expect_method(GET)
        .expect_path("/hello")
        .return_status(200)
        .create_on(&server);

    // Send an HTTP request to the mock server. This simulates your code.
    // The server variable is being used to generate a mock server URL for path "/hello".
    let response = get(server.url("/hello")).unwrap();

    // Ensure the mock server did respond as specified above.
    assert_eq!(response.status(), 200);
    // Ensure the specified mock responded exactly one time.
    assert_eq!(hello_mock.hits(), 1);
}

#[async_std::test]
async fn async_getting_started_test() {
    let _ = env_logger::try_init();

    // Start a local mock server for exclusive use by this test function.
    let server = MockServer::start_async().await;

    // Create a mock on the mock server. The mock will return HTTP status code 200 whenever
    // the mock server receives a GET-request with path "/hello".
    let hello_mock = Mock::new()
        .expect_method(GET)
        .expect_path("/hello")
        .return_status(200)
        .create_on_async(&server)
        .await;

    // Send an HTTP request to the mock server. This simulates your code.
    let url = format!("http://{}/hello", server.address());
    let response = get_async(&url).await.unwrap();

    // Ensure the mock server did respond as specified above.
    assert_eq!(response.status(), 200);
    // Ensure the specified mock responded exactly one time.
    assert_eq!(hello_mock.hits_async().await, 1);
}
