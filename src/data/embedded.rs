//! Build-time-generated manifest of embedded campaign/scenario JSON.
//!
//! `build.rs` scans `assets/campaigns/` and `assets/scenarios/` and writes
//! `EMBEDDED_CAMPAIGNS` / `EMBEDDED_SCENARIOS` (each a `&[&str]` of file
//! contents). This is the "WASM embedded manifest": on `wasm32` there is no
//! filesystem, so these arrays are the only content source. On native builds
//! `data::content_source` overlays a runtime directory scan on top.

include!(concat!(env!("OUT_DIR"), "/embedded_content.rs"));
