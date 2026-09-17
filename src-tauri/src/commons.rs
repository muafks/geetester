use std::{collections::HashMap, sync::{Mutex, Arc}};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tokio::sync::oneshot;

use crate::link_engine::LinkEngine;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Source {
    pub s_id: String, // source id
    pub s_type: String, // vector, raster, raster-dem, geojson, image, video
    pub data: Option<String>, // link for geojson sources
    pub url: Option<String>, // for tileJSON servers
    // for xyz templates apparently
    pub tiles: Option<Vec<String>>, 
    pub tile_size: Option<u64>,
    pub min_zoom: Option<u64>,
    pub max_zoom: Option<u64>
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Layer {
    pub l_id: String,
    pub l_type: String,
    pub s_id: String,
    pub s_layer: Option<String>, // for vector types dont impl initially
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerSourceCollection {
    pub source: Source,
    pub layer: Layer
}

pub struct AppState {     
    pub lsc: Arc<Mutex<HashMap<String, LayerSourceCollection>>>,
    pub d_lsc: Option<HashMap<String, LayerSourceCollection>>,
    pub l_engine: Arc<tokio::sync::Mutex<LinkEngine>>,
    pub sender: Mutex<Option<oneshot::Sender<String>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QuestionOptions {
    pub question: String,
    pub options: Option<Vec<String>>,
    pub q_type: String
}

pub enum EndpointTypes {
    TileJSON,
    GeoJSON,
    Template,
    Unknown
}

#[derive(Deserialize, Debug, Clone)]
pub struct VectorLayer {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct TileJSON {
    pub vector_layers: Vec<VectorLayer>,
}

pub static ENDPOINT_TYPE_OPTIONS: [&str; 3] = ["TileJSON", "GeoJSON", "Template"];
pub static SOURCE_TYPE_OPTIONS: [&str; 4] = [ "vector", "raster", "raster-dem", "geojson" ];
pub static LAYER_TYPE_OPTIONS: [&str; 8] = ["fill", "line", "circle", "heatmap", "fill-extrusion", "raster", "hillshade", "color-relief"];