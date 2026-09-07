use crate::{Context, Error, Pila};

/// Responde con "Pong!" al usuario.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
    Ok(())
}

/// Agrega un item a la memoria.
#[poise::command(slash_command)]
pub async fn push(ctx: Context<'_>, #[description = "Item"] item: String) -> Result<(), Error> {
    {
        let mut memoria = ctx.data().memoria.lock().unwrap();
        let memoria_usuario = memoria.entry(ctx.author().id).or_insert(Pila::new());
        memoria_usuario.push(item);
    }
    ctx.reply("Se guardó el item en memoria.").await?;
    Ok(())
}

/// Agrega un item a la memoria.
#[poise::command(slash_command)]
pub async fn pop(ctx: Context<'_>) -> Result<(), Error> {
    let popped = {
        let mut memoria = ctx.data().memoria.lock().unwrap();
        let memoria_usuario = memoria.entry(ctx.author().id).or_insert(Pila::new());
        memoria_usuario.pop()
    };
    if let Some(item) = popped {
        ctx.reply(format!("Se sacó el item: {}", item)).await?;
    } else {
        ctx.reply("No hay items qué sacar.").await?;
    }
    Ok(())
}
