use anyhow::*;
use core::result::Result::Ok;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use tracing::*;

use crate::domain::{
    episode::model::Episode, media_actor::model::MediaActor as MediaActor, season::model::Season,
    tv_show::model::TvShow,
};

pub fn parse_tv_serie(nfo_path_str: &String) -> Result<TvShow> {
    let mut reader = Reader::from_file(nfo_path_str)?;
    reader.config_mut().trim_text(true);

    let mut tv_serie = TvShow::default();

    let mut element_stack = Vec::new();
    let mut curr_elem: Option<String> = None;
    let mut is_in_actor = false;

    let mut buf = Vec::new();

    let mut actor = MediaActor::default();
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
                    actor = MediaActor::default();
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
