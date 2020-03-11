use futures::future::try_join_all;
use loa::server::Server as WrappedServer;
use loa::vm::VM;
use loa::{Source, SourceKind, URI};
use std::error::Error;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::*;
use web_sys::*;

extern crate console_error_panic_hook;
extern crate js_sys;
extern crate loa;
extern crate web_sys;

extern crate futures;

#[macro_use]
extern crate serde_derive;

const CDN_URL: &str = concat!("https://cdn.loalang.xyz/", env!("CARGO_PKG_VERSION"), "/");

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum StdManifestEntry {
    File {
        name: String,
    },

    Directory {
        name: String,
        contents: Vec<StdManifestEntry>,
    },
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

/// @internal
/// @private
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Server {
    server: WrappedServer,
    vm: VM,
    cursor: loa::assembly::Cursor,
}

#[wasm_bindgen]
impl Server {
    pub async fn load() -> Result<Server, JsValue> {
        let mut server = WrappedServer::new();
        server.add_all(stdlib_sources().await?);
        let mut cursor = loa::assembly::Cursor::new();
        let std_instructions = server.generator().generate_all().unwrap().compile(&mut cursor);
        let mut vm = VM::new();
        vm.eval::<()>(std_instructions);
        Ok(Server { server, vm, cursor })
    }

    pub fn set(&mut self, uri: &str, code: &str) {
        self.server
            .set(URI::Exact(uri.into()), code.into(), SourceKind::REPLLine);
    }

    pub fn evaluate(&mut self, uri: &str) -> Result<Option<String>, JsValue> {
        let uri = URI::Exact(uri.into());
        match self.server.diagnostics().get(&uri).and_then(|d| {
            if d.len() == 0 {
                None
            } else {
                Some(d)
            }
        }) {
            Some(d) => Err(JsValue::from_serde(
                &d.iter().map(|d| d.to_string()).collect::<Vec<_>>(),
            )
            .unwrap()),
            None => {
                let mut generator = self.server.generator();
                let mut assembly = loa::assembly::Assembly::new();
                match generator.generate((), &mut assembly, &uri) {
                    Err(e) => Err(JsValue::from_str(format!("{:?}", e).as_str())),
                    Ok(_) => {
                        Ok(self.vm.eval_pop::<()>(assembly.compile(&mut self.cursor)).map(|o| o.to_string()))
                    }
                }
            }
        }
    }
}

async fn stdlib_sources() -> Result<Vec<Arc<Source>>, JsValue> {
    let mut urls = vec![];
    let prefix = format!("{}std", CDN_URL);
    for entry in load_manifest().await? {
        push_urls_from_entry(entry, prefix.clone(), &mut urls);
    }

    let mut sources = vec![];
    for url in urls {
        sources.push(load_stdlib_module_from_url(url.clone()));
    }
    try_join_all(sources).await
}

fn push_urls_from_entry(entry: StdManifestEntry, mut prefix: String, urls: &mut Vec<String>) {
    match entry {
        StdManifestEntry::File { name } => {
            prefix.push('/');
            prefix.push_str(name.as_str());
            urls.push(prefix);
        }

        StdManifestEntry::Directory { name, contents } => {
            prefix.push('/');
            prefix.push_str(name.as_str());
            for entry in contents {
                push_urls_from_entry(entry, prefix.clone(), urls);
            }
        }
    }
}

async fn load_manifest() -> Result<Vec<StdManifestEntry>, JsValue> {
    let resp = fetch(format!("{}std/manifest.json", CDN_URL).as_str()).await?;

    let entry = JsFuture::from(resp.json()?).await?.into_serde();

    Ok(entry.map_err(|e| JsValue::from_str(e.description()))?)
}

async fn fetch(url: &str) -> Result<Response, JsValue> {
    let request = Request::new_with_str(&url)?;

    let global = js_sys::global();

    let promise = if global.constructor().name() == "Window" {
        let window: web_sys::Window = global.dyn_into().unwrap();
        window.fetch_with_request(&request)
    } else {
        let self_: web_sys::WorkerGlobalScope = global.dyn_into().unwrap();
        self_.fetch_with_request(&request)
    };

    let resp_value = JsFuture::from(promise).await?;

    assert!(resp_value.is_instance_of::<Response>());
    resp_value.dyn_into()
}

async fn load_stdlib_module_from_url(url: String) -> Result<Arc<Source>, JsValue> {
    let resp = fetch(url.as_str()).await?;

    // Convert this other `Promise` into a rust `Future`.
    let code = JsFuture::from(resp.text()?).await?;

    let source = Source::new(
        SourceKind::Module,
        URI::Stdlib(url[CDN_URL.len()..].into()),
        code.as_string().unwrap(),
    );

    Ok(source)
}
