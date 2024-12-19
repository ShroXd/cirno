use std::sync::Arc;

use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use tracing::*;

use crate::{
    infrastructure::database::{
        actor::{QueryAllMediaItems, QueryMediaItemsByMediaLibraryId},
        database::Database,
        media_item::repository::MediaRepository,
    },
    init::repository_manager::Repositories,
    interfaces::dtos::MediaItemDto,
};
