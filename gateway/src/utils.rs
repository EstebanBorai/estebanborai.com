use cfg_if::cfg_if;
use url::Url;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn make_target_url(base_url: Url, incoming: Url) -> Url {
    let mut base_url = base_url;

    base_url.set_path(incoming.path());
    base_url.set_query(incoming.query());
    base_url
}

#[cfg(test)]
mod tests {
    use reqwest::Url;
    use std::str::FromStr;

    use super::make_target_url;

    #[test]
    fn makes_target_url() {
        let base_url = vec![
            Url::parse("https://api.example.com").unwrap(),
            Url::parse("https://johnwick.com").unwrap(),
            Url::parse("https://www.estebanborai.com").unwrap(),
        ];

        let incoming = vec![
            "https://estebanborai.com/graphql",
            "https://estebanborai.com/api/v1/auth",
            "https://estebanborai.com/graphql?query=test",
        ];

        let outgoing = vec![
            "https://api.example.com/graphql",
            "https://johnwick.com/api/v1/auth",
            "https://www.estebanborai.com/graphql?query=test",
        ];

        incoming
            .iter()
            .zip(base_url)
            .zip(outgoing)
            .for_each(|((incoming, base_url), outgoing)| {
                assert_eq!(
                    make_target_url(base_url, Url::from_str(&incoming).unwrap()),
                    Url::from_str(outgoing).unwrap()
                );
            });
    }
}
