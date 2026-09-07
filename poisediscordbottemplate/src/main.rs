#![warn(clippy::str_to_string)]

mod commands;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use poise::serenity_prelude::{self as serenity};
use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {
    memoria: MemoryMap,
}

// Memoria

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Pila {
    vec: Vec<String>,
}

impl Pila {
    pub fn new() -> Pila {
        Pila { vec: Vec::new() }
    }

    pub fn push(&mut self, item: String) {
        self.vec.push(item);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.vec.pop()
    }
}

type MemoryMap = Arc<Mutex<HashMap<serenity::UserId, Pila>>>;

// Proceso principal

#[tokio::main]
async fn main() {
    // Se inicializa la memoria
    let memoria: MemoryMap = Arc::new(Mutex::new(HashMap::new()));
    let datos_memoria = memoria.clone();

    // Se crea el framework del bot
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping(), commands::push(), commands::pop()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    memoria: datos_memoria,
                }) // Se establece el dato compartido por los contextos.
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Falló al crear el cliente");

    let shard_manager = client.shard_manager.clone();

    // Se inicia el bot como una tarea
    let client_task = tokio::spawn(async move {
        if let Err(e) = client.start().await {
            eprintln!("El cliente finalizó con el siguiente error: {:?}", e)
        }
    });

    // Apagar gracefuly.
    tokio::signal::ctrl_c()
        .await
        .expect("Falló al esperar CTRL+C");

    shard_manager.shutdown_all().await;

    client_task
        .await
        .expect("Hubo un al esperar que finalize el cliente");
}
