pub mod routes;
mod slaves;

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use crate::models::request;
    use crate::models::response::Response;
    use super::*;

    fn process_response(res: Response, equality: bool, to_test: u16) {
        match res {
            Response::HTTP(res) => {
                if equality {
                    assert_eq!(res.status, to_test);
                } else {
                    assert_ne!(res.status, to_test);
                }
            },
            Response::Buffer(res) => {
                //convert to http as call itself
                println!("{:#?}", res);
            },
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn empty_route_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("/".to_string());
        let res = routes::handle_route(req, remote).await;
        process_response(res, false, 404);
    }

    #[tokio::test]
    #[should_panic]
    async fn help_method_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "POST".to_string();
        req.uri.push("help".to_string());
        req.version = "1.1".to_string();
        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 405);
    }

    #[tokio::test]
    async fn help_success() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("help".to_string());
        req.version = "1.1".to_string();
        let res = routes::handle_route(req, remote).await;

        process_response(res, true, 200);
    }

    #[tokio::test]
    #[should_panic]
    async fn loadtest_method_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "POST".to_string();
        req.uri.push("loadtest".to_string());
        req.params.insert("sleep".to_string(), "0".to_string());
        req.params.insert("tasks".to_string(), "1".to_string());
        req.version = "1.1".to_string();
        
        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 405);
    }

    #[tokio::test]
    #[should_panic]
    async fn loadtest_tasks_empty_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("loadtest".to_string());
        req.params.insert("sleep".to_string(), "0".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn loadtest_sleep_empty_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("loadtest".to_string());
        req.params.insert("task".to_string(), "0".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn loadtest_tasks_parse_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("loadtest".to_string());
        req.params.insert("tasks".to_string(), "test".to_string());
        req.params.insert("sleep".to_string(), "0".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn loadtest_sleep_parse_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("loadtest".to_string());
        req.params.insert("tasks".to_string(), "0".to_string());
        req.params.insert("sleep".to_string(), "0test".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn countwords_method_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "POST".to_string();
        req.uri.push("countwords".to_string());
        req.params.insert("name".to_string(), "counttest.txt".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 405);
    }

    #[tokio::test]
    #[should_panic]
    async fn countwords_name_empty_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("countwords".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn countwords_read_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("countwords".to_string());
        req.params.insert("name".to_string(), "countwords_test.txt".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;
        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn countwords_missing_slaves_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("countwords".to_string());
        req.params.insert("name".to_string(), "counttest.txt".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;
        process_response(res, false, 500);
    }

    #[tokio::test]
    #[should_panic]
    async fn matrixmult_method_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "POST".to_string();
        req.uri.push("matrixmult".to_string());
        req.version = "1.1".to_string();
        req.body = request::Body::JSON(r#"
            {
                "matrix_a": {
                    "matrix": [
                        [
                            1,
                            2
                        ],
                        [
                            3,
                            4
                        ]
                    ]
                },
                "matrix_b": {
                    "matrix": [
                        [
                            5,
                            6
                        ],
                        [
                            7,
                            8
                        ]
                    ]
                }
            }"#.to_string()
        );

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 405);
    }

    #[tokio::test]
    #[should_panic]
    async fn matrixmult_body_empty_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("matrixmult".to_string());
        req.version = "1.1".to_string();

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn matrixmult_body_parse_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("matrixmult".to_string());
        req.version = "1.1".to_string();
        req.body = request::Body::JSON(r#"
            {
                "matrix": [
                    [
                        1,
                        2
                    ],
                    [
                        3,
                        4
                    ]
                ],
                "matrix": [
                    [
                        5,
                        6
                    ],
                    [
                        7,
                        8
                    ]
                ]
            }"#.to_string()
        );

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn matrixmult_matrix_validate_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("matrixmult".to_string());
        req.version = "1.1".to_string();
        req.body = request::Body::JSON(r#"
            {
                "matrix_a": {
                    "matrix": [
                        [
                            1,
                            2,
                            3
                        ],
                        [
                            3,
                            4
                        ]
                    ]
                },
                "matrix_b": {
                    "matrix": [
                        [
                            5,
                            6
                        ],
                        [
                            7,
                            8
                        ]
                    ]
                }
            }"#.to_string()
        );

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }

    #[tokio::test]
    #[should_panic]
    async fn matrixmult_matrix_compatible_error() {
        let remote = "0.0.0.0:0".to_string().parse::<SocketAddr>().unwrap();
        let mut req = request::HttpRequest::default();
        req.method = "GET".to_string();
        req.uri.push("matrixmult".to_string());
        req.version = "1.1".to_string();
        req.body = request::Body::JSON(r#"
            {
                "matrix_a": {
                    "matrix": [
                        [
                            1,
                            2,
                            0
                        ],
                        [
                            3,
                            4,
                            0
                        ]
                    ]
                },
                "matrix_b": {
                    "matrix": [
                        [
                            5,
                            6
                        ],
                        [
                            7,
                            8
                        ]
                    ]
                }
            }"#.to_string()
        );

        let res = routes::handle_route(req, remote).await;

        process_response(res, false, 400);
    }
}