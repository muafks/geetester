// use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::oneshot;

use crate::commons::{AppState, LayerSourceCollection, QuestionOptions};

pub mod commons;
pub mod link_engine;

// #[tauri::command]
// async fn get_latest_collection(
//     app: AppHandle,
//     state: tauri::State<'_, AppState>,
// ) -> Result<(), ()> {
//     let lsc_arc = state.lsc.clone();

//     let mut d_lsc_snapshot = state.d_lsc.clone();

//     tauri::async_runtime::spawn(async move {
//         loop {
//             let is_equal = if let Some(ref delta) = d_lsc_snapshot {
//                 let guard = lsc_arc.lock().unwrap();
//                 *guard == *delta
//             } else {
//                 false
//             };

//             if is_equal {
//                 tokio::time::sleep(Duration::from_millis(50)).await;
//                 continue;
//             }

//             let lscs = lsc_arc
//                 .lock()
//                 .unwrap()
//                 .clone()
//                 .into_values()
//                 .collect::<Vec<LayerSourceCollection>>();

//             _ = app.emit("geo-update", lscs);
//             println!("emitted gupdate");
//             d_lsc_snapshot = Some(lsc_arc.lock().unwrap().clone());
//         }
//     });

//     Ok(())
// }

async fn send_layer_update(app: &tauri::AppHandle, state: &tauri::State<'_, AppState>) {
    let lscs = {
        let engine = state.l_engine.lock().await;
        engine.collection
            .lock()
            .unwrap()
            .clone()
            .into_values()
            .collect::<Vec<LayerSourceCollection>>()
    };
    _ = app.emit("geo-update", lscs);
}

#[tauri::command]
async fn clear_all_layers(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), ()> {
    println!("Clearing all layers");

    let mut engine = state.l_engine.lock().await;
    engine.clear_all().await;
    println!("Cleared all");

    drop(engine);

    send_layer_update(&app, &state).await;
    
    Ok(())
}


#[tauri::command]
async fn add_link(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    url: String,
) -> Result<(), String> {
    let mut engine = state.l_engine.lock().await;
    println!("adding url to lengine");
    engine.add_url(url.as_str(), Some(&app)).await;
    println!("added url to lengine");
    drop(engine);
    send_layer_update(&app, &state).await;
    Ok(())
}

#[tauri::command]
fn submit_web_input(value: String, state: tauri::State<'_, AppState>) {
    if let Some(sender) = state.sender.lock().unwrap().take() {
        let _ = sender.send(value);
    }
}

#[tauri::command]
async fn delete_a_link(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    url: String,
) -> Result<(), String> {
    let mut engine = state.l_engine.lock().await;

    if let Ok(_) = engine.remove(url) {println!("did remove");} else {println!("Failed to remove");}
    drop(engine);
    send_layer_update(&app, &state).await;

    println!("emitted gupdate");
    Ok(())
}

pub async fn prompt_webui(
    app: &AppHandle,
    message: &str,
    options: Option<Vec<&str>>,
) -> Result<String, String> {
    let state = app.state::<AppState>();

    let (tx, rx) = oneshot::channel();

    *state.sender.lock().unwrap() = Some(tx);

    app.emit(
        "rust-request-input",
        QuestionOptions {
            question: message.to_string(),
            options: if let Some(q) = options.clone() {Some(q.into_iter().map(String::from).collect::<Vec<String>>())} else {None},
            q_type: if let Some(_) = options { "choice".to_string() } else { "written".to_string() }
        },
    )
    .map_err(|e| e.to_string())?;

    let response = rx
        .await
        .map_err(|_| "Failed to receive input from webui".to_string())?;

    Ok(response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(link_to_source_map: AppState) {
    tauri::Builder::default()
        .manage(link_to_source_map)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            add_link,
            submit_web_input,
            clear_all_layers,
            delete_a_link
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
