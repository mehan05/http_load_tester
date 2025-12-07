<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>Rust HTTP Load Tester</title>
</head>
<body>
  <h1>Rust HTTP Load Tester</h1>

  <p>
    A small HTTP load testing CLI built in Rust using <code>tokio</code>,
    <code>reqwest</code>, <code>clap</code>, and <code>indicatif</code>.
  </p>

  <h2>Requirements</h2>
  <ul>
    <li>Rust and Cargo installed</li>
    <li>Internet access or a local HTTP server to test against</li>
  </ul>

  

  <h2>CLI Options</h2>
  <p>The tool accepts the following arguments:</p>
  <ul>
    <li><code>--url &lt;URL&gt;</code> (required)</li>
    <li><code>--method &lt;METHOD&gt;</code> (GET, POST, PUT, PATCH, DELETE; default: GET)</li>
    <li><code>--concurrency &lt;N&gt;</code> (number of workers; default: 10)</li>
    <li><code>--duration &lt;DURATION&gt;</code> (e.g. <code>10s</code>, <code>1m</code>)</li>
    <li><code>--data &lt;BODY&gt;</code> (optional; mainly for POST/PUT)</li>
    <li><code>--header &lt;KEY: VALUE&gt;</code> (can be repeated)</li>
    <li><code>--timeout &lt;DURATION&gt;</code> (per-request timeout; default: 5s)</li>
  </ul>

  <h2>Run (using cargo)</h2>
  <p>Example: simple GET test against httpbin for 10 seconds with 5 workers:</p>
  <pre><code>cargo run -- \
  --url https://httpbin.org/get \
  --method GET \
  --concurrency 5 \
  --duration 10s</code></pre>



  <h2>Examples</h2>

  <h3>GET with custom headers</h3>
  <pre><code>./target/release/http_load_tester \
  --url https://httpbin.org/get \
  --method GET \
  --concurrency 5 \
  --duration 5s \
  --header "User-Agent: RustLoadTester" \
  --header "X-Test: Example"</code></pre>


  <h2>Output</h2>
  <p>At the end of a run, the tool prints aggregate metrics similar to:</p>
  <pre><code>Metrics {
  total_requests: &lt;u64&gt;,
  total_errors: &lt;u64&gt;,
  RPS: &lt;f64&gt;,
  error_rate: &lt;f64&gt;,
  min_latency: &lt;Duration&gt;,
  max_latency: &lt;Duration&gt;,
  p95_latency: &lt;Duration&gt;
}</code></pre>

</body>
</html>
