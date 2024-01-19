# Normal Entries

I'm attempting to push general processing off of the GUI thread. The suggested
approach is to use web workers. Reading background for this includes:
https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers
In that they describe how it works in the browser. In a javascript block they
first create a `Worker` which references a script.

```js
if (window.Worker) {
    const myWorker = new Worker("worker.js")
    ...
}
```

The `myWorker` is then a handle to a separate script that I guess is running in
parallel once it is registered. The gui thread can then use the `myWorker` as a handle
to post a message to that script.

```js
  [first, second].forEach(input => {
    input.onchange = function() {
      myWorker.postMessage([first.value, second.value]);
      console.log('Message posted to worker');
    }
  })
```

Presumably later the script will finish its work and message the main GUI
thread, which can be picked up via:

```js
  myWorker.onmessage = function(e) {
    result.textContent = e.data;
    console.log('Message received from worker');
  }
```

In the example provided the `worker.js` script has this:

```js
onmessage = function(e) {
  console.log('Worker: Message received from main script');
  const result = e.data[0] * e.data[1];
  if (isNaN(result)) {
    postMessage('Please write two numbers');
  } else {
    const workerResult = 'Result: ' + result;
    console.log('Worker: Posting message back to main script');
    postMessage(workerResult);
  }
}
```

So that script is just waiting for a message, doing the work, and posting a
message with the result if good. In the rust we use `gloo-worker` to do this
processing. I found what I think is an example in an
[anagram_solver](https://github.com/Innominus/anagram_solver). The whole point
of this repo is to be a starter template to show tailwindsss and the trunk tool.
Since trunk is all about running the client without a server I think his end
goal is csr only whereas my setup is ssr. I'm not sure the implications.

I'll try to model of this example. It uses the `gloo-worker` oneshot feature
which I'm not sure is appropriate but I'll try to follow the pattern. Here is what
he does:

1. Create a `[oneshot]` function `GetFromDictionaryWorker` that does the actual
   work. In my case it is `NormalEntries` which does the work of creating a
   thousand random entries (`Vec<HistogramEntry>`).
2. From some component (`HomeContext` in his case) spawn the script as a worker.
   ```rust
       let mut bridge = GetFromDictionaryWorker::spawner().spawn("/worker.js");
   ```
3. WebWorkers run a script. I think here he is telling it the script to run and
   sure enough he has a file called `worker.rs` in `src/bin`. So following suit,
   I create the file `src/bin/normal_entries.rs`. All that script appears to do
   is register the worker:

   ```rust
   fn main() {
        console_error_panic_hook::set_once();
        GetFromDictionaryWorker::registrar().register();
    }
   ```

   I follow suit in mine:
   ```rust
   fn main() {
        console_error_panic_hook::set_once();
        NormalEntries::registrar().register();
    }
   ```

I can successfully build the project. When I run with `cargo leptos watch` I get this error:

```js
GET
http://127.0.0.1:3004/normal_entries.js

	
GET
	http://127.0.0.1:3004/normal_entries.js
Status
404
Not Found
VersionHTTP/1.1
Transferred131 B (0 B size)
Referrer Policystrict-origin-when-cross-origin
DNS ResolutionSystem

    	
    content-type
    	text/html; charset=utf-8
    date
    	Fri, 19 Jan 2024 19:15:32 GMT
    transfer-encoding
    	chunked
    	
    Accept
    	*/*
    Accept-Encoding
    	gzip, deflate, br
    Accept-Language
    	en-US,en;q=0.5
    Connection
    	keep-alive
    Host
    	127.0.0.1:3004
    Referer
    	http://127.0.0.1:3004/
    Sec-Fetch-Dest
    	script
    Sec-Fetch-Mode
    	no-cors
    Sec-Fetch-Site
    	same-origin
    User-Agent
    	Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0
```

