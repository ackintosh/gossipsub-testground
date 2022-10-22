# smoke

## How to run

```shell
# Import the test plans from this repo
git clone https://github.com/sigp/gossipsub-testground.git
testground plan import --from ./gossipsub-testground/

# Run `smoke` test plan
testground run single \
  --plan=gossipsub-testground/smoke \
  --testcase=smoke \
  --builder=docker:generic \
  --runner=local:docker \
  --instances=3 \
  --wait
```

## Dashboards

Please see the root [README](https://github.com/sigp/gossipsub-testground/blob/main/README.md) for how to run Grafana.

### Message Propagations

Visualization of a message propagation. 

Variables for this dashboard:

- `run_id`: ID for the test run you want to see.
- `publisher_id`: Publisher's peer ID of message you want to see.
  - In this `smoke` test plan, each gossipsub instance publishes a message once so specifing peer ID can identifies a message.

<img width="1308" alt="image" src="https://user-images.githubusercontent.com/1885716/197323564-835851f7-1036-4ba4-8574-97b201c1dba7.png">
