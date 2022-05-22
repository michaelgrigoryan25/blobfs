use lazy_static::lazy_static;
use prometheus::{
    opts, register_histogram_vec, register_int_counter_vec, register_int_gauge, HistogramVec,
    IntCounterVec, IntGauge,
};

lazy_static! {
    pub(crate) static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("http_requests_total", "HTTP requests total"),
        &["method", "path"]
    )
    .expect("vxs_server::metrics::HTTP_REQUESTS_TOTAL: cannot create metric");
    pub(crate) static ref HTTP_CONNECTED_SSE_CLIENTS: IntGauge =
        register_int_gauge!(opts!("http_connected_sse_clients", "Connected SSE clients"))
            .expect("vxs_server::metrics::HTTP_CONNECTED_SSE_CLIENTS: cannot create metric");
    pub(crate) static ref HTTP_RESPONSE_TIME_SECONDS: HistogramVec = register_histogram_vec!(
        "http_response_time_seconds",
        "HTTP response times",
        &["method", "path"],
    )
    .expect("vxs_server::metrics::HTTP_RESPONSE_TIME_SECONDS: cannot create metric");
}

// let mut buffer = vec![];
// let encoder = TextEncoder::new();
// encoder
//     .encode(&prometheus::gather(), &mut buffer)
//     .expect("vxs_server::metrics::handler: cannot encode metrics");
