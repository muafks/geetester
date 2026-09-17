use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};

use geojson::{GeoJson, GeometryValue};
use mime_guess::{Mime, from_ext, mime};
use rand::{RngExt, distr::Alphanumeric};
use tauri::AppHandle;

use crate::{
    commons::{
        ENDPOINT_TYPE_OPTIONS, EndpointTypes, LAYER_TYPE_OPTIONS, Layer, LayerSourceCollection,
        Source, TileJSON,
    },
    prompt_webui,
};

#[derive(Clone)]
pub struct LinkEngine {
    pub collection: Arc<Mutex<HashMap<String, LayerSourceCollection>>>, // link to layer collection
    slug_pointers: HashMap<String, String>,                             // slug-id to link
}

impl LinkEngine {
    pub fn new() -> Self {
        Self {
            collection: Arc::new(Mutex::new(HashMap::new())),
            slug_pointers: HashMap::new(),
        }
    }

    pub async fn clear_all(&mut self) {
        self.collection = Arc::new(Mutex::new(HashMap::new()));
        self.slug_pointers = HashMap::new();
    }

    pub async fn add_url(&mut self, url: &str, app_h: Option<&AppHandle>) -> String {
        // let slug_id = LinkEngineFunctions::generate_petname();
        let source = LinkEngineFunctions::generate_source(url, None, app_h).await;
        let layer = LinkEngineFunctions::generate_layer(&source, app_h).await;
        let lid = layer.l_id.clone();
        let lsc = LayerSourceCollection {
            source: source,
            layer: layer,
        };

        self.populate_collection(&lid, url, lsc);

        return lid;
    }

    pub fn remove(&mut self, url_or_id: String) -> Result<(), ()> {
        let url_to_remove = if let Some(resolved_url) = self.slug_pointers.remove(&url_or_id) {
            resolved_url
        } else {
            url_or_id
        };

        if self
            .collection
            .lock()
            .unwrap()
            .remove(&url_to_remove)
            .is_some()
        {
            Ok(())
        } else {
            Err(())
        }
    }

    fn populate_collection(&mut self, slug_id: &String, url: &str, lsc: LayerSourceCollection) {
        self.collection.lock().unwrap().insert(url.to_string(), lsc);
        self.slug_pointers.insert(slug_id.clone(), url.to_string());
    }

    pub fn _get_collection(&mut self, url_or_id: &str) -> Result<LayerSourceCollection, String> {
        if let Some(url) = self.slug_pointers.get(&url_or_id.to_string())
            && let Some(lsc) = self.collection.lock().unwrap().get(url)
        {
            return Ok(lsc.clone());
        }

        if let Some(lsc) = self.collection.lock().unwrap().get(&url_or_id.to_string()) {
            return Ok(lsc.clone());
        }

        Err(String::new())
    }
}

struct LinkEngineFunctions;
impl LinkEngineFunctions {
    async fn infer_endpoint_type(
        url: &str,
    ) -> Result<EndpointTypes, Box<dyn std::error::Error + Send + Sync>> {
        if ["{x}", "{y}", "{z}"].iter().any(|&item| url.contains(item)) {
            return Ok(EndpointTypes::Template);
        }

        let body = reqwest::get(url).await?.text().await?;
        let response: serde_json::Value = serde_json::from_str(&body)?;

        if response.get("tilejson").is_some() || response.get("tiles").is_some() {
            return Ok(EndpointTypes::TileJSON);
        }

        let is_geojson_type = response
            .get("type")
            .and_then(|t| t.as_str())
            .map_or(false, |t| t == "FeatureCollection" || t == "Feature");

        if is_geojson_type || response.get("features").is_some() {
            return Ok(EndpointTypes::GeoJSON);
        }

        Ok(EndpointTypes::Unknown)
    }

    fn generate_petname() -> String {
        petname::petname(2, "-").unwrap_or_else(|| {
            rand::rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(|c| (c as char).to_ascii_lowercase())
                .collect()
        })
    }

    async fn infer_s_type_from_extension(ext: &str, url: &str) -> &'static str {
        if let Some(mime) = from_ext(ext).first()
            && { mime.type_().as_str() == "image" && mime.subtype().as_str() != "svg+xml" }
        {
            return "raster";
        }

        let test_url = url
            .replace("{z}", "1")
            .replace("{x}", "1")
            .replace("{y}", "0");

        let client = reqwest::Client::new();

        let mut response = match client.head(&test_url).send().await {
            Ok(r) => r,
            Err(_) => {
                return "vector";
            }
        };

