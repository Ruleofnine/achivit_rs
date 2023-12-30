extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::{
        db::establish_connection,
        parsing::{FileFetcher, ParsingCategory},
        requirements::{check_requirements, get_requirements_file},
        guild_settings::insert_requirements, str_to_i64,
    };
    use color_eyre::Result;
    use dotenv::dotenv;
    use sqlx::query;
    #[test]
    fn inn_loads() -> Result<()> {
        get_requirements_file("InnList.json")?;
        Ok(())
    }
    #[tokio::test]
    async fn insert_ascends()->Result<()>{
        // dotenv().ok();
        // let pool = establish_connection().await?;
        // let inn_reqs = get_requirements_file("InnList.json")?;
        // let inn_guild_id = str_to_i64("inn_list");
        // let ascendancies_guild_id = str_to_i64("ascendancies");
        // let ascendancies_reqs = get_requirements_file("ascendancies.json")?;
        // insert_requirements(inn_guild_id, &pool, &inn_reqs).await?;
        // query!("insert into guild_settings (guild_name,guild_id) VALUES ('ascendancies',$1)",ascendancies_guild_id).execute(&pool).await?;
        // insert_requirements(ascendancies_guild_id, &pool, &ascendancies_reqs).await?;
        Ok(())

    }
    #[tokio::test]
    async fn db_roles_test() -> Result<()> {
        dotenv().ok();
        let pool = establish_connection().await?;
        let reqs = get_requirements_file("roles.json")?;
        query!("insert into guild_settings (guild_name,guild_id) VALUES ('test',0)").execute(&pool).await?;
        insert_requirements(0, &pool, &reqs).await?;
        let ruleofnine = FileFetcher::new("htmls/ruleofnine.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_character_data()?;
        let ruleofnine_roles = check_requirements(&ruleofnine, 0, &pool).await?;
        let just_name = FileFetcher::new("htmls/just_name.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_character_data()?;
        let just_name_roles = check_requirements(&just_name, 0, &pool).await?;
        let ach = FileFetcher::new("htmls/3ach.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_character_data()?;
        let ach_roles = check_requirements(&ach, 0, &pool).await?;
        query!("delete from guild_settings where guild_id = 0").execute(&pool).await?;
        assert_eq!(0, just_name_roles.requirements().len());
        assert_eq!(14, ruleofnine_roles.requirements().len());
        assert_eq!(17, ach_roles.requirements().len());
        let ascend_guild_id = str_to_i64("ascendancies");
        let ruleofnine_ascends = check_requirements(&ruleofnine, ascend_guild_id, &pool).await?;
        let just_name_ascends = check_requirements(&just_name, ascend_guild_id, &pool).await?;
        let ach_roles_ascends = check_requirements(&ach, ascend_guild_id, &pool).await?;
        assert_eq!(ach_roles_ascends.requirements().len(),5);
        assert_eq!(ruleofnine_ascends.requirements().len(),11);
        assert_eq!(just_name_ascends.requirements().len(),0);
        Ok(())
    }
}
