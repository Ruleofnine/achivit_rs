use crate::{embeds, requirements::{get_requirements_bytes, RequirementList}, Context, Error, db::{ASCEND_GUILD_ID, INN_GUILD_ID}};
use color_eyre::Result;
use getset::Getters;
use poise::serenity_prelude::Attachment;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use crate::dev_tools::is_superuser_check;
#[derive(sqlx::FromRow, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct GuildSettings {
    guild_name: String,
    guild_id: i64,
    announcement_channel_id: Option<i64>,
    announcement_role_id: Option<i64>,
}
//we need to return 'a reference of transaction/pool but we don't need the &requierments
pub async fn insert_requirements<'a,'b>(guild_id:i64,pool:&'a PgPool,requirements:&'b RequirementList)->Result<()>{
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
async fn set_requirements(ctx: Context<'_>, file: Attachment,guild_id:i64,title:String)->Result<()>{
    if let Some(file_type) = &file.content_type {
        if file_type != "application/json; charset=utf-8" {
            return Ok(embeds::wrong_file_type(ctx, file_type).await?);
        }
    }
    let file = file.download().await?;
    let mut requirements = match get_requirements_bytes(&file) {
        Ok(data) => data,
        Err(e) => return Ok(embeds::role_init_error(ctx, e).await?),
    };
    let pool = &ctx.data().db_connection;
    insert_requirements(guild_id, pool, &requirements).await?;
    embeds::roles_embed(ctx, &mut requirements,title).await?;
    Ok(())
}
#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn set_roles(ctx: Context<'_>, file: Attachment) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().0 as i64;
    let title = format!("{}'s Roles",ctx.guild().unwrap().name);
    set_requirements(ctx, file, guild_id,title).await?;
    Ok(())
}
#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR", guild_only,check = "is_superuser_check")]
pub async fn set_ascends(ctx: Context<'_>, file: Attachment) -> Result<(), Error> {
    set_requirements(ctx, file, ASCEND_GUILD_ID,"Ascendancies".to_string()).await?;
    Ok(())
}

#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR", guild_only,check = "is_superuser_check")]
pub async fn set_inn_items(ctx: Context<'_>, file: Attachment) -> Result<(), Error> {
    set_requirements(ctx, file, INN_GUILD_ID,"Inn Items".to_string()).await?;
    Ok(())
}
