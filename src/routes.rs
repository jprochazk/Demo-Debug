cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use actix_web::{get, web::Bytes, HttpResponse, Responder};
        use async_openai::types::{
            ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
        };
        use futures::StreamExt;

        #[get("/")]
        pub async fn index() -> std::io::Result<actix_files::NamedFile> {
            actix_files::NamedFile::open("./dist/index.html")
        }
        #[get("/reportstream")]
        pub async fn report() -> impl Responder {
            let (mut tx, rx) = futures::channel::mpsc::unbounded::<
                std::result::Result<actix_web::web::Bytes, std::io::Error>,
            >();

            let client = async_openai::Client::new();
            let request = CreateChatCompletionRequestArgs::default()
                .model("gpt-3.5-turbo")
                .max_tokens(1000u16)
                .messages([ChatCompletionRequestMessageArgs::default()
                    .content("Generate a random response to a random question")
                    .role(Role::User)
                    .build()
                    .unwrap()])
                .build()
                .unwrap();

            let mut result = client.chat().create_stream(request).await.unwrap();

            tokio::task::spawn(async move {
                while let Some(message) = result.next().await {
                    let message = match message {
                        Ok(message) => {
                            if let Some(message) = message.choices[0].delta.content.clone() {
                                message.clone()
                            } else {
                                String::new()
                            }
                        }
                        Err(e) => e.to_string(),
                    };
                    let _ = tx.start_send(Ok(Bytes::from(message)));
                }
            });

            HttpResponse::Ok().streaming(rx)
        }
    }
}
