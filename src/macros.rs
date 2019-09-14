#[macro_export]
macro_rules! api {
    () => {
        "https://discordapp.com/api/v6"
    };
    ($dest:expr) => {
        concat!(::automatea::api!(), $dest)
    };
    ($($dest:expr),*) => {{
        let mut s = String::from(::automatea::api!());
        $(::std::fmt::Write::write_fmt(&mut s, format_args!("{}", $dest)).expect("Failed to write api string");)*
        s
    }}
}

#[macro_export]
macro_rules! get {
    ($client:expr, $dest:expr) => {
        ::automatea::json::FromJson::from_json(
            ::reqwest::Client::get(&$client, ::automatea::api!($dest))
                .header("Authorization", "Bot NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY")
                .header("User-Agent", "DiscordBot (https://github.com/mbenoukaiss/automatea, 0.1.0)")
                .send()?
                .text()?
                .as_ref()
        )?
    };
    ($dest:expr) => {
        ::automatea::json::FromJson::from_json(
            ::reqwest::Client::get(&::reqwest::Client::new(), ::automatea::api!($dest))
                .header("Authorization", "Bot NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY")
                .header("User-Agent", "DiscordBot (https://github.com/mbenoukaiss/automatea, 0.1.0)")
                .send()?
                .text()?
                .as_ref()
        )?
    }
}

#[macro_export]
macro_rules! post {
    ($client:expr, $dest:expr, $content:expr) => {{
        ::reqwest::Client::post(&$client, &$dest)
            .header("Authorization", "Bot NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY")
            .header("User-Agent", "DiscordBot (https://github.com/mbenoukaiss/automatea, 0.1.0)")
            .header("Content-Type", "application/json")
            .body($content.as_json())
            .send()?
    }};
    ($dest:expr, $content:expr) => {
        ::reqwest::Client::post(&::reqwest::Client::new(), &$dest)
            .header("Authorization", "Bot NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY")
            .header("User-Agent", "DiscordBot (https://github.com/mbenoukaiss/automatea, 0.1.0)")
            .header("Content-Type", "application/json")
            .body(::automatea::json::AsJson::as_json(&$content))
            .send()?
    }
}

#[macro_export]
macro_rules! deserialize {
    ($data:ident) => {
        ::automatea::json::FromJson::from_json(&$data)
    };

    ($data:ident as $type:ty) => {
        <$type as ::automatea::json::FromJson>::from_json(&$data)
    }
}

#[macro_export]
macro_rules! map {
    {$($key:expr => $val:expr),*} => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key.to_owned(), $val.to_owned());)*

        map
    }}
}