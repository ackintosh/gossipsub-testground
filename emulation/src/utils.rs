use libp2p::futures::{Stream, StreamExt};
use prometheus_client::encoding::proto::openmetrics_data_model::gauge_value;
use prometheus_client::encoding::proto::openmetrics_data_model::metric_point;
use prometheus_client::encoding::proto::openmetrics_data_model::Label;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Debug;
use testground::client::Client;

// States for `barrier()`
pub(crate) const BARRIER_STARTED_LIBP2P: &str = "Started libp2p";
pub(crate) const BARRIER_DIALED: &str = "Dialed";
pub(crate) const BARRIER_DONE: &str = "Done";

// Publish info and collect it from the participants. The return value includes one published by
// myself.
pub(crate) async fn publish_and_collect<T: Serialize + DeserializeOwned>(
    client: &Client,
    info: T,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    const TOPIC: &str = "publish_and_collect";

    client.publish(TOPIC, serde_json::to_string(&info)?).await?;

    let mut stream = client.subscribe(TOPIC).await;

    let mut vec: Vec<T> = vec![];

    for _ in 0..client.run_parameters().test_instance_count {
        match stream.next().await {
            Some(Ok(other)) => {
                let info: T = serde_json::from_str(&other)?;
                vec.push(info);
            }
            Some(Err(e)) => return Err(Box::new(e)),
            None => unreachable!(),
        }
    }

    Ok(vec)
}

// Sets a barrier on the supplied state that fires when it reaches all participants.
pub(crate) async fn barrier<T: StreamExt + Unpin + libp2p::futures::stream::FusedStream>(
    client: &Client,
    swarm: &mut T,
    state: impl Into<Cow<'static, str>> + Copy,
) where
    <T as Stream>::Item: Debug,
{
    loop {
        tokio::select! {
            _ = client.signal_and_wait(state, client.run_parameters().test_instance_count) => {
                break;
            }
            // Record the Swarm events that happen while waiting for the barrier.
            event = swarm.select_next_some() => {
                client.record_message(format!("{:?}", event));
            }
        }
    }
}

pub(crate) fn get_gauge_value(value: metric_point::Value) -> (Option<i64>, Option<f64>) {
    match value {
        metric_point::Value::GaugeValue(gauge_value) => match gauge_value.value {
            Some(gauge_value::Value::IntValue(i)) => (Some(i), None),
            Some(gauge_value::Value::DoubleValue(f)) => (None, Some(f)),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub(crate) fn get_topic_hash(labels: &[Label]) -> String {
    labels
        .iter()
        .find(|l| l.name == "hash")
        .expect("have topic hash")
        .value
        .clone()
}
