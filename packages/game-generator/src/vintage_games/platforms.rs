//! Platform information and functions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub id: u32,
    pub name: &'static str,
    pub abbreviation: Option<&'static str>,
    pub deck: Option<&'static str>,
    pub install_base: Option<u64>,
    pub original_price: Option<&'static str>,
    pub release_date: Option<&'static str>,
    pub online_support: Option<bool>,
}

/// Platform information for correlation and context
pub const PLATFORM_INFO: &[PlatformInfo] = &[
    PlatformInfo {
        id: 1,
        name: r#"Amiga"#,
        abbreviation: Some(r#"AMI"#),
        deck: Some(
            r#"The Amiga was a personal computer from Commodore that was released in a variety of different configurations."#,
        ),
        install_base: Some(6000000),
        original_price: Some(r#"1285.00"#),
        release_date: Some(r#"1985-07-23 00:00:00"#),
        online_support: Some(true),
    },
    PlatformInfo {
        id: 3,
        name: r#"Game Boy"#,
        abbreviation: Some(r#"GB"#),
        deck: Some(
            r#"Nintendo's first handheld gaming console was immensely popular among gamers, selling millions. Despite its grayscale color scheme, it still got support from developers and publishers."#,
        ),
        install_base: Some(118690000),
        original_price: Some(r#"179.00"#),
        release_date: Some(r#"1989-04-21 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 4,
        name: r#"Game Boy Advance"#,
        abbreviation: Some(r#"GBA"#),
        deck: Some(
            r#"The third platform in the Game Boy line, the Game Boy Advance was offered in a multitude of colors and had three hardware offerings, the sideways Game Boy Advance, the flip Game Boy Advance SP and the tiny Game Boy Advance Micro."#,
        ),
        install_base: Some(81510000),
        original_price: Some(r#"100.00"#),
        release_date: Some(r#"2001-03-21 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 6,
        name: r#"Genesis"#,
        abbreviation: Some(r#"GEN"#),
        deck: Some(
            r#"After the cult success of their 8-bit Master System, Sega decided to give gamers a taste of their arcade capabilities with a 16-bit console. Known worldwide as the Mega Drive but called Genesis in the US, it provided graphics and sound a couple of steps below their popular System 16 arcade cabinets. The Mega Drive/Genesis turned out to be Sega's most successful console."#,
        ),
        install_base: Some(30750000),
        original_price: Some(r#"189.00"#),
        release_date: Some(r#"1988-10-29 00:00:00"#),
        online_support: Some(true),
    },
    PlatformInfo {
        id: 11,
        name: r#"Amstrad CPC"#,
        abbreviation: Some(r#"CPC"#),
        deck: Some(
            r#"The Amstrad CPC (Colour Personal Computer) was a series of 8-bit personal computers developed by Amstrad between 1984 and 1990. During its lifetime, approximately 3 million CPCs were sold."#,
        ),
        install_base: Some(3000000),
        original_price: Some(r#"800.00"#),
        release_date: Some(r#"1984-06-21 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 12,
        name: r#"Apple II"#,
        abbreviation: Some(r#"APL2"#),
        deck: Some(
            r#"Introduced at the West Coast Computer Faire in 1977, the Apple II was the first mass produced microcomputer on the market, becoming very popular in classrooms throughout the 1980s and 1990s. Somewhere between five and six million Apple II series computers were sold."#,
        ),
        install_base: Some(6000000),
        original_price: Some(r#"1300.00"#),
        release_date: Some(r#"1977-06-30 00:00:00"#),
        online_support: Some(true),
    },
    PlatformInfo {
        id: 14,
        name: r#"Commodore 64"#,
        abbreviation: Some(r#"C64"#),
        deck: Some(
            r#"The Commodore 64 personal computer dominated the market from 1983-1985, and stands as one of the best-selling personal computers of all time."#,
        ),
        install_base: Some(17000000),
        original_price: Some(r#"595.00"#),
        release_date: Some(r#"1982-08-31 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 15,
        name: r#"MSX"#,
        abbreviation: Some(r#"MSX"#),
        deck: Some(
            r#"MSX is a standardized home computer architecture. It was popular in Japan, South Korea, Brazil, Netherlands, France, Spain, Finland, Arabian Gulf countries and former Soviet Union during the 1980s. Like the PC of today, the MSX computers were manufactured by many different companies."#,
        ),
        install_base: Some(5000000),
        original_price: Some(r#"0.00"#),
        release_date: Some(r#"1983-06-16 00:00:00"#),
        online_support: Some(true),
    },
    PlatformInfo {
        id: 38,
        name: r#"Apple IIgs"#,
        abbreviation: Some(r#"A2GS"#),
        deck: Some(
            r#"The Apple ][gs - which stood for "Graphics and Sound" - was Apple's upgraded version of the popular Apple ][ line of computers.  The system was capable of playing standard Apple ][ games, as well as games made specifically for the GS."#,
        ),
        install_base: None,
        original_price: Some(r#"1000.00"#),
        release_date: Some(r#"1986-11-18 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 39,
        name: r#"Amiga CD32"#,
        abbreviation: Some(r#"CD32"#),
        deck: Some(
            r#"The Amiga CD32 was Commodore's attempt at a gaming console and what turned out to be their swan song. The majority of its library were upgraded Amiga games."#,
        ),
        install_base: Some(100000),
        original_price: Some(r#"250.00"#),
        release_date: Some(r#"1993-09-17 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 57,
        name: r#"Game Boy Color"#,
        abbreviation: Some(r#"GBC"#),
        deck: Some(
            r#"Nintendo's successor to the Game Boy, featuring a color screen and backwards compatibility for all previous Game Boy titles."#,
        ),
        install_base: Some(118690000),
        original_price: Some(r#"70.00"#),
        release_date: Some(r#"1998-10-31 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 75,
        name: r#"PC-FX"#,
        abbreviation: Some(r#"PCFX"#),
        deck: Some(
            r#"The NEC PC-FX was a console designed in the form of a PC and planned to be upgradable.  It failed due to lack of 3D graphical power and little developer support.  The PC-FX is known for its large percentage of adult titles and was NEC Corporation's last gaming console."#,
        ),
        install_base: Some(300000),
        original_price: Some(r#"500.00"#),
        release_date: Some(r#"1994-12-31 00:00:00"#),
        online_support: None,
    },
    PlatformInfo {
        id: 84,
        name: r#"Arcade"#,
        abbreviation: Some(r#"ARC"#),
        deck: Some(
            r#"Stand-alone machines specialized for individual games. Arcades began the game industry and peaked in popularity before home consoles took over the gaming public. Arcade games usually cost 25 cents, or 100 yen, per play. Known for the most cutting-edge technology of their time, arcades have the largest video game library, and greatest variety of control methods, of any platform."#,
        ),
        install_base: None,
        original_price: Some(r#"20000.00"#),
        release_date: Some(r#"1971-08-31 00:00:00"#),
        online_support: Some(true),
    },
    PlatformInfo {
        id: 94,
        name: r#"PC"#,
        abbreviation: Some(r#"PC"#),
        deck: Some(
            r#"The PC (Personal Computer) is a highly configurable and upgradable gaming platform that, among home systems, sports the widest variety of control methods, largest library of games, and cutting edge graphics and sound capabilities."#,
        ),
        install_base: Some(1250000000),
        original_price: Some(r#"1565.00"#),
        release_date: Some(r#"1981-08-12 00:00:00"#),
        online_support: Some(true),
    },
    PlatformInfo {
        id: 109,
        name: r#"NEC PC-8801"#,
        abbreviation: Some(r#"PC88"#),
        deck: Some(
            r#"This NEC computer was released in 1981 and is more commonly referred to as the PC-88."#,
        ),
        install_base: None,
        original_price: None,
        release_date: Some(r#"1981-01-31 00:00:00"#),
        online_support: Some(true),
    },
    PlatformInfo {
        id: 112,
        name: r#"NEC PC-9801"#,
        abbreviation: Some(r#"PC98"#),
        deck: Some(
            r#"A 16/32-bit Japanese personal computer system launched by NEC in 1982. It was the most successful computer platform in Japan and one of the best-selling computer systems of the 20th century. It has a very large video game library with thousands of titles, the majority of which were never released outside Japan."#,
        ),
        install_base: Some(18000000),
        original_price: Some(r#"1400.00"#),
        release_date: Some(r#"1982-10-31 00:00:00"#),
        online_support: Some(true),
    },
];

/// Get platform info by name
pub fn get_platform_info(name: &str) -> Option<&'static PlatformInfo> {
    PLATFORM_INFO.iter().find(|p| {
        p.name.eq_ignore_ascii_case(name)
            || p.abbreviation
                .as_ref()
                .is_some_and(|abbr| abbr.eq_ignore_ascii_case(name))
    })
}

/// Get platforms sorted by install base
pub fn platforms_by_popularity() -> Vec<&'static PlatformInfo> {
    let mut platforms: Vec<&'static PlatformInfo> = PLATFORM_INFO.iter().collect();
    platforms.sort_by(|a, b| {
        let a_base = a.install_base.unwrap_or(0);
        let b_base = b.install_base.unwrap_or(0);
        b_base.cmp(&a_base)
    });
    platforms
}

/// Get platforms from a specific era
pub fn platforms_by_release_year(start_year: i32, end_year: i32) -> Vec<&'static PlatformInfo> {
    PLATFORM_INFO
        .iter()
        .filter(|p| {
            if let Some(date_str) = p.release_date {
                // Parse year from date string (format: "YYYY-MM-DD HH:MM:SS")
                if let Some(year_str) = date_str.split('-').next()
                    && let Ok(year) = year_str.parse::<i32>()
                {
                    return year >= start_year && year <= end_year;
                }
            }
            false
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_lookup() {
        // Test that we can find platforms by name and abbreviation
        let genesis = get_platform_info("Genesis");
        assert!(genesis.is_some());

        let amiga = get_platform_info("Amiga");
        assert!(amiga.is_some());

        // Test abbreviation lookup
        let gb = get_platform_info("GB");
        assert!(gb.is_some());
        assert_eq!(gb.unwrap().name, "Game Boy");
    }
}
