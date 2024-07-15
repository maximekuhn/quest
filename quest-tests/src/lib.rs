pub fn start_test_server() {
    rouille::start_server("127.0.0.1:54638", move |request| {
        rouille::router!(request,
            (GET) (/) => {
                rouille::Response::text("hello world")
            },
            _ => rouille::Response::empty_404()
        )
    });
}
