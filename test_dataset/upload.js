/*
  DESCRIPTION:
This script will take MathEnthusiast314's JSON dataset and put it in the Rust database in a single API call.  

  USAGE:
1. Run the rust web server.
2. Run upload.js using node. Make sure it keeps running.
3. Navigate to http://localhost:8000/graphs?limit=10
4. Paste this script into the console there, and run it.
5. Reload the page. If graph JSON appears that wasn't there before, it worked.
6. You can now safely close upload.js.
*/

let dataset = await ((await fetch("http://localhost:8001/")).text());

await (await fetch("/graphs", {
    method: "POST",
    body: dataset,
    headers: { "Content-Type": "application/json" }
}, 100000)).text();