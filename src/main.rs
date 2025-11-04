use rodio::{Source, source::noise::{Blue, Brownian, Pink, Velvet, Violet, WhiteGaussian, WhiteTriangular, WhiteUniform}};
use std::{time::Duration};
use clap::Parser;
use clap::ValueEnum;
// These are the names of the different 
// types of noise generators that rodio supports
#[derive(Debug, Clone, ValueEnum)]
pub enum NoiseFlavor {
    White,
    Gaussian,
    WhiteTriangular,
    Pink,
    Blue,
    Violet,
    Brownian,
    Velvet
}

// We only support one sampling rate. Mono 441kHz
const FS: u32 = 44100;

pub struct NoiseRequest {
    flavor: NoiseFlavor,
    amplitude: f32,
    duration: Duration
}

#[derive(Parser, Debug)]
#[command(name="wn", version, about = "Generate noise", long_about=None)]
struct Args {
    // The type of noise to play
    #[arg(short, long)]
    noise: NoiseFlavor,

    // The amplitude of the noise
    #[arg(short, long)]
    amplitude: f32,

    // The duration of the noise
    #[arg(short, long)]
    duration: String
}

fn play_noise(request: &NoiseRequest) {
    let stream_handle=rodio::OutputStreamBuilder::open_default_stream().expect("failed to open stream");

    let noise_source: Box<dyn rodio::Source<Item = f32> + Send> = match request.flavor {
        NoiseFlavor::White => Box::new(WhiteUniform::new(FS)),
        NoiseFlavor::Gaussian => Box::new(WhiteGaussian::new(FS)),
        NoiseFlavor::WhiteTriangular => Box::new(WhiteTriangular::new(FS)),
        NoiseFlavor::Pink => Box::new(Pink::new(FS)),
        NoiseFlavor::Violet => Box::new(Violet::new(FS)),
        NoiseFlavor::Brownian => Box::new(Brownian::new(FS)),
        NoiseFlavor::Velvet => Box::new(Velvet::new(FS)),
        NoiseFlavor::Blue => Box::new(Blue::new(FS))
    };

    stream_handle.mixer().add(noise_source.amplify(request.amplitude));
    std::thread::sleep(request.duration);
}

/*
    let request = NoiseRequest{
        flavor: NoiseFlavor::Blue, 
        amplitude: 0.10, 
        duration: Duration::from_secs(10)};
    play_noise(&request);

*/

fn main() {
    let args = Args::parse();
    let request = NoiseRequest{
        flavor: args.noise, 
        amplitude: args.amplitude, 
        duration: Duration::from_secs_f32(args.duration.parse().unwrap())};
    play_noise(&request);
}
