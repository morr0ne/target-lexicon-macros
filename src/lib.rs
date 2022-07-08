use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::LitStr;
use target_lexicon::Triple;

/// Creates a static [Triple](target_lexicon::Triple) at build time.
///
/// # Example
/// ```rust
/// const TARGET: Triple = triple!("x86_64-unknown-linux-gnu");
/// ```
#[proc_macro]
pub fn triple(tokens: TokenStream) -> TokenStream {
    let triple: LitStr = syn::parse_macro_input!(tokens);

    let parsed_triple = Triple::from_str(&triple.value()).expect("Invalid triple value");

    triple_to_tokens(parsed_triple)
}

/*
I am gonna be honest, I am not really proud of this function.
This could be implemented with a macro instead of this questionable brute force approach but it does get the job done.
In the future it's probably a good idea to make this more mantainable, but it was actually faster to do it this way than to figure out
the proper macro_rules syntax.

This is what I tell my self to justify writing all this by hand. If you are reading this please fix it.

P.S.

You may have noticed that there are no test cases... It's 4 am, I haven't slept in 48 hours and I'm fueled by Monster Energy and unrelated hate against nvidia.
Just shut up.
*/
fn triple_to_tokens(triple: Triple) -> TokenStream {
    let architecture = match triple.architecture {
        target_lexicon::Architecture::Unknown => quote! {::target_lexicon::Architecture::Unknown},
        target_lexicon::Architecture::Arm(arm) => match arm {
            target_lexicon::ArmArchitecture::Arm => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Arm)}
            }
            target_lexicon::ArmArchitecture::Armeb => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armeb)}
            }
            target_lexicon::ArmArchitecture::Armv4 => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv4)}
            }
            target_lexicon::ArmArchitecture::Armv4t => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv4t)}
            }
            target_lexicon::ArmArchitecture::Armv5t => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv5t)}
            }
            target_lexicon::ArmArchitecture::Armv5te => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv5te)}
            }
            target_lexicon::ArmArchitecture::Armv5tej => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv5tej)}
            }
            target_lexicon::ArmArchitecture::Armv6 => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv6)}
            }
            target_lexicon::ArmArchitecture::Armv6j => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv6j)}
            }
            target_lexicon::ArmArchitecture::Armv6k => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv6k)}
            }
            target_lexicon::ArmArchitecture::Armv6z => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv6z)}
            }
            target_lexicon::ArmArchitecture::Armv6kz => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv6kz)}
            }
            target_lexicon::ArmArchitecture::Armv6t2 => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv6t2)}
            }
            target_lexicon::ArmArchitecture::Armv6m => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv6m)}
            }
            target_lexicon::ArmArchitecture::Armv7 => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv7)}
            }
            target_lexicon::ArmArchitecture::Armv7a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv7a)}
            }
            target_lexicon::ArmArchitecture::Armv7ve => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv7ve)}
            }
            target_lexicon::ArmArchitecture::Armv7m => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv7m)}
            }
            target_lexicon::ArmArchitecture::Armv7r => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv7r)}
            }
            target_lexicon::ArmArchitecture::Armv7s => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv7s)}
            }
            target_lexicon::ArmArchitecture::Armv8 => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8)}
            }
            target_lexicon::ArmArchitecture::Armv8a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8a)}
            }
            target_lexicon::ArmArchitecture::Armv8_1a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8_1a)}
            }
            target_lexicon::ArmArchitecture::Armv8_2a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8_2a)}
            }
            target_lexicon::ArmArchitecture::Armv8_3a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8_3a)}
            }
            target_lexicon::ArmArchitecture::Armv8_4a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8_4a)}
            }
            target_lexicon::ArmArchitecture::Armv8_5a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8_5a)}
            }
            target_lexicon::ArmArchitecture::Armv8mBase => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8mBase)}
            }
            target_lexicon::ArmArchitecture::Armv8mMain => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8mMain)}
            }
            target_lexicon::ArmArchitecture::Armv8r => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armv8r)}
            }
            target_lexicon::ArmArchitecture::Armebv7r => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Armebv7r)}
            }
            target_lexicon::ArmArchitecture::Thumbeb => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbeb)}
            }
            target_lexicon::ArmArchitecture::Thumbv4t => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv4t)}
            }
            target_lexicon::ArmArchitecture::Thumbv6m => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv6m)}
            }
            target_lexicon::ArmArchitecture::Thumbv7a => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv7a)}
            }
            target_lexicon::ArmArchitecture::Thumbv7em => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv7em)}
            }
            target_lexicon::ArmArchitecture::Thumbv7m => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv7m)}
            }
            target_lexicon::ArmArchitecture::Thumbv7neon => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv7neon)}
            }
            target_lexicon::ArmArchitecture::Thumbv8mBase => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv8mBase)}
            }
            target_lexicon::ArmArchitecture::Thumbv8mMain => {
                quote! {::target_lexicon::Architecture::Arm(:: target_lexicon::ArmArchitecture::Thumbv8mMain)}
            }
            _ => panic!("Unsupported architecture"),
        },
        target_lexicon::Architecture::AmdGcn => quote! {::target_lexicon::Architecture::AmdGcn},
        target_lexicon::Architecture::Aarch64(aarch64) => match aarch64 {
            target_lexicon::Aarch64Architecture::Aarch64 => {
                quote! {::target_lexicon::Architecture::Aarch64(::target_lexicon::Aarch64Architecture::Aarch64)}
            }
            target_lexicon::Aarch64Architecture::Aarch64be => {
                quote! {::target_lexicon::Architecture::Aarch64(::target_lexicon::Aarch64Architecture::Aarch64be)}
            }
            _ => panic!("Unsupported architecture"),
        },
        target_lexicon::Architecture::Asmjs => quote! {::target_lexicon::Architecture::Asmjs},
        target_lexicon::Architecture::Avr => quote! {::target_lexicon::Architecture::Avr},
        target_lexicon::Architecture::Bpfeb => quote! {::target_lexicon::Architecture::Bpfeb},
        target_lexicon::Architecture::Bpfel => quote! {::target_lexicon::Architecture::Bpfel},
        target_lexicon::Architecture::Hexagon => quote! {::target_lexicon::Architecture::Hexagon},
        target_lexicon::Architecture::X86_32(x86_32) => match x86_32 {
            target_lexicon::X86_32Architecture::I386 => {
                quote! {::target_lexicon::Architecture::X86_32(::target_lexicon::X86_32Architecture::I386)}
            }
            target_lexicon::X86_32Architecture::I586 => {
                quote! {::target_lexicon::Architecture::X86_32(::target_lexicon::X86_32Architecture::I586)}
            }
            target_lexicon::X86_32Architecture::I686 => {
                quote! {::target_lexicon::Architecture::X86_32(::target_lexicon::X86_32Architecture::I686)}
            }
            _ => panic!("Unsupported architecture"),
        },
        target_lexicon::Architecture::M68k => quote! {::target_lexicon::Architecture::M68k},
        target_lexicon::Architecture::Mips32(mips32) => match mips32 {
            target_lexicon::Mips32Architecture::Mips => {
                quote! {::target_lexicon::Architecture::Mips32(:: target_lexicon::Mips32Architecture::Mips)}
            }
            target_lexicon::Mips32Architecture::Mipsel => {
                quote! {::target_lexicon::Architecture::Mips32(:: target_lexicon::Mips32Architecture::Mipsel)}
            }
            target_lexicon::Mips32Architecture::Mipsisa32r6 => {
                quote! {::target_lexicon::Architecture::Mips32(:: target_lexicon::Mips32Architecture::Mipsisa32r6)}
            }
            target_lexicon::Mips32Architecture::Mipsisa32r6el => {
                quote! {::target_lexicon::Architecture::Mips32(:: target_lexicon::Mips32Architecture::Mipsisa32r6el)}
            }
            _ => panic!("Unsupported architecture"),
        },
        target_lexicon::Architecture::Mips64(mips64) => match mips64 {
            target_lexicon::Mips64Architecture::Mips64 => {
                quote! {::target_lexicon::Architecture::Mips64(::target_lexicon::Mips64Architecture::Mips64)}
            }
            target_lexicon::Mips64Architecture::Mips64el => {
                quote! {::target_lexicon::Architecture::Mips64(::target_lexicon::Mips64Architecture::Mips64el)}
            }
            target_lexicon::Mips64Architecture::Mipsisa64r6 => {
                quote! {::target_lexicon::Architecture::Mips64(::target_lexicon::Mips64Architecture::Mipsisa64r6)}
            }
            target_lexicon::Mips64Architecture::Mipsisa64r6el => {
                quote! {::target_lexicon::Architecture::Mips64(::target_lexicon::Mips64Architecture::Mipsisa64r6el)}
            }
            _ => panic!("Unsupported architecture"),
        },
        target_lexicon::Architecture::Msp430 => quote! {::target_lexicon::Architecture::Msp430},
        target_lexicon::Architecture::Nvptx64 => quote! {::target_lexicon::Architecture::Nvptx64},
        target_lexicon::Architecture::Powerpc => quote! {::target_lexicon::Architecture::Powerpc},
        target_lexicon::Architecture::Powerpc64 => {
            quote! {::target_lexicon::Architecture::Powerpc64}
        }
        target_lexicon::Architecture::Powerpc64le => {
            quote! {::target_lexicon::Architecture::Powerpc64le}
        }
        target_lexicon::Architecture::Riscv32(riscv32) => match riscv32 {
            target_lexicon::Riscv32Architecture::Riscv32 => {
                quote! {::target_lexicon::Architecture::Riscv32(::target_lexicon::Riscv32Architecture::Riscv32)}
            }
            target_lexicon::Riscv32Architecture::Riscv32gc => {
                quote! {::target_lexicon::Architecture::Riscv32(::target_lexicon::Riscv32Architecture::Riscv32gc)}
            }
            target_lexicon::Riscv32Architecture::Riscv32i => {
                quote! {::target_lexicon::Architecture::Riscv32(::target_lexicon::Riscv32Architecture::Riscv32i)}
            }
            target_lexicon::Riscv32Architecture::Riscv32im => {
                quote! {::target_lexicon::Architecture::Riscv32(::target_lexicon::Riscv32Architecture::Riscv32im)}
            }
            target_lexicon::Riscv32Architecture::Riscv32imac => {
                quote! {::target_lexicon::Architecture::Riscv32(::target_lexicon::Riscv32Architecture::Riscv32imac)}
            }
            target_lexicon::Riscv32Architecture::Riscv32imc => {
                quote! {::target_lexicon::Architecture::Riscv32(::target_lexicon::Riscv32Architecture::Riscv32imc)}
            }
            _ => panic!("Unsupported architecture"),
        },
        target_lexicon::Architecture::Riscv64(riscv64) => match riscv64 {
            target_lexicon::Riscv64Architecture::Riscv64 => {
                quote! {::target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64)}
            }
            target_lexicon::Riscv64Architecture::Riscv64gc => {
                quote! {::target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64gc)}
            }
            target_lexicon::Riscv64Architecture::Riscv64imac => {
                quote! {::target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64imac)}
            }
            _ => panic!("Unsupported architecture"),
        },
        target_lexicon::Architecture::S390x => quote! {::target_lexicon::Architecture::S390x},
        target_lexicon::Architecture::Sparc => quote! {::target_lexicon::Architecture::Sparc},
        target_lexicon::Architecture::Sparc64 => quote! {::target_lexicon::Architecture::Sparc64},
        target_lexicon::Architecture::Sparcv9 => quote! {::target_lexicon::Architecture::Sparcv9},
        target_lexicon::Architecture::Wasm32 => quote! {::target_lexicon::Architecture::Wasm32},
        target_lexicon::Architecture::Wasm64 => quote! {::target_lexicon::Architecture::Wasm64},
        target_lexicon::Architecture::X86_64 => {
            quote! {::target_lexicon::Architecture::X86_64}
        }
        target_lexicon::Architecture::XTensa => quote! {::target_lexicon::Architecture::XTensa},
        _ => panic!("Unsupported architecture"),
    };

    let vendor = match triple.vendor {
        target_lexicon::Vendor::Unknown => {
            quote! {::target_lexicon::Vendor::Unknown}
        }
        target_lexicon::Vendor::Amd => quote! {::target_lexicon::Vendor::Amd},
        target_lexicon::Vendor::Apple => quote! {::target_lexicon::Vendor::Apple},
        target_lexicon::Vendor::Espressif => quote! {::target_lexicon::Vendor::EspressifEspressif},
        target_lexicon::Vendor::Experimental => quote! {::target_lexicon::Vendor::Experimental},
        target_lexicon::Vendor::Fortanix => quote! {::target_lexicon::Vendor::Fortanix},
        target_lexicon::Vendor::Kmc => quote! {::target_lexicon::Vendor::Kmc},
        target_lexicon::Vendor::Nintendo => quote! {::target_lexicon::Vendor::Nintendo},
        target_lexicon::Vendor::Nvidia => quote! {::target_lexicon::Vendor::Nvidia},
        target_lexicon::Vendor::Pc => quote! {::target_lexicon::Vendor::Pc},
        target_lexicon::Vendor::Rumprun => quote! {::target_lexicon::Vendor::Rumprun},
        target_lexicon::Vendor::Sun => quote! {::target_lexicon::Vendor::Sun},
        target_lexicon::Vendor::Uwp => quote! {::target_lexicon::Vendor::Uwp},
        target_lexicon::Vendor::Wrs => quote! {::target_lexicon::Vendor::Wrs},
        target_lexicon::Vendor::Custom(custom_vendor) => {
            // TODO: test this cause it doesn't currently work
            let vendor = custom_vendor.as_str();
            quote! {::target_lexicon::Vendor::Unknown(::target_lexicon::CustomVendor::Static(#vendor))}
        }
        _ => panic!("Unsupported vendor"),
    };

    let operating_system = match triple.operating_system {
        target_lexicon::OperatingSystem::Unknown => {
            quote! {::target_lexicon::OperatingSystem::Unknown}
        }
        target_lexicon::OperatingSystem::AmdHsa => {
            quote! {::target_lexicon::OperatingSystem::AmdHsa}
        }
        target_lexicon::OperatingSystem::Bitrig => {
            quote! {::target_lexicon::OperatingSystem::Bitrig}
        }
        target_lexicon::OperatingSystem::Cloudabi => {
            quote! {::target_lexicon::OperatingSystem::Cloudabi}
        }
        target_lexicon::OperatingSystem::Cuda => {
            quote! {::target_lexicon::OperatingSystem::Cuda}
        }
        target_lexicon::OperatingSystem::Darwin => {
            quote! {::target_lexicon::OperatingSystem::Darwin}
        }
        target_lexicon::OperatingSystem::Dragonfly => {
            quote! {::target_lexicon::OperatingSystem::Dragonfly}
        }
        target_lexicon::OperatingSystem::Emscripten => {
            quote! {::target_lexicon::OperatingSystem::Emscripten}
        }
        target_lexicon::OperatingSystem::Espidf => {
            quote! {::target_lexicon::OperatingSystem::Espidf}
        }
        target_lexicon::OperatingSystem::Freebsd => {
            quote! {::target_lexicon::OperatingSystem::Freebsd}
        }
        target_lexicon::OperatingSystem::Fuchsia => {
            quote! {::target_lexicon::OperatingSystem::Fuchsia}
        }
        target_lexicon::OperatingSystem::Haiku => {
            quote! {::target_lexicon::OperatingSystem::Haiku}
        }
        target_lexicon::OperatingSystem::Hermit => {
            quote! {::target_lexicon::OperatingSystem::Hermit}
        }
        target_lexicon::OperatingSystem::Horizon => {
            quote! {::target_lexicon::OperatingSystem::Horizon}
        }
        target_lexicon::OperatingSystem::Illumos => {
            quote! {::target_lexicon::OperatingSystem::Illumos}
        }
        target_lexicon::OperatingSystem::Ios => quote! {::target_lexicon::OperatingSystem::Ios},
        target_lexicon::OperatingSystem::L4re => {
            quote! {::target_lexicon::OperatingSystem::L4re}
        }
        target_lexicon::OperatingSystem::Linux => {
            quote! {::target_lexicon::OperatingSystem::Linux}
        }
        target_lexicon::OperatingSystem::MacOSX {
            major,
            minor,
            patch,
        } => {
            quote! {::target_lexicon::OperatingSystem::MacOSX { major: #major, minor: #minor, patch: #patch }}
        }
        target_lexicon::OperatingSystem::Nebulet => {
            quote! {::target_lexicon::OperatingSystem::Nebulet}
        }
        target_lexicon::OperatingSystem::Netbsd => {
            quote! {::target_lexicon::OperatingSystem::Netbsd}
        }
        target_lexicon::OperatingSystem::None_ => {
            quote! {::target_lexicon::OperatingSystem::None_}
        }
        target_lexicon::OperatingSystem::Openbsd => {
            quote! {::target_lexicon::OperatingSystem::Openbsd}
        }
        target_lexicon::OperatingSystem::Psp => quote! {::target_lexicon::OperatingSystem::Psp},
        target_lexicon::OperatingSystem::Redox => {
            quote! {::target_lexicon::OperatingSystem::Redox}
        }
        target_lexicon::OperatingSystem::Solaris => {
            quote! {::target_lexicon::OperatingSystem::Solaris}
        }
        target_lexicon::OperatingSystem::SolidAsp3 => {
            quote! {::target_lexicon::OperatingSystem::SolidAsp3}
        }
        target_lexicon::OperatingSystem::Tvos => {
            quote! {::target_lexicon::OperatingSystem::Tvos}
        }
        target_lexicon::OperatingSystem::Uefi => {
            quote! {::target_lexicon::OperatingSystem::Uefi}
        }
        target_lexicon::OperatingSystem::VxWorks => {
            quote! {::target_lexicon::OperatingSystem::VxWorks}
        }
        target_lexicon::OperatingSystem::Wasi => {
            quote! {::target_lexicon::OperatingSystem::Wasi}
        }
        target_lexicon::OperatingSystem::Windows => {
            quote! {::target_lexicon::OperatingSystem::Windows}
        }
        _ => panic!("Unsupported operating system"),
    };

    let environment = match triple.environment {
        target_lexicon::Environment::Unknown => quote! {::target_lexicon::Environment::Unknown},
        target_lexicon::Environment::AmdGiz => quote! {::target_lexicon::Environment::AmdGiz},
        target_lexicon::Environment::Android => quote! {::target_lexicon::Environment::Android},
        target_lexicon::Environment::Androideabi => {
            quote! {::target_lexicon::Environment::Androideabi}
        }
        target_lexicon::Environment::Eabi => quote! {::target_lexicon::Environment::Eabi},
        target_lexicon::Environment::Eabihf => quote! {::target_lexicon::Environment::Eabihf},
        target_lexicon::Environment::Gnu => {
            quote! {::target_lexicon::Environment::Gnu}
        }
        target_lexicon::Environment::Gnuabi64 => quote! {::target_lexicon::Environment::Gnuabi64},
        target_lexicon::Environment::Gnueabi => quote! {::target_lexicon::Environment::Gnueabi},
        target_lexicon::Environment::Gnueabihf => quote! {::target_lexicon::Environment::Gnueabihf},
        target_lexicon::Environment::Gnuspe => quote! {::target_lexicon::Environment::Gnuspe},
        target_lexicon::Environment::Gnux32 => quote! {::target_lexicon::Environment::Gnux32},
        target_lexicon::Environment::GnuIlp32 => quote! {::target_lexicon::Environment::GnuIlp32},
        target_lexicon::Environment::HermitKernel => {
            quote! {::target_lexicon::Environment::HermitKernel}
        }
        target_lexicon::Environment::LinuxKernel => {
            quote! {::target_lexicon::Environment::LinuxKernel}
        }
        target_lexicon::Environment::Macabi => quote! {::target_lexicon::Environment::Macabi},
        target_lexicon::Environment::Musl => quote! {::target_lexicon::Environment::Musl},
        target_lexicon::Environment::Musleabi => quote! {::target_lexicon::Environment::Musleabi},
        target_lexicon::Environment::Musleabihf => {
            quote! {::target_lexicon::Environment::Musleabihf}
        }
        target_lexicon::Environment::Muslabi64 => quote! {::target_lexicon::Environment::Muslabi64},
        target_lexicon::Environment::Msvc => quote! {::target_lexicon::Environment::Msvc},
        target_lexicon::Environment::Newlib => quote! {::target_lexicon::Environment::Newlib},
        target_lexicon::Environment::Kernel => quote! {::target_lexicon::Environment::Kernel},
        target_lexicon::Environment::Uclibc => quote! {::target_lexicon::Environment::Uclibc},
        target_lexicon::Environment::Uclibceabi => {
            quote! {::target_lexicon::Environment::Uclibceabi}
        }
        target_lexicon::Environment::Uclibceabihf => {
            quote! {::target_lexicon::Environment::Uclibceabihf}
        }
        target_lexicon::Environment::Sgx => quote! {::target_lexicon::Environment::Sgx},
        target_lexicon::Environment::Sim => quote! {::target_lexicon::Environment::Sim},
        target_lexicon::Environment::Softfloat => quote! {::target_lexicon::Environment::Softfloat},
        target_lexicon::Environment::Spe => quote! {::target_lexicon::Environment::Spe},
        _ => panic!("Unsupported environment"),
    };

    let binary_format = match triple.binary_format {
        target_lexicon::BinaryFormat::Unknown => quote! {::target_lexicon::BinaryFormat::Unknown},
        target_lexicon::BinaryFormat::Elf => quote! {::target_lexicon::BinaryFormat::Elf},
        target_lexicon::BinaryFormat::Coff => quote! {::target_lexicon::BinaryFormat::Coff},
        target_lexicon::BinaryFormat::Macho => quote! {::target_lexicon::BinaryFormat::Macho},
        target_lexicon::BinaryFormat::Wasm => quote! {::target_lexicon::BinaryFormat::Wasm},
        _ => panic!("Unsupported binary format"),
    };

    quote! {
        ::target_lexicon::Triple {
            architecture: #architecture,
            vendor: #vendor,
            operating_system: #operating_system,
            environment: #environment,
            binary_format: #binary_format,
        }
    }
    .into()
}
