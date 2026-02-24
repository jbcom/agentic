//! Game data and functions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TimelineGame {
    pub id: u32,
    pub year: i32,
    pub genre: &'static str,
    pub name: &'static str,
    pub deck: Option<&'static str>,
    pub platforms: &'static [&'static str],
    pub developer: Option<&'static str>,
    pub image_urls: ImageUrls,
    pub site_url: &'static str,
}

// Manual Serialize/Deserialize implementation for TimelineGame
impl Serialize for TimelineGame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TimelineGame", 9)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("year", &self.year)?;
        state.serialize_field("genre", &self.genre)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("deck", &self.deck)?;
        state.serialize_field("platforms", &self.platforms.to_vec())?;
        state.serialize_field("developer", &self.developer)?;
        state.serialize_field("image_urls", &self.image_urls)?;
        state.serialize_field("site_url", &self.site_url)?;
        state.end()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrls {
    pub icon: Option<&'static str>,
    pub medium: Option<&'static str>,
    pub screen: Option<&'static str>,
    pub screen_large: Option<&'static str>,
    pub small: Option<&'static str>,
    pub super_url: Option<&'static str>,
    pub thumb: Option<&'static str>,
    pub tiny: Option<&'static str>,
    pub original: &'static str,
}

/// Timeline of exemplar games organized by year and genre
pub const TIMELINE_GAMES: &[TimelineGame] = &[
    TimelineGame {
        id: 86,
        year: 1984,
        genre: r#"Action"#,
        name: r#"Balloon Fight"#,
        deck: Some(
            r#"Balloon Fight is an arcade-style platform action game for 1-2 players that is similar to Joust. Players take to the sky and try to pop their foes' balloons before their own get popped."#,
        ),
        platforms: &[
            r#"Game Boy Advance"#,
            r#"Nintendo Entertainment System"#,
            r#"Wii Shop"#,
            r#"NEC PC-8801"#,
            r#"Sharp X1"#,
            r#"Nintendo 3DS eShop"#,
            r#"Wii U"#,
            r#"Nintendo Switch"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/16/165930/2365659-balloonfight_nes__usa_front.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/balloon-fight/3030-86/"#,
    },
    TimelineGame {
        id: 101,
        year: 1989,
        genre: r#"Action"#,
        name: r#"Corsarios"#,
        deck: Some(
            r#"Battle your way through an island full of pirates and then battle your way through a ship full of pirates to rescue the damsel in distress."#,
        ),
        platforms: &[
            r#"Amiga"#,
            r#"Amstrad CPC"#,
            r#"Atari ST"#,
            r#"MSX"#,
            r#"ZX Spectrum"#,
            r#"PC"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/0/7465/1708641-corsarios_01.png"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/0/7465/1708641-corsarios_01.png"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/0/7465/1708641-corsarios_01.png"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/0/7465/1708641-corsarios_01.png"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/0/7465/1708641-corsarios_01.png"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/0/7465/1708641-corsarios_01.png"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/0/7465/1708641-corsarios_01.png"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/0/7465/1708641-corsarios_01.png"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/0/7465/1708641-corsarios_01.png"#,
        },
        site_url: r#"https://www.giantbomb.com/corsarios/3030-101/"#,
    },
    TimelineGame {
        id: 15,
        year: 1985,
        genre: r#"Action"#,
        name: r#"Kampfgruppe"#,
        deck: Some(
            r#"Kampfgruppe is a WWII strategy game by SSI. The german word Kampfgruppe refers to a flexible combat formation of any kind, and more specifically to those formations used by Germany during WWII."#,
        ),
        platforms: &[
            r#"Amiga"#,
            r#"Apple II"#,
            r#"Commodore 64"#,
            r#"Atari 8-bit"#,
            r#"PC"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/0/238/709028-kampfgruppe_2.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/0/238/709028-kampfgruppe_2.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/kampfgruppe/3030-15/"#,
    },
    TimelineGame {
        id: 33,
        year: 1994,
        genre: r#"Action"#,
        name: r#"Slam City with Scottie Pippen"#,
        deck: Some(
            r#"A one-on-one streetball game that utilized full-motion video sequences and simplistic controls for gameplay."#,
        ),
        platforms: &[r#"Sega CD"#, r#"Sega 32X"#, r#"PC"#],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/8/87790/2958020-box_slamcity.png"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/8/87790/2958020-box_slamcity.png"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/8/87790/2958020-box_slamcity.png"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/8/87790/2958020-box_slamcity.png"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/8/87790/2958020-box_slamcity.png"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/8/87790/2958020-box_slamcity.png"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/8/87790/2958020-box_slamcity.png"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/8/87790/2958020-box_slamcity.png"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/8/87790/2958020-box_slamcity.png"#,
        },
        site_url: r#"https://www.giantbomb.com/slam-city-with-scottie-pippen/3030-33/"#,
    },
    TimelineGame {
        id: 4,
        year: 1986,
        genre: r#"Action"#,
        name: r#"The Chessmaster 2000"#,
        deck: Some(
            r#"One of the most well-known comptuer chess games of the 1980's, The Chessmaster 2000 features an advanced chess engine by David Kittinger along with numerous training tools and mouse controls."#,
        ),
        platforms: &[
            r#"Amiga"#,
            r#"Amstrad CPC"#,
            r#"Apple II"#,
            r#"Atari ST"#,
            r#"Commodore 64"#,
            r#"MSX"#,
            r#"ZX Spectrum"#,
            r#"Mac"#,
            r#"Atari 8-bit"#,
            r#"PC"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/7/73970/3436007-chessmaster_2000_a800_1_1.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/the-chessmaster-2000/3030-4/"#,
    },
    TimelineGame {
        id: 1,
        year: 1992,
        genre: r#"Action"#,
        name: r#"Desert Strike: Return to the Gulf"#,
        deck: Some(
            r#"A top-down isometric helicopter shoot 'em up originally for the Sega Genesis, which was later ported to a variety of platforms. It is best known for its open-ended mission design and was followed by several sequels."#,
        ),
        platforms: &[
            r#"Amiga"#,
            r#"Game Boy"#,
            r#"Game Boy Advance"#,
            r#"Game Gear"#,
            r#"Genesis"#,
            r#"Atari Lynx"#,
            r#"Sega Master System"#,
            r#"Super Nintendo Entertainment System"#,
            r#"PC"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/9/93770/2370498-genesis_desertstrike_2__1_.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/desert-strike-return-to-the-gulf/3030-1/"#,
    },
    TimelineGame {
        id: 30,
        year: 1988,
        genre: r#"Action"#,
        name: r#"Romantic Encounters at the Dome"#,
        deck: Some(r#"Romantic Encounters at the Dome is a text based game."#),
        platforms: &[r#"Amiga"#, r#"PC"#],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/10/103881/1796224-re.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/10/103881/1796224-re.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/10/103881/1796224-re.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/10/103881/1796224-re.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/10/103881/1796224-re.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/10/103881/1796224-re.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/10/103881/1796224-re.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/10/103881/1796224-re.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/10/103881/1796224-re.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/romantic-encounters-at-the-dome/3030-30/"#,
    },
    TimelineGame {
        id: 78,
        year: 1987,
        genre: r#"Action"#,
        name: r#"Deflektor"#,
        deck: Some(
            r#"A puzzle game in which each of the 60 screens has a laser and a target and the aim is to connect the two using optical devices like mirrors."#,
        ),
        platforms: &[
            r#"Amiga"#,
            r#"Game Boy Advance"#,
            r#"Amstrad CPC"#,
            r#"Atari ST"#,
            r#"Commodore 64"#,
            r#"ZX Spectrum"#,
            r#"Sharp X68000"#,
            r#"NEC PC-8801"#,
            r#"NEC PC-9801"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/7/73970/3116917-deflektor_amiga_1_1.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/deflektor/3030-78/"#,
    },
    TimelineGame {
        id: 80,
        year: 1990,
        genre: r#"Action"#,
        name: r#"Ruff and Reddy in the Space Adventure"#,
        deck: None,
        platforms: &[
            r#"Amiga"#,
            r#"Amstrad CPC"#,
            r#"Atari ST"#,
            r#"Commodore 64"#,
            r#"ZX Spectrum"#,
            r#"Atari 8-bit"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/16/164924/2999465-6664166472-87863.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/16/164924/2999465-6664166472-87863.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/ruff-and-reddy-in-the-space-adventure/3030-80/"#,
    },
    TimelineGame {
        id: 311,
        year: 1981,
        genre: r#"Action"#,
        name: r#"Donkey Kong"#,
        deck: Some(
            r#"Control Jumpman (later known as Mario) and save his girlfriend, Pauline, from the evil barrel-tossing gorilla known as Donkey Kong. Designed by Shigeru Miyamoto, this was the game that set the template for the platformer genre."#,
        ),
        platforms: &[
            r#"Game Boy Advance"#,
            r#"Amstrad CPC"#,
            r#"Apple II"#,
            r#"Commodore 64"#,
            r#"MSX"#,
            r#"ZX Spectrum"#,
            r#"Nintendo Entertainment System"#,
            r#"Atari 8-bit"#,
            r#"VIC-20"#,
            r#"Atari 2600"#,
            r#"ColecoVision"#,
            r#"TI-99/4A"#,
            r#"Intellivision"#,
            r#"Atari 7800"#,
            r#"Arcade"#,
            r#"Wii Shop"#,
            r#"Famicom Disk System"#,
            r#"PC"#,
            r#"Nintendo 3DS eShop"#,
            r#"Wii U"#,
            r#"Nintendo Switch"#,
            r#"Coleco Adam"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/9/93770/2361668-nes_donkeykong.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/9/93770/2361668-nes_donkeykong.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/donkey-kong/3030-311/"#,
    },
    TimelineGame {
        id: 197,
        year: 1982,
        genre: r#"Action"#,
        name: r#"Carnival"#,
        deck: Some(
            r#"Carnival is an arcade game released in 1980 by Sega. It was ported to various consoles of the day."#,
        ),
        platforms: &[
            r#"Atari 2600"#,
            r#"ColecoVision"#,
            r#"Intellivision"#,
            r#"Arcade"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/0/5768/812031-carnival.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/0/5768/812031-carnival.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/0/5768/812031-carnival.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/0/5768/812031-carnival.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/0/5768/812031-carnival.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/0/5768/812031-carnival.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/0/5768/812031-carnival.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/0/5768/812031-carnival.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/0/5768/812031-carnival.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/carnival/3030-197/"#,
    },
    TimelineGame {
        id: 102,
        year: 1980,
        genre: r#"Action"#,
        name: r#"Jabbertalky"#,
        deck: None,
        platforms: &[r#"Apple II"#, r#"TRS-80"#, r#"PC"#],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/11/110673/3026329-gb_default-16_9.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/11/110673/3026329-gb_default-16_9.png"#,
        },
        site_url: r#"https://www.giantbomb.com/jabbertalky/3030-102/"#,
    },
    TimelineGame {
        id: 2,
        year: 1995,
        genre: r#"Action"#,
        name: r#"Breakfree"#,
        deck: Some(
            r#"Breakfree is a block-breaking game that is similar to Breakout; however, it is played from a first-person perspective."#,
        ),
        platforms: &[r#"PC"#],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/0/1940/621001-1056454795_00.gif"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/0/1940/621001-1056454795_00.gif"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/0/1940/621001-1056454795_00.gif"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/0/1940/621001-1056454795_00.gif"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/0/1940/621001-1056454795_00.gif"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/0/1940/621001-1056454795_00.gif"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/0/1940/621001-1056454795_00.gif"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/0/1940/621001-1056454795_00.gif"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/0/1940/621001-1056454795_00.gif"#,
        },
        site_url: r#"https://www.giantbomb.com/breakfree/3030-2/"#,
    },
    TimelineGame {
        id: 20,
        year: 1993,
        genre: r#"Action"#,
        name: r#"Burntime"#,
        deck: Some(
            r#"Burntime is a strategy/role-playing game played from a top-down perspective and set in a post-apocalyptic desert world."#,
        ),
        platforms: &[r#"Amiga"#, r#"PC"#],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/0/9408/723295-23343_boxshot_1.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/0/9408/723295-23343_boxshot_1.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/burntime/3030-20/"#,
    },
    TimelineGame {
        id: 11,
        year: 1983,
        genre: r#"Action"#,
        name: r#"Gothmog's Lair"#,
        deck: Some(r#"Text adventure game for the Commodore 64"#),
        platforms: &[r#"Commodore 64"#],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/0/238/707629-gothmog_s_lair.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/0/238/707629-gothmog_s_lair.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/gothmogs-lair/3030-11/"#,
    },
    TimelineGame {
        id: 54,
        year: 1991,
        genre: r#"Action"#,
        name: r#"Star Wars"#,
        deck: Some(
            r#"This game, based on Star Wars Episode IV: A New Hope, features both platforming and shooting segments."#,
        ),
        platforms: &[
            r#"Game Boy"#,
            r#"Game Gear"#,
            r#"Sega Master System"#,
            r#"Nintendo Entertainment System"#,
        ],
        developer: None,
        image_urls: ImageUrls {
            icon: Some(
                r#"https://www.giantbomb.com/a/uploads/square_avatar/9/93770/2362258-nes_starwars.jpg"#,
            ),
            medium: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_medium/9/93770/2362258-nes_starwars.jpg"#,
            ),
            screen: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_medium/9/93770/2362258-nes_starwars.jpg"#,
            ),
            screen_large: Some(
                r#"https://www.giantbomb.com/a/uploads/screen_kubrick/9/93770/2362258-nes_starwars.jpg"#,
            ),
            small: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_small/9/93770/2362258-nes_starwars.jpg"#,
            ),
            super_url: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_large/9/93770/2362258-nes_starwars.jpg"#,
            ),
            thumb: Some(
                r#"https://www.giantbomb.com/a/uploads/scale_avatar/9/93770/2362258-nes_starwars.jpg"#,
            ),
            tiny: Some(
                r#"https://www.giantbomb.com/a/uploads/square_mini/9/93770/2362258-nes_starwars.jpg"#,
            ),
            original: r#"https://www.giantbomb.com/a/uploads/original/9/93770/2362258-nes_starwars.jpg"#,
        },
        site_url: r#"https://www.giantbomb.com/star-wars/3030-54/"#,
    },
];

/// Get games for a specific year
pub fn games_by_year(year: i32) -> Vec<&'static TimelineGame> {
    TIMELINE_GAMES
        .iter()
        .filter(|game| game.year == year)
        .collect()
}

/// Get games for a specific genre across all years
pub fn games_by_genre(genre: &str) -> Vec<&'static TimelineGame> {
    let genre_lower = genre.to_lowercase();
    TIMELINE_GAMES
        .iter()
        .filter(|game| game.genre.to_lowercase() == genre_lower)
        .collect()
}

/// Get all unique genres in the timeline
pub fn all_genres() -> Vec<String> {
    let mut genres: Vec<String> = TIMELINE_GAMES
        .iter()
        .map(|game| game.genre.to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    genres.sort();
    genres
}

/// Get all years that have games
pub fn timeline_years() -> Vec<i32> {
    let mut years: Vec<i32> = TIMELINE_GAMES
        .iter()
        .map(|game| game.year)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    years.sort();
    years
}

/// Build a year-to-games index for efficient timeline navigation
pub fn build_timeline_index() -> HashMap<i32, Vec<&'static TimelineGame>> {
    let mut index: HashMap<i32, Vec<&'static TimelineGame>> = HashMap::new();

    for game in TIMELINE_GAMES.iter() {
        index.entry(game.year).or_default().push(game);
    }

    index
}

/// Get a random exemplar game (useful for inspiration)
pub fn random_exemplar() -> &'static TimelineGame {
    // Use a simple deterministic "random" based on current time
    let index = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize)
        % TIMELINE_GAMES.len();

    &TIMELINE_GAMES[index]
}

/// Find games that match a search query
pub fn search_games(query: &str) -> Vec<&'static TimelineGame> {
    let query_lower = query.to_lowercase();
    TIMELINE_GAMES
        .iter()
        .filter(|game| {
            game.name.to_lowercase().contains(&query_lower)
                || game.genre.to_lowercase().contains(&query_lower)
                || game
                    .deck
                    .as_ref()
                    .is_some_and(|d| d.to_lowercase().contains(&query_lower))
        })
        .collect()
}

/// Group games by their primary platform
pub fn games_by_platform() -> HashMap<String, Vec<&'static TimelineGame>> {
    let mut platform_games: HashMap<String, Vec<&'static TimelineGame>> = HashMap::new();

    for game in TIMELINE_GAMES.iter() {
        if let Some(platform) = game.platforms.first() {
            platform_games
                .entry(platform.to_string())
                .or_default()
                .push(game);
        }
    }

    platform_games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_timeline_integrity() {
        assert!(!TIMELINE_GAMES.is_empty());
        assert!(
            TIMELINE_GAMES
                .iter()
                .all(|g| g.year >= crate::vintage_games::TIMELINE_START
                    && g.year <= crate::vintage_games::TIMELINE_END)
        );
    }

    #[test]
    fn test_year_filtering() {
        let years = timeline_years();
        assert!(!years.is_empty());
        assert!(years.windows(2).all(|w| w[0] < w[1])); // Sorted
    }
}
