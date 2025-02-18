use bollard::container::{ListContainersOptions, StartContainerOptions, StopContainerOptions};
use bollard::errors::Error;
use bollard::image::{CreateImageOptions, ListImagesOptions};
use bollard::models::{ContainerSummary, ImageSummary};
use bollard::Docker;
use futures_util::TryStreamExt;
pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new() -> Self {
        let docker = Docker::connect_with_unix(
            "/Users/priyadav/.docker/run/docker.sock",
            120, // Timeout in seconds
            bollard::API_DEFAULT_VERSION,
        )
        .expect("Failed to connect to Docker");

        Self { docker: docker }
    }

    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerSummary>, Error> {
        let options = Some(ListContainersOptions::<String> {
            all,
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await?;
        Ok(containers)
    }

    pub async fn list_images(&self) -> Result<Vec<ImageSummary>, Error> {
        let options = Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        });

        let images = self.docker.list_images(options).await?;
        Ok(images)
    }

    pub async fn start_container(&self, container_name: &str) -> Result<(), Error> {
        self.docker
            .start_container(container_name, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

    pub async fn stop_container(&self, container_name: &str) -> Result<(), Error> {
        let options = Some(StopContainerOptions { t: 30 });
        self.docker.stop_container(container_name, options).await?;
        Ok(())
    }

    pub async fn pull_image(&self, image_name: &str) -> Result<(), Error> {
        let options = Some(CreateImageOptions {
            from_image: image_name,
            ..Default::default()
        });

        let mut stream = self.docker.create_image(options, None, None);

        while let Some(msg) = stream.try_next().await? {
            if let Some(status) = msg.status {
                println!("{}", status);
            }
        }

        Ok(())
    }
}
