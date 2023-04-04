# rusty-hue-controller

docker create -p 8080:8080 --restart unless-stopped --name rusty_hue mazhewitt/rusty_hue_controller

docker start rusty_hue

docker exec -it rusty_hue /app/register_hue
