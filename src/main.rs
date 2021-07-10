use rocket_prometheus::PrometheusMetrics;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let prometheus = PrometheusMetrics::with_default_registry();
    rocket::build()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_one() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/metrics").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_two() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
