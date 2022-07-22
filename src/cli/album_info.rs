use imgurs::ImgurClient;

use super::print_image_info;

pub async fn album_info(client: ImgurClient, id: String) {
    // get a image info from imgur
    let info = client
        .album_info(&id)
        .await
        .expect("send request to imgur api");
    
    print!("got album: {:?}", info);

    // print image information from imgur
    for image in info.data.images {
        print_image_info(&image);
        println!("")
    }
}
