const graphs = require("./GraphsList.json");
const owners = require("./objowner.json");
const parents = require("./ParentGraphsList.json");
const titles = require("./thetitles.json");
const fs = require("fs").promises;
const http = require("http");

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

const data = JSON.stringify({
  data: reqList
});

http.createServer((req, res) => {
  res.setHeader("Access-Control-Allow-Origin", "*")
  res.end(data);
}).listen(8001);
