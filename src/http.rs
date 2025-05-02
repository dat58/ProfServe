use crate::{
    error::{Error, Result},
    profile::Profiling,
};
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::{sync::Mutex, time::Duration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OutputFormat {
    Text,
    Flamegraph,
}

#[derive(Debug, Deserialize)]
struct ProfileQuery {
    pub seconds: Option<u64>,
    pub frequency: Option<i32>,
    pub output: Option<OutputFormat>,
}

const DEFAULT_DURATION: Duration = Duration::from_secs(10);
const DEFAULT_FREQUENCY: i32 = 100;
const DEFAULT_OUTPUT: OutputFormat = OutputFormat::Flamegraph;

#[get("/prof/cpu")]
pub async fn cpu_profile(
    profiler: web::Data<Mutex<Profiling>>,
    query: web::Query<ProfileQuery>,
) -> Result<HttpResponse> {
    let query = query.into_inner();
    let duration = query
        .seconds
        .and_then(|v| Some(Duration::from_secs(v)))
        .unwrap_or(DEFAULT_DURATION);
    let frequency = query.frequency.unwrap_or(DEFAULT_FREQUENCY);
    let output = query.output.unwrap_or(DEFAULT_OUTPUT);
    let mut profiler = profiler.lock()?;
    *profiler = Profiling::new(duration, frequency);
    let profile = match output {
        OutputFormat::Text => {
            let text = profiler.dump_text().await?;
            Ok(HttpResponse::Ok().body(text))
        }
        OutputFormat::Flamegraph => {
            let flamegraph = profiler.dump_flamegraph().await?;
            Ok(HttpResponse::Ok().body(flamegraph))
        }
    };
    profile
}

pub fn serve(port: u16) -> Result<()> {
    actix_web::rt::System::new().block_on(async {
        let profiler = web::Data::new(Mutex::new(Profiling::new(
            DEFAULT_DURATION,
            DEFAULT_FREQUENCY,
        )));
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(profiler.clone())
                .service(cpu_profile)
        })
        .bind(format!("0.0.0.0:{port}"))
        .map_err(|_| Error::General(format!("Port {port} already in use.")))?
        .run()
        .await
        .map_err(|e| Error::General(e.to_string()))?;
        Ok(())
    })
}
