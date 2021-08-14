pub fn voyager_source<'a, S>(path: S) -> String
where
    S: Into<&'a str>,
{
    let html_source = include_str!("index.html");
    html_source.replace("{GQL_PATH}", path.into())
}
