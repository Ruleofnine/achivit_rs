#[cfg(test)]
mod tests {
    use dfelp::wiki::get_wiki;
    #[tokio::test]
    async fn test_wiki2(){
        let res = match get_wiki("jack crescent and the gnarly guitar of doom").await {
            Ok(_) => {"Found"}
            Err(_) => {"Not Found"}
        };
        assert_eq!("Found",res);
    }
}

