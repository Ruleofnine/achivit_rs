use crate::dev_tools::is_superuser_check;
use crate::{
    db::{ASCEND_GUILD_ID, INN_GUILD_ID},
    embeds,
    requirements::{get_requirements_bytes, RequirementList},
    Context, Error,
};
use color_eyre::Result;
use getset::Getters;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Attachment;
use serde::{Deserialize, Serialize};
use serenity::model::{channel::Channel, guild::Role};
use sqlx::{query, query_as, PgPool};
#[derive(sqlx::FromRow, Serialize, Deserialize, Getters, Debug)]
#[getset(get = "pub")]
pub struct GuildSettings {
    pub guild_name: String,
    pub guild_id: i64,
    pub announcement_channel_id: Option<i64>,
    pub announcement_role_id: Option<i64>,
}
//we need to return 'a reference of transaction/pool but we don't need the &requierments
pub async fn insert_requirements(
    guild_id: i64,
    pool: &PgPool,
    requirements: &RequirementList,
) -> Result<()> {
    let mut transaction = pool.begin().await?;
    // delete any requirements already assigned to guild.
    query!("delete from requirements where guild_id = $1", guild_id)
        .execute(&mut *transaction)
        .await?;
    //insert requirement
    for req in requirements.requirements() {
        let record = sqlx::query_as!(
                RequirementId,
                "INSERT INTO requirements (guild_id, name, description, type, amount) VALUES ($1, $2, $3, $4, $5) RETURNING requirementid",
                guild_id,
                req.name(),
                req.description,
                req.req_type.to_string(),
                req.amount
            )
            .fetch_one(&mut *transaction)
            .await?;
        // if required items insert them here
        if let Some(reqs) = &req.required {
            for item in reqs {
                sqlx::query!(
                    "INSERT INTO requireditems (requirementid, itemname) VALUES ($1, $2)",
                    record.requirementid,
                    item
                )
                .execute(&mut *transaction)
                .await?;
            }
        }
    }
    // second loop to insert the prereqs
    for req in requirements.requirements() {
        let record = sqlx::query_as!(
            RequirementId,
            "select  RequirementId from requirements where guild_id = $1 and name = $2",
            guild_id,
            req.name
        )
        .fetch_one(&mut *transaction)
        .await?;
        let requirement_id = record.requirementid;
        // if required items insert them here
        if let Some(prereqs) = &req.prereqs {
            for prereq in prereqs {
                let record = sqlx::query_as!(
                    RequirementId,
                    "select RequirementId from requirements where guild_id = $1 and name = $2",
                    guild_id,
                    prereq
                )
                .fetch_one(&mut *transaction)
                .await?;
                let prereq_id = record.requirementid;
                sqlx::query!("INSERT into prerequisites (RequirementId, PrerequisiteRequirementID) VALUES ($1,$2)",requirement_id,prereq_id).execute(&mut *transaction).await?;
            }
        }
    }
    transaction.commit().await?;
    Ok(())
}
struct RequirementId {
    requirementid: i32,
}
async fn set_requirements(
    ctx: Context<'_>,
    file: Attachment,
    guild_id: i64,
    title: String,
) -> Result<()> {
    if let Some(file_type) = &file.content_type {
        if file_type != "application/json; charset=utf-8" {
            embeds::wrong_file_type(ctx, file_type).await?
        }
    }
    let file = file.download().await?;
    let mut requirements = match get_requirements_bytes(&file) {
        Ok(data) => data,
        Err(e) => return embeds::role_init_error(ctx, e).await
    };
    let pool = &ctx.data().db_connection;
    insert_requirements(guild_id, pool, &requirements).await?;
    embeds::roles_embed(ctx, &mut requirements, title).await?;
    Ok(())
}
#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn set_roles(ctx: Context<'_>, file: Attachment) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().0 as i64;
    let guild_name = ctx.guild().unwrap().name;
    query!(
        "INSERT INTO guild_settings (guild_id, guild_name)
VALUES ($1, $2)
ON CONFLICT (guild_id)
DO NOTHING",
        guild_id,
        guild_name
    )
    .execute(ctx.data().db())
    .await?;
    let title = format!("{}'s Roles", ctx.guild().unwrap().name);
    set_requirements(ctx, file, guild_id, title).await?;
    Ok(())
}
#[poise::command(
    prefix_command,
    required_permissions = "ADMINISTRATOR",
    guild_only,
    check = "is_superuser_check"
)]
pub async fn set_ascends(ctx: Context<'_>, file: Attachment) -> Result<(), Error> {
    set_requirements(ctx, file, ASCEND_GUILD_ID, "Ascendancies".to_string()).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR",
    guild_only,
    check = "is_superuser_check"
)]
pub async fn set_inn_items(ctx: Context<'_>, file: Attachment) -> Result<(), Error> {
    set_requirements(ctx, file, INN_GUILD_ID, "Inn Items".to_string()).await?;
    Ok(())
}
#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn init_announcements(
    ctx: Context<'_>,
    channel: Channel,
    role: Role,
) -> Result<(), Error> {
    let pool = ctx.data().db();
    let guild_id = ctx.guild_id().unwrap().0 as i64;
    let guild_name = ctx.guild().unwrap().name;
    let channel_id = channel.id().0 as i64;
    let role_id = role.id.0 as i64;
    let settings = query_as!(
        GuildSettings,
        "
INSERT INTO guild_settings (guild_id, guild_name, announcement_channel_id, announcement_role_id)
VALUES ($1, $2, $3, $4)
ON CONFLICT (guild_id)
DO UPDATE SET 
    guild_name = EXCLUDED.guild_name,
    announcement_role_id = EXCLUDED.announcement_role_id,
    announcement_channel_id = EXCLUDED.announcement_channel_id RETURNING *;
    ",
        guild_id,
        guild_name,
        channel_id,
        role_id
    )
    .fetch_one(pool)
    .await?;
    ctx.reply(format!(
        "announcement channel for {} set to {}",settings.guild_name,settings.announcement_channel_id.unwrap_or(-1)
    ))
    .await?;
    Ok(())
}
#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn init_guild(ctx: Context<'_>) -> Result<(), Error> {
    let pool = ctx.data().db();
    let guild_id = ctx.guild_id().unwrap().0 as i64;
    let guild_name = ctx.guild().unwrap().name;
    let guild_settings = query_as!(
        GuildSettings,
        "
INSERT INTO guild_settings (guild_id, guild_name)
VALUES ($1, $2)
ON CONFLICT (guild_id)
DO UPDATE SET 
    guild_name = EXCLUDED.guild_name
    RETURNING *;
    ",
        guild_id,
        guild_name,
    )
    .fetch_one(pool)
    .await?;
    ctx.reply(format!(
        "Guild Init with Guild ID:{} and Guild Name:{}",
        guild_settings.guild_id, guild_settings.guild_name
    ))
    .await?;
    Ok(())
}
#[poise::command(prefix_command, owners_only, dm_only)]
pub async fn leave_guild(ctx: Context<'_>, guild_id: i64) -> Result<(), Error> {
    let pool = ctx.data().db();
    query!("DELETE FROM guild_settings where guild_id = $1;", guild_id,)
        .execute(pool)
        .await?;
    if let Some(guild) =
        serenity::GuildId(guild_id as u64).to_guild_cached(&ctx.serenity_context().cache)
    {
        guild.leave(ctx).await?;
        ctx.say(format!("Successfully left the guild: {}", guild.name))
            .await?;
    } else {
        ctx.say("Guild not found or bot is not a member of that guild.")
            .await?;
    }
    Ok(())
}
