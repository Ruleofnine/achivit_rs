use std::fs;

use poise::serenity_prelude::Attachment;
use color_eyre::Result;
use sqlx::query;
use crate::{Context,Error, embeds, requirements::get_requirements_bytes};
use serde::{Serialize, Deserialize};
use getset::Getters;
#[derive(sqlx::FromRow,Serialize, Deserialize,Getters)]
#[getset(get = "pub")]
pub struct GuildSettings{
guild_name:String,
guild_id:i64,
roles_path:String
}
#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR")]
pub async fn set_roles(ctx: Context<'_>,file:Attachment) -> Result<(), Error> {
    if let Some(file_type)=  &file.content_type{
        if file_type != "application/json; charset=utf-8"{
            return Ok(embeds::wrong_file_type(ctx, file_type).await?)
        }
    }
    let file = file.download().await?;
    let mut roles = match get_requirements_bytes(&file){
        Ok(data) => data,
        Err(e) => return Ok(embeds::role_init_error(ctx,e).await?)
    };
    let guild = ctx.guild().expect("expected guild");
    let guild_name = guild.name.as_str();
    let json_path = format!("{guild_name}_roles.json"); 
    let guild_setting = GuildSettings{
        guild_name:guild.name.to_owned(),
        guild_id:guild.id.0 as i64,
        roles_path:json_path.to_owned()
    };
    let roles_json = serde_json::to_string(roles.requirements())?;
    fs::write(format!("JSONS/{json_path}"),roles_json.as_bytes())?;
    let pool = &ctx.data().db_connection;
    query!("
INSERT INTO public.guild_settings (guild_id, guild_name, roles_path)
VALUES ($1, $2, $3)
ON CONFLICT (guild_id) 
DO UPDATE SET
    guild_name = EXCLUDED.guild_name,
    roles_path = EXCLUDED.roles_path;
    ",guild_setting.guild_id as i64,guild_setting.guild_name,guild_setting.roles_path).execute(pool).await?;
    embeds::roles_embed(ctx, &mut roles).await?;
    Ok(())
}
