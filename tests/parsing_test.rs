extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::requirements::get_requirements_file;
    use achivit_rs::paginate::get_requirement_pages;
    use color_eyre::Result;
    #[test]
    fn paginate_test() -> Result<()> {
        let reqs = get_requirements_file("InnList.json")?;
        let pages = get_requirement_pages(reqs,None);
        pages.iter().for_each(|page|assert!(page.len()<4096));
        Ok(())
    }
}
