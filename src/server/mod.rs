mod request;
mod response;
mod parser;
mod routes;
pub mod server;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::server::request::HttpRequest;

    use super::*;

    #[test]
    #[should_panic]
    fn empty_route_error() {
        let req = request::HttpRequest::basic("GET".to_string());
        let res = routes::handle_route(req, 0).unwrap();
        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn createfile_method_error() {
        let method = "GET".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "create_route_test".to_string());
        params.insert("content".to_string(), "test".to_string());
        params.insert("repeat".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }
    
    #[test]
    #[should_panic]
    fn createfile_name_error() {
        let method = "POST".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("content".to_string(), "test".to_string());
        params.insert("repeat".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn createfile_content_error() {
        let method = "POST".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "test".to_string());
        params.insert("repeat".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn createfile_repeat_empty_error() {
        let method = "POST".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "test".to_string());
        params.insert("content".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn createfile_repeat_parse_error() {
        let method = "POST".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "test".to_string());
        params.insert("content".to_string(), "test".to_string());
        params.insert("repeat".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn createfile_exists_error() {
        let method = "POST".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "create_route_test".to_string());
        params.insert("content".to_string(), "test".to_string());
        params.insert("repeat".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req.clone(), 0).unwrap();
        
        // First attempt should be successful
        assert_eq!(res.status, 200);

        let res = routes::handle_route(req, 0).unwrap();
        let _ = std::fs::remove_file("create_route_test");

        assert_ne!(res.status, 400);
    }

    #[test]
    fn createfile_success() {
        let method = "POST".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "create_route_test".to_string());
        params.insert("content".to_string(), "test".to_string());
        params.insert("repeat".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req.clone(), 0).unwrap();
        let _ = std::fs::remove_file("create_route_test");
        
        // First attempt should be successful
        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn deletefile_method_error() {
        let method = "GET".to_string();
        let uri = vec!["deletefile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "delete_route_test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn deletefile_name_error() {
        let method = "DELETE".to_string();
        let uri = vec!["deletefile".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn deletefile_error() {
        let method = "DELETE".to_string();
        let uri = vec!["deletefile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), "delete_route_test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    fn deletefile_success() {
        let name = "route_tests".to_string();
        let method = "POST".to_string();
        let uri = vec!["createfile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), name.clone());
        params.insert("content".to_string(), "test".to_string());
        params.insert("repeat".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let _ = routes::handle_route(req.clone(), 0).unwrap();

        let method = "DELETE".to_string();
        let uri = vec!["deletefile".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("name".to_string(), name);
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn fibonacci_method_error() {
        let method = "POST".to_string();
        let uri = vec!["fibonacci".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("num".to_string(), "100".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn fibonacci_num_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["fibonacci".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn fibonacci_num_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["fibonacci".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("num".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn fibonacci_res_max_error() {
        let method = "GET".to_string();
        let uri = vec!["fibonacci".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("num".to_string(), "1000".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 507);
    }

    #[test]
    fn fibonacci_success() {
        let method = "GET".to_string();
        let uri = vec!["fibonacci".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("num".to_string(), "100".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn hash_method_error() {
        let method = "POST".to_string();
        let uri = vec!["hash".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("text".to_string(), "hello".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn hash_text_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["hash".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    fn hash_success() {
        let method = "GET".to_string();
        let uri = vec!["hash".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("text".to_string(), "hello".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn help_method_error() {
        let method = "POST".to_string();
        let uri = vec!["help".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    fn help_success() {
        let method = "GET".to_string();
        let uri = vec!["help".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn loadtest_method_error() {
        let method = "POST".to_string();
        let uri = vec!["loadtest".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn loadtest_tasks_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["loadtest".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("sleep".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn loadtest_sleep_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["loadtest".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("tasks".to_string(), "0".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn loadtest_tasks_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["loadtest".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("tasks".to_string(), "test".to_string());
        params.insert("sleep".to_string(), "5".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn loadtest_sleep_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["loadtest".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("tasks".to_string(), "10".to_string());
        params.insert("sleep".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn random_method_error() {
        let method = "POST".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("count".to_string(), "10".to_string());
        params.insert("min".to_string(), "0".to_string());
        params.insert("max".to_string(), "10".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn random_count_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("min".to_string(), "0".to_string());
        params.insert("max".to_string(), "10".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn random_min_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("count".to_string(), "10".to_string());
        params.insert("max".to_string(), "10".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn random_max_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("count".to_string(), "10".to_string());
        params.insert("min".to_string(), "10".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn random_count_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("count".to_string(), "test".to_string());
        params.insert("min".to_string(), "0".to_string());
        params.insert("max".to_string(), "10".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn random_min_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("count".to_string(), "10".to_string());
        params.insert("min".to_string(), "test".to_string());
        params.insert("max".to_string(), "10".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn random_max_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("count".to_string(), "10".to_string());
        params.insert("min".to_string(), "0".to_string());
        params.insert("max".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    fn random_success() {
        let method = "GET".to_string();
        let uri = vec!["random".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("count".to_string(), "10".to_string());
        params.insert("min".to_string(), "0".to_string());
        params.insert("max".to_string(), "10".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn reverse_method_error() {
        let method = "POST".to_string();
        let uri = vec!["reverse".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("text".to_string(), "hello".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn reverse_text_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["reverse".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    fn reverse_success() {
        let method = "GET".to_string();
        let uri = vec!["reverse".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("text".to_string(), "hello".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn simulate_method_error() {
        let method = "POST".to_string();
        let uri = vec!["simulate".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("tasks".to_string(), "10".to_string());
        params.insert("seconds".to_string(), "1".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn simulate_task_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["simulate".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("seconds".to_string(), "1".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn simulate_seconds_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["simulate".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("task".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }
    
    #[test]
    #[should_panic]
    fn simulate_seconds_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["simulate".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("tasks".to_string(), "10".to_string());
        params.insert("seconds".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }
    
    #[test]
    #[should_panic]
    fn simulate_success() {
        let method = "GET".to_string();
        let uri = vec!["simulate".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("tasks".to_string(), "10".to_string());
        params.insert("seconds".to_string(), "1".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn sleep_method_error() {
        let method = "POST".to_string();
        let uri = vec!["sleep".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("seconds".to_string(), "1".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn sleep_seconds_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["sleep".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    #[should_panic]
    fn sleep_seconds_parse_error() {
        let method = "GET".to_string();
        let uri = vec!["sleep".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("seconds".to_string(), "test".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    fn sleep_success() {
        let method = "GET".to_string();
        let uri = vec!["sleep".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("seconds".to_string(), "2".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn status_method_error() {
        let method = "POST".to_string();
        let uri = vec!["status".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn timestamp_method_error() {
        let method = "POST".to_string();
        let uri = vec!["timestamp".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    fn timestamp_success() {
        let method = "GET".to_string();
        let uri = vec!["timestamp".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }

    #[test]
    #[should_panic]
    fn toupper_method_error() {
        let method = "POST".to_string();
        let uri = vec!["toupper".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("text".to_string(), "hello".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 405);
    }

    #[test]
    #[should_panic]
    fn toupper_text_empty_error() {
        let method = "GET".to_string();
        let uri = vec!["toupper".to_string()];
        let params = HashMap::<String, String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_ne!(res.status, 400);
    }

    #[test]
    fn toupper_success() {
        let method = "GET".to_string();
        let uri = vec!["toupper".to_string()];
        let mut params = HashMap::<String, String>::new();
        params.insert("text".to_string(), "hello".to_string());
        let version = "1.1".to_string();
        let headers = HashMap::<String, String>::new();
        let body = HashMap::<String, String>::new();

        let req = HttpRequest::new(method, uri, params, version, headers, body);
        let res = routes::handle_route(req, 0).unwrap();

        assert_eq!(res.status, 200);
    }
}
