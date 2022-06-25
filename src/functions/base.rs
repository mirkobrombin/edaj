use crate::internal::exec::*;
use crate::internal::*;
use crate::internal::files::append_file;
use std::path::PathBuf;
use log::warn;

pub fn install_base_packages(kernel: String) {
    let mut kernel_to_install: String = String::new();
    std::fs::create_dir_all("/mnt/etc").unwrap();
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
    if kernel.is_empty() {
        kernel_to_install = "linux".to_string();
    } else {
        match kernel.as_str() {
            "linux" => kernel_to_install = "linux".to_string(),
            "linux-lts" => kernel_to_install = "linux-lts".to_string(),
            "linux-zen" => kernel_to_install = "linux-zen".to_string(),
            "linux-hardened" => kernel_to_install = "linux-hardened".to_string(),
            _ => {
                warn!("Unknown kernel: {}, using default instead", kernel);
                kernel_to_install = "linux".to_string();
            }
        }
    }
    install::install(vec![
        "base",
        kernel_to_install.as_str(),
        "linux-firmware",
        "systemd-sysvcompat",
        "networkmanager",
        "man-db",
        "man-pages",
        "texinfo",
        "micro",
        "sudo",
        "curl",
        "archlinux-keyring",
        "neofetch",
        "btrfs-progs",
        "which",
    ]);
}

pub fn genfstab() {
    exec_eval(
        exec(
            "bash",
            vec![
                String::from("-c"),
                String::from("genfstab -U /mnt >> /mnt/etc/fstab"),
            ],
        ),
        "Generate fstab",
    );
}

pub fn install_bootloader_efi(efidir: PathBuf) {
    install::install(vec!["grub", "efibootmgr", "grub-btrfs", "crystal-grub-theme", "os-prober"]);
    let efidir = std::path::Path::new("/mnt").join(efidir);
    let efi_str = efidir.to_str().unwrap();
    if !std::path::Path::new(&format!("/mnt{efi_str}")).exists() {
        crash(format!("The efidir {efidir:?} doesn't exist"), 1);
    }
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}" , efi_str),
                String::from("--bootloader-id=crystal"),
                String::from("--removable"),
            ],
        ),
        "install grub as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=crystal"),
            ],
        ),
        "install grub as efi without --removable",
    );
    files_eval(
        append_file("/mnt/etc/default/grub", "GRUB_THEME=\"/usr/share/grub/themes/crystal/theme.txt\""),
        "enable crystal grub theme"
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn install_bootloader_legacy(device: PathBuf) {
    install::install(vec!["grub", "grub-btrfs", "crystal-grub-theme", "os-prober"]);
    if !device.exists() {
        crash(format!("The device {device:?} does not exist"), 1);
    }
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![String::from("--target=i386-pc"), device],
        ),
        "install grub as legacy",
    );
    files_eval(
        append_file("/mnt/etc/default/grub", "GRUB_THEME=\"/usr/share/grub/themes/crystal/theme.txt\""),
        "enable crystal grub theme"
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn setup_timeshift() {
    install(vec!["timeshift", "timeshift-autosnap"]);
    exec_eval(
        exec_chroot("timeshift", vec![String::from("--btrfs")]),
        "setup timeshift",
    )
}

pub fn install_homemgr() {
    install(vec!["nix"]);
}

pub fn install_flatpak() {
    install(vec!["flatpak"]);
    exec_eval(
        exec_chroot("flatpak", vec![String::from("remote-add"), String::from("--if-not-exists"), String::from("flathub"), String::from("https://flathub.org/repo/flathub.flatpakrepo")]),
        "add flathub remote",
    )
}
