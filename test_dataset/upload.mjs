// const graphs = require("./GraphsList.json");
// const owners = require("./objowner.json");
// const parents = require("./ParentGraphsList.json");
// const titles = require("./thetitles.json");
// const fs = require("fs").promises;
// const http = require("http");
import fs from "fs";
console.log("Getting graph data from JSON files...")
const graphs = JSON.parse(fs.readFileSync("./GraphsList.json").toString());
const owners = JSON.parse(fs.readFileSync("./objowner.json").toString());
const parents = JSON.parse(fs.readFileSync("./ParentGraphsList.json").toString());
const titles = JSON.parse(fs.readFileSync("./thetitles.json").toString());
//const fetch = require("node-fetch");
import fetch from "node-fetch";


console.log("Formatting graph data...");
let reqList = [];
for (let i = 0; i < graphs.length; i++) {
  let id = graphs[i];
  let name = titles[id];
  let creator = owners[id];
  let parent_id = parents[i];
  if (typeof parent_id != "string") parent_id = "Unknown";
  if (typeof name != "string") name = "Unknown";
  if (typeof creator != "string") creator = "Unknown";
  if (typeof id != "string") id = "Unknown";
  reqList.push({
    id,
    parent_id,
    name,
    creator
  });
}

const dataset = JSON.stringify({
  data: reqList
});

console.log("Uploading graph data to database...");
(async () => {
  let result = await (await fetch("http://localhost:8000/graphs", {
    method: "POST",
    body: dataset,
    headers: { "Content-Type": "application/json" }
  }, 100000)).text();
  console.log(result);
  console.log("Done!");
})();


// http.createServer((req, res) => {
//   res.setHeader("Access-Control-Allow-Origin", "*")
//   res.end(data);
// }).listen(8001);
