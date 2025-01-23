use crate::step::System;

pub struct PackageManager {
    pub install_cmd: &'static str,
}

impl PackageManager {
    pub fn get_by_system(system: &System) -> Self {
        PackageManager {
            install_cmd: match system {
                System::Mac => "HOMEBREW_COLOR=1 brew install",
                System::Arch => "pacman -Syu --noconfirm",
                System::Debian => "sudo apt-get install -y",
                System::Fedora => "sudo dnf install -y",
            },
        }
    }
}
