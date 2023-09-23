cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        use actix_cors::Cors;
        use actix_web::{App, HttpServer};

        mod routes;
        use routes::report;
        use routes::index;
        use tracing::Level;
        mod page;

        #[actix_web::main]
        async fn main() {

            tracing_subscriber::fmt::fmt().with_max_level(Level::DEBUG).init();
            std::env::set_var("OPENAI_API_KEY", "");

            HttpServer::new(move || {
                let cors = Cors::default().allow_any_origin().max_age(3600);
                App::new()
                    // Note that for the middleware to work during development, you need to connect
                    // via 127.0.0.1 and not localhost.
                    .service(report).service(index)
                    .service(actix_files::Files::new("/", "./dist").prefer_utf8(true))
                    .wrap(cors)
                    .wrap(actix_web::middleware::Logger::new("%r %U").log_target("actix"))
                    .wrap(actix_web::middleware::Compress::default())
            })
            .bind(("127.0.0.1", 8000))
            .unwrap()
            .run()
            .await
            .unwrap();
        }

    }
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use report_gen_demo::WebHandle;
    // Redirect `log` message to `console.log` and friends:
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async { WebHandle::new().start("Example").await.unwrap() });
}
