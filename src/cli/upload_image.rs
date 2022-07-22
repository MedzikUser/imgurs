use imgurs::ImgurClient;
use notify_rust::Notification;

use crate::{
    cli::{clipboard::set_clipboard, print_image_info},
    config::toml,
};

// show notification
macro_rules! notify (
    ($notification: expr) => (
        if toml::parse().notification.enabled {
            $notification.show().expect("send notification");
        }
    );
);

pub async fn upload_image(client: ImgurClient, path: String) {
    // parse configuration file
    let config = toml::parse();

    // upload a image to imgur
    let mut i = client.upload_image(&path).await.unwrap_or_else(|err| {
        notify!(Notification::new()
            .summary("Error!")
            .body(&format!("Error: {}", err))
            .appname("Imgurs")); // I don't think you can set it to error

        panic!("send request to imgur api: {}", err)
    });

    // change domain to proxy (to be set in config)
    if config.imgur.image_cdn != "i.imgur.com" {
        i.data.link = i.data.link.replace("i.imgur.com", &config.imgur.image_cdn)
    }

    // print image information from imgur
    print_image_info(&i.data);

    // send notification that the image has been uploaded
    notify!(Notification::new()
        .summary("Imgurs")
        .body(&format!("Uploaded {}", i.data.link)));

    // if enabled copy link to clipboard
    if config.clipboard.enabled {
        set_clipboard(&i.data.link)
    }
}
