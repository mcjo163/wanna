#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct WannaCli {
    pub verb: Option<String>,
    pub name: Option<String>,
    pub modifier: Option<String>,
    pub modifier_param: Option<String>,
}
