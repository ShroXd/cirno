use anyhow::*;
use core::result::Result::Ok;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::*;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TVSerie {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub show_title: Option<String>,
    pub sort_title: Option<String>,
    pub year: Option<String>,
    pub plot: Option<String>,
    pub genres: Vec<String>,
    pub country: Option<String>,
    pub actors: Vec<Actor>,
    pub tmdb_id: Option<String>,
    pub imdb_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub tvdb_id: Option<String>,

    // Information from the folder scanner
    pub nfo_path: Option<String>,
    pub poster_path: Option<String>,
    pub fanart_path: Option<String>,
    pub seasons: HashMap<u8, Season>,
}
impl Default for TVSerie {
    fn default() -> Self {
        TVSerie {
            title: None,
            original_title: None,
            show_title: None,
            sort_title: None,
            year: None,
            plot: None,
            genres: Vec::new(),
            country: None,
            actors: Vec::new(),
            tmdb_id: None,
            imdb_id: None,
            wikidata_id: None,
            tvdb_id: None,
            nfo_path: None,
            poster_path: None,
            fanart_path: None,
            seasons: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Actor {
    pub name: Option<String>,
    pub role: Option<String>,
    pub thumb: Option<String>,
    pub profile: Option<String>,
    pub tmdb_id: Option<String>,
}
impl Default for Actor {
    fn default() -> Self {
        Actor {
            name: None,
            role: None,
            thumb: None,
            profile: None,
            tmdb_id: None,
        }
    }
}

pub fn parse_tv_serie(nfo_path_str: &String) -> Result<TVSerie> {
    let mut reader = Reader::from_file(nfo_path_str)?;
    reader.config_mut().trim_text(true);

    let mut tv_serie = TVSerie::default();

    let mut element_stack = Vec::new();
    let mut curr_elem: Option<String> = None;
    let mut is_in_actor = false;

    let mut buf = Vec::new();

    let mut actor = Actor::default();
    let mut type_attr = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let elem_name = String::from_utf8(e.name().as_ref().to_vec())?;

                if elem_name == "actor" {
                    is_in_actor = true;
                }

                if elem_name == "uniqueid" {
                    for attr in e.attributes() {
                        match attr {
                            Ok(attr) => {
                                if attr.key == QName(b"type") {
                                    type_attr = Some(attr.unescape_value().unwrap().to_string());
                                }
                            }
                            Err(e) => {
                                error!("Error reading attribute: {}", e);
                            }
                        }
                    }
                }

                element_stack.push(elem_name.clone());
                curr_elem = Some(elem_name);
                // TODO: we may need to create some nested data structures here
            }
            Event::Text(e) => {
                if let Some(ref elem_name) = curr_elem {
                    let text = e.unescape().unwrap().to_string();
                    match elem_name.as_ref() {
                        "title" => set_field(&mut tv_serie.title, &text, "title"),
                        "originaltitle" => {
                            set_field(&mut tv_serie.original_title, &text, "originaltitle")
                        }
                        "showtitle" => set_field(&mut tv_serie.show_title, &text, "showtitle"),
                        "sorttitle" => set_field(&mut tv_serie.sort_title, &text, "sorttitle"),
                        "year" => set_field(&mut tv_serie.year, &text, "year"),
                        "plot" => set_field(&mut tv_serie.plot, &text, "plot"),
                        "country" => set_field(&mut tv_serie.country, &text, "country"),
                        "uniqueid" => match type_attr.as_deref() {
                            Some("tmdb") => set_field(&mut tv_serie.tmdb_id, &text, "tmdb id"),
                            Some("imdb") => set_field(&mut tv_serie.imdb_id, &text, "imdb id"),
                            Some("wikidata") => {
                                set_field(&mut tv_serie.wikidata_id, &text, "wikidata id")
                            }
                            Some("tvdb") => set_field(&mut tv_serie.tvdb_id, &text, "tvdb id"),
                            _ => {}
                        },
                        "genre" => {
                            debug!("genre: {}", text);
                            tv_serie.genres.push(text);
                        }
                        "name" | "role" | "thumb" | "profile" | "tmdbid" => {
                            if !is_in_actor {
                                continue;
                            }

                            match elem_name.as_ref() {
                                "name" => set_field(&mut actor.name, &text, "actor name"),
                                "role" => set_field(&mut actor.role, &text, "actor role"),
                                "thumb" => set_field(&mut actor.thumb, &text, "actor thumb"),
                                "profile" => set_field(&mut actor.profile, &text, "actor profile"),
                                "tmdbid" => set_field(&mut actor.tmdb_id, &text, "actor tmdb id"),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::End(e) => {
                let elem_name = String::from_utf8(e.name().as_ref().to_vec())?;
                if elem_name == "actor" {
                    debug!("Add new actor: {:?}", actor);
                    is_in_actor = false;
                    tv_serie.actors.push(actor.clone());
                    actor = Actor::default();
                }

                element_stack.pop();
                curr_elem = element_stack.last().cloned();
            }
            Event::Eof => break,
            _ => {}
        }
    }

    info!("tv_serie: {:?}", tv_serie);
    Ok(tv_serie)
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Season {
    pub title: Option<String>,
    pub show_title: Option<String>,
    pub sort_title: Option<String>,
    pub year: Option<String>,
    pub plot: Option<String>,
    pub tvdb_id: Option<String>,
    pub imdb_id: Option<String>,
    pub tmdb_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub premiered: Option<String>,

    // Information from the folder scanner
    pub season_number: Option<u8>,
    pub description: Option<String>,
    pub nfo_path: Option<String>,
    pub episodes: HashMap<u8, Episode>,
}
impl Default for Season {
    fn default() -> Self {
        Season {
            title: None,
            show_title: None,
            sort_title: None,
            year: None,
            plot: None,
            tvdb_id: None,
            imdb_id: None,
            tmdb_id: None,
            wikidata_id: None,
            premiered: None,
            season_number: None,
            description: None,
            nfo_path: None,
            episodes: HashMap::new(),
        }
    }
}
pub fn parse_season(nfo_path_str: &String) -> Result<Season> {
    let mut reader = Reader::from_file(nfo_path_str)?;
    reader.config_mut().trim_text(true);

    let mut seasons = Season::default();

    let mut element_stack = Vec::new();
    let mut curr_elem: Option<String> = None;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let elem_name = String::from_utf8(e.name().as_ref().to_vec())?;
                element_stack.push(elem_name.clone());
                curr_elem = Some(elem_name);
            }
            Event::Text(e) => {
                if let Some(ref elem_name) = curr_elem {
                    let text = e.unescape().unwrap().to_string();
                    match elem_name.as_ref() {
                        "title" => set_field(&mut seasons.title, &text, "title"),
                        "showtitle" => set_field(&mut seasons.show_title, &text, "showtitle"),
                        "sorttitle" => set_field(&mut seasons.sort_title, &text, "sorttitle"),
                        "year" => set_field(&mut seasons.year, &text, "year"),
                        "plot" => set_field(&mut seasons.plot, &text, "plot"),
                        "tvdbid" => set_field(&mut seasons.tvdb_id, &text, "tvdb id"),
                        "imdbid" => set_field(&mut seasons.imdb_id, &text, "imdb id"),
                        "tmdbid" => set_field(&mut seasons.tmdb_id, &text, "tmdb id"),
                        "wikidataid" => set_field(&mut seasons.wikidata_id, &text, "wikidata id"),
                        "premiered" => set_field(&mut seasons.premiered, &text, "premiered"),
                        _ => {}
                    }
                }
            }
            Event::End(e) => {
                element_stack.pop();
                curr_elem = element_stack.last().cloned();
            }
            Event::Eof => break,
            _ => {}
        }
    }

    info!("seasons: {:?}", seasons);
    Ok(seasons)
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
// TODO: maybe add file info fields here, we can get these info from the nfo file.
pub struct Episode {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub plot: Option<String>,
    pub nfo_path: Option<String>,
    pub video_file_path: String,
    pub subtitle_file_path: Option<String>,
    pub thumb_image_url: Option<String>,
    pub thumb_image: Option<String>,
    pub episode_number: Option<String>,
    pub runtime: Option<String>,
}
impl Default for Episode {
    fn default() -> Self {
        Episode {
            title: None,
            original_title: None,
            plot: None,
            nfo_path: None,
            video_file_path: "".to_string(),
            subtitle_file_path: None,
            thumb_image_url: None,
            thumb_image: None,
            episode_number: None,
            runtime: None,
        }
    }
}
impl Episode {
    pub fn merge(&mut self, other: Episode) {
        if let Some(title) = other.title {
            self.title = Some(title);
        }
        if let Some(original_title) = other.original_title {
            self.original_title = Some(original_title);
        }
        if let Some(plot) = other.plot {
            self.plot = Some(plot);
        }
        if let Some(nfo_path) = other.nfo_path {
            self.nfo_path = Some(nfo_path);
        }
        if !other.video_file_path.is_empty() {
            self.video_file_path = other.video_file_path;
        }
        if let Some(subtitle_file_path) = other.subtitle_file_path {
            self.subtitle_file_path = Some(subtitle_file_path);
        }
        if let Some(thumb_image_url) = other.thumb_image_url {
            self.thumb_image_url = Some(thumb_image_url);
        }
        if let Some(thumb_image) = other.thumb_image {
            self.thumb_image = Some(thumb_image);
        }
        if let Some(episode_number) = other.episode_number {
            self.episode_number = Some(episode_number);
        }
        if let Some(runtime) = other.runtime {
            self.runtime = Some(runtime);
        }
    }
}

pub fn parse_episode(nfo_path_str: &String) -> Result<Episode> {
    let mut reader = Reader::from_file(nfo_path_str)?;
    reader.config_mut().trim_text(true);

    let mut episode = Episode::default();

    let mut element_stack = Vec::new();
    let mut curr_elem: Option<String> = None;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let elem_name = String::from_utf8(e.name().as_ref().to_vec())?;
                element_stack.push(elem_name.clone());
                curr_elem = Some(elem_name);
            }
            Event::Text(e) => {
                if let Some(ref elem_name) = curr_elem {
                    let text = e.unescape().unwrap().to_string();
                    match elem_name.as_ref() {
                        "title" => set_field(&mut episode.title, &text, "title"),
                        "originaltitle" => {
                            set_field(&mut episode.original_title, &text, "originaltitle")
                        }
                        "episode" => {
                            set_field(&mut episode.episode_number, &text, "episode number")
                        }
                        "plot" => {
                            set_field(&mut episode.plot, &text, "plot");
                        }
                        "thumb" => {
                            set_field(&mut episode.thumb_image_url, &text, "thumb image url")
                        }
                        "runtime" => set_field(&mut episode.runtime, &text, "runtime"),
                        _ => {}
                    }
                }
            }
            Event::End(e) => {
                element_stack.pop();
                curr_elem = element_stack.last().cloned();
            }
            Event::Eof => break,
            _ => {}
        }
    }

    info!("episode: {:?}", episode);
    Ok(episode)
}

fn set_field(field: &mut Option<String>, value: &str, field_name: &str) {
    *field = Some(value.to_string());
    debug!("{}: {}", field_name, value);
}