        if !response.status().is_success()
            || response.status() == reqwest::StatusCode::METHOD_NOT_ALLOWED
        {
            response = match client.get(&test_url).send().await {
                Ok(r) => r,
                Err(_) => {
                    return "vector";
                }
            };
        }

        if !response.status().is_success() {
            return "vector";
        }

        if let Some(content_type_header) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type_header.to_str() {
                if let Ok(mime_type) = Mime::from_str(content_type_str) {
                    if mime_type.type_() == mime::IMAGE {
                        return "raster";
                    }
                }
            }
        }

        return "vector";
    }

    async fn extract_first_url_from_tilejson_tiles(
        url: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let body = reqwest::get(url).await?.text().await?;
        let response: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(tiles) = response.get("tiles") {
            return Ok(tiles
                .as_array()
                .ok_or("Not list")?
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .next()
                .ok_or("Failed to get first")?);
        } else {
            return Err("".into());
        }
    }

    async fn handle_tilejson_s_type(url: &str) -> &'static str {
        if let Ok(e_url) = LinkEngineFunctions::extract_first_url_from_tilejson_tiles(url).await
            && let Some(ext) = e_url.rsplit('.').next()
        {
            return LinkEngineFunctions::infer_s_type_from_extension(ext, url).await;
        } else {
            return "raster";
        }
    }

    async fn split_url_and_infer_s_type(url: &str) -> &'static str {
        if let Some(ext) = url.rsplit('.').next()
            && LinkEngineFunctions::infer_s_type_from_extension(ext, url).await == "vector"
        {
            println!("{} : {}", url, ext);
            return "vector";
        }

        //effectively a fallback value
        return "raster";
    }

    // async fn query_user_for_source(url: &str) -> Source {
    //     println!("Unable to automatically infer source type, please answer the following questions.");

    //     Source
    // }

    async fn generate_source(
        url: &str,
        force_type: Option<EndpointTypes>,
        app_h: Option<&AppHandle>,
    ) -> Source {
        let endpoint_type = if let Some(e_type) = force_type {
            e_type
        } else {
            LinkEngineFunctions::infer_endpoint_type(url)
                .await
                .unwrap_or(EndpointTypes::Unknown)
        };
        match endpoint_type {
            EndpointTypes::GeoJSON => {
                return Source {
                    s_id: LinkEngineFunctions::generate_petname(),
                    s_type: "geojson".to_string(),
                    data: Some(url.to_string()),
                    url: None,
                    tiles: None,
                    tile_size: None,
                    min_zoom: None,
                    max_zoom: None,
                };
            }
            EndpointTypes::TileJSON => {
                return Source {
                    s_id: LinkEngineFunctions::generate_petname(),
                    s_type: LinkEngineFunctions::handle_tilejson_s_type(url)
                        .await
                        .to_string(),
                    data: None,
                    url: Some(url.to_string()),
                    tiles: None,
                    tile_size: None,
                    min_zoom: None,
                    max_zoom: None,
                };
            }
            EndpointTypes::Template => {
                return Source {
                    s_id: LinkEngineFunctions::generate_petname(),
                    s_type: LinkEngineFunctions::split_url_and_infer_s_type(url)
                        .await
                        .to_string(),
                    data: None,
                    url: None,
                    tiles: Some(vec![url.to_string()]),
                    tile_size: Some(256),
                    min_zoom: Some(0),
                    max_zoom: Some(21),
                };
            }
            EndpointTypes::Unknown => {
                if let Some(app_handle) = app_h {
                    let res = prompt_webui(
                        app_handle,
                        "Select Source Type: ",
                        Some(ENDPOINT_TYPE_OPTIONS.to_vec()),
                    )
                    .await
                    .unwrap_or("".to_string());
                    let selected_type = match res.as_str() {
                        "TileJSON" => EndpointTypes::TileJSON,
                        "GeoJSON" => EndpointTypes::GeoJSON,
                        "Template" => EndpointTypes::Template,
                        _ => EndpointTypes::Unknown,
                    };
                    return Box::pin(LinkEngineFunctions::generate_source(
                        url,
                        Some(selected_type),
                        app_h,
                    ))
                    .await;
                } else {
                    println!("Unable to infer source type, sorry.");
                    let selected_type = match inquire::Select::new(
                        "Select Source Type",
                        ENDPOINT_TYPE_OPTIONS.to_vec(),
                    )
                    .prompt()
                    .unwrap_or("")
                    {
                        "TileJSON" => EndpointTypes::TileJSON,
                        "GeoJSON" => EndpointTypes::GeoJSON,
                        "Template" => EndpointTypes::Template,
                        _ => EndpointTypes::Unknown,
                    };
                    return Box::pin(LinkEngineFunctions::generate_source(
                        url,
                        Some(selected_type),
                        None,
                    ))
                    .await;
                }
            }
        }
    }
    //
    //
    // layer section
    //
    //
    async fn geojson_to_geometry_value(gj: &GeoJson) -> Option<GeometryValue> {
        match gj {
            GeoJson::Feature(feature) => feature.geometry.clone().map(|g| g.value),
            GeoJson::FeatureCollection(collection) => collection
                .features
                .iter()
                .find_map(|f| f.geometry.as_ref().map(|g| g.value.clone())),
            GeoJson::Geometry(geometry) => Some(geometry.value.clone()),
        }
    }
    async fn infer_layer_type_from_source(source: &Source, app_h: Option<&AppHandle>) -> String {
        //lazy
        match source.s_type.as_str() {
            "image" | "video" => {
                return "raster".to_string();
            }
            "raster-dem" => {
                return "hillshade".to_string();
            }
            "raster" => {
                return "raster".to_string();
            }
            _ => {}
        }

        //geojson
        if let Some(url) = &source.data
            && let Ok(res) = reqwest::get(url).await
            && let Ok(body) = res.text().await
            && let Ok(gj) = body.parse::<GeoJson>()
            && let Some(geometry_value) = LinkEngineFunctions::geojson_to_geometry_value(&gj).await
        {
            let layer_type = match geometry_value {
                GeometryValue::Point { .. } | GeometryValue::MultiPoint { .. } => {
                    "circle".to_string()
                }
                GeometryValue::LineString { .. } | GeometryValue::MultiLineString { .. } => {
                    "line".to_string()
                }
                GeometryValue::Polygon { .. } | GeometryValue::MultiPolygon { .. } => {
                    "fill".to_string()
                }
                GeometryValue::GeometryCollection { .. } => "".to_string(),
            };

            if !layer_type.is_empty() {
                return layer_type;
            }
        }

        if let Some(app_handle) = app_h {
            prompt_webui(
                app_handle,
                "Select layer type",
                Some(LAYER_TYPE_OPTIONS.to_vec()),
            )
            .await
            .unwrap_or("circle".to_string())
        } else {
            inquire::Select::new("Select layer type", LAYER_TYPE_OPTIONS.to_vec())
                .prompt()
                .unwrap_or("circle")
                .to_string()
        }
    }

    async fn generate_layer(source: &Source, app_h: Option<&AppHandle>) -> Layer {
        if let Some(url) = &source.url
            && let Ok(vector_source_options) =
                LinkEngineFunctions::get_available_vector_layers_from_tilejson(&url).await
        {
            let ans = match app_h {
                Some(app_handle) => prompt_webui(
                    app_handle,
                    "Which of the 'vector' layers do you want to use?",
                    Some(vector_source_options.iter().map(|s| s.as_str()).collect()),
                )
                .await
                .unwrap_or("".to_string()),
                None => {
                    match inquire::Select::new(
                        "Which of the 'vector' layers do you want to use?",
                        vector_source_options,
                    )
                    .prompt()
                    {
                        Ok(t) => t,
                        Err(_) => "".to_string(),
                    }
                }
            };

            return Layer {
                l_id: LinkEngineFunctions::generate_petname(),
                l_type: LinkEngineFunctions::infer_layer_type_from_source(source, app_h).await,
                s_id: source.s_id.clone(),
                s_layer: Some(ans),
            };
        } else if let Some(_) = source.url {
            // - monitor if works
            let ans = match app_h {
                Some(app_handle) => prompt_webui(app_handle, "Type the intended source-layer", None).await.unwrap_or("".to_string()),
                None => {
match inquire::Text::new("The source has been detected as 'vector', but we are unable to find the available source-layer ids. \nPlease type your intended source-layer's id: ").prompt() {
                Ok(t) => t,
                Err(_) => "".to_string()
            }
                }
            };

            return Layer {
                l_id: LinkEngineFunctions::generate_petname(),
                l_type: LinkEngineFunctions::infer_layer_type_from_source(source, app_h).await,
                s_id: source.s_id.clone(),
                s_layer: Some(ans),
            };
        }

        Layer {
            l_id: LinkEngineFunctions::generate_petname(),
            l_type: LinkEngineFunctions::infer_layer_type_from_source(source, app_h).await,
            s_id: source.s_id.clone(),
            s_layer: None,
        }
    }

    async fn get_available_vector_layers_from_tilejson(
        url: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let data: TileJSON = reqwest::get(url).await?.json().await?;

        Ok(data.vector_layers.iter().map(|i| i.id.clone()).collect())
    }
}
