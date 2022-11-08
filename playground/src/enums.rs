enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String)
}

pub fn run() {
    let linux = OS::Linux(1991, String::from("linus"));
    let windows = OS::Windows(1985, String::from("Microsoft"));
    let mac = OS::Mac(2001, String::from("Apple"));

    print_os_info(windows);
    print_os_info(mac);
    print_os_info(linux);
}

fn print_os_info(os: OS) {
    match os {
        OS::Windows(year, who ) => {
            println!("Windows : First release in {} by {}", year, who);
        }

        OS::Mac(year, who ) => {
            println!("Mac : First release in {} by {}", year, who);
        }

        OS::Linux(year, who ) => {
            println!("Linux : First release in {} by {}", year, who);
        }
    }
}
