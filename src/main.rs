use std::{
    fs::File,
    io::Write,
    sync::atomic::{AtomicBool, Ordering},
};

use bevy::{app::AppExit, asset::LoadState, gltf::Gltf, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    name: String,
    screenshot: String,
}

struct ModelIndex(Vec<Model>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum State {
    NotLoaded,
    Loading,
    Loaded,
    Failed,
}

impl From<LoadState> for State {
    fn from(load_state: LoadState) -> Self {
        match load_state {
            LoadState::NotLoaded => State::NotLoaded,
            LoadState::Loading => State::Loading,
            LoadState::Loaded => State::Loaded,
            LoadState::Failed => State::Failed,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
struct SuiteState {
    #[serde(skip)]
    handle: Handle<Gltf>,
    states: Vec<State>,
}

static LOAD_FAILED_FLAG: AtomicBool = AtomicBool::new(false);

const OUTPUT_PATH: &'static str = "output.json";
const README_PATH: &'static str = "temp.md";

fn main() {
    //let panic_handle = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        println!("panic: {}", panic_info);
        if let Some(location) = panic_info.location() {
            if location.file().ends_with("bevy_gltf/src/loader.rs") {
                // glTF panic
                LOAD_FAILED_FLAG.store(true, Ordering::Relaxed)
            }
        }
        //(panic_handle)(panic_info);
    }));

    // loads the model index
    let file = File::open("assets/gltf_samples/2.0/model-index.json").unwrap();
    let model_index = ModelIndex(serde_json::from_reader(file).unwrap());

    // load previous output or
    let suite_state = SuiteState {
        states: File::open(OUTPUT_PATH)
            .ok()
            .and_then(|file| serde_json::from_reader(file).ok())
            .unwrap_or_default(),
        ..Default::default()
    };

    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(model_index)
        .insert_resource(suite_state)
        .add_system(test_suite.system())
        .run();
}

fn check(ok: bool) -> &'static str {
    if ok {
        ":heavy_check_mark:"
    } else {
        ":x:"
    }
}

fn test_suite(
    mut suite_state: ResMut<SuiteState>,
    mut exit: EventWriter<AppExit>,
    model_index: Res<ModelIndex>,
    asset_server: Res<AssetServer>,
) {
    // Break down
    let suite_state = &mut *suite_state;
    let states = &mut suite_state.states;
    let handle = &mut suite_state.handle;

    let mut done = false;

    let index = states.len();
    match states.last_mut() {
        Some(State::Failed) | Some(State::Loaded) | None => {
            // Load next if any
            if index < model_index.0.len() {
                let name = &model_index.0[index].name;
                let path = format!("gltf_samples/2.0/{}/glTF/{}.gltf", name, name);
                info!("loading gltf at `{}`", path,);

                // Load
                LOAD_FAILED_FLAG.store(false, Ordering::Relaxed);
                *handle = asset_server.load(path.as_str());

                states.push(State::Loading);
            } else {
                done = true;
            }
        }
        Some(state) => {
            if LOAD_FAILED_FLAG.load(Ordering::Relaxed) {
                *state = State::Failed;
                done = true; // Failed with panic
                error!(
                    "{} failed to load with panic, please restart this test suite to continue",
                    model_index.0[index - 1].name,
                );
            } else {
                *state = asset_server.get_load_state(&*handle).into();
            }
        }
    }

    if done {
        info!("done, writing result at `{}`", OUTPUT_PATH);
        serde_json::to_writer_pretty(File::create(OUTPUT_PATH).unwrap(), states).unwrap();

        let mut file = File::create(README_PATH).unwrap();
        writeln!(&mut file, "|Model|Screenshot|Load|Spawn|Glitch|").unwrap();
        writeln!(&mut file, "|-----|----------|----|-----|------|").unwrap();
        for (i, state) in states.iter().enumerate() {
            let model = &model_index.0[i];
            writeln!(
                &mut file,
                "|[{}]({})|![]({}/{})|{}| | |",
                model.name,
                model.name,
                model.name,
                model.screenshot,
                check(*state == State::Loaded)
            )
            .unwrap();
        }

        exit.send(AppExit);
    }
}
