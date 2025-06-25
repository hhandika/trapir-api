use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) subcommand: MainSubcommand,
}

#[derive(Subcommand)]
pub(crate) enum MainSubcommand {
    #[command(name = "resize", about = "Image resizing functionality")]
    ImgResize(ImgResizeArgs),
}

#[derive(Args)]
pub(crate) struct ImgResizeArgs {
    #[arg(short, long, help = "Path to the image file to analyze")]
    pub(crate) path: String,
    #[arg(short, long, help = "Width to resize the image to")]
    pub(crate) width: Option<u32>,
    #[arg(short, long, help = "Height to resize the image to")]
    pub(crate) height: Option<u32>,
    #[arg(short, long, help = "Output path for the resized image")]
    pub(crate) output: Option<String>,
}
