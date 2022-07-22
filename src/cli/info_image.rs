use imgurs::ImgurClient;

use super::print_image_info;

pub async fn image_info(client: ImgurClient, id: String) {
    // get a image info from imgur
    let info = client
        .image_info(&id)
        .await
        .expect("send request to imfur api");

    // print image information from imgur
    print_image_info(&info.data);
}
