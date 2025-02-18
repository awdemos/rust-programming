mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};
use docker::DockerClient;

#[tokio::main]
async fn main() {
    // Parse the CLI input
    let args = Cli::parse();
    let docker_client = DockerClient::new();

    // Handle the commands
    match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all } => {
                println!("Printing containers:");
                match docker_client.list_containers(all).await {
                    Ok(containers) => {
                        for container in containers {
                            println!(
                                "{}\t{}\t{}",
                                container.id.unwrap_or_default(),
                                container.names.unwrap_or_default().join(","),
                                container.status.unwrap_or_default()
                            );
                        }
                    }
                    Err(e) => eprintln!("Error listing containers: {}", e),
                }
            }
            ListCommands::Images => {
                println!("Printing images:");
                match docker_client.list_images().await {
                    Ok(images) => {
                        for image in images {
                            println!("{}\t{}", image.id, image.repo_tags.join(","));
                        }
                    }
                    Err(e) => eprintln!("Error listing images: {}", e),
                }
            }
        },
        Command::Start { container_name } => {
            println!("Starting container: {}", container_name);
            match docker_client.start_container(&container_name).await {
                Ok(_) => println!("Container started successfully"),
                Err(e) => eprintln!("Error starting container: {}", e),
            }
        }
        Command::Stop { container_name } => {
            println!("Stopping container {}", container_name);
            match docker_client.stop_container(&container_name).await {
                Ok(_) => println!("Container stopped successfully"),
                Err(e) => eprintln!("Error stopping container: {}", e),
            }
        }
        Command::Pull { image_name } => {
            println!("Pulling image {}", image_name);
            match docker_client.pull_image(&image_name).await {
                Ok(_) => println!("Image pulled successfully"),
                Err(e) => eprintln!("Error pulling image: {}", e),
            }
        }
    }
}
