# desmosearch-api
API and site for discovering, cataloguing, and keeping track of publicly-available Desmos graphs. Not affiliated with Desmos. This project is currently in a very early stage of development, so expect lots of breaking changes for the time being.

## Hosting
To host desmosearch-api, follow these instructions:
1. Install [Nodejs](https://nodejs.org/en/) and [Rust](https://www.rust-lang.org/tools/install) if you have not already.
2. Clone this repository.
3. Navigate to the root directory of the repository, and run `cargo run --release`. This will run the backend (web server). The backend is the server that exposes the API that allows for the querying of Desmos graphs, and also allows users to access the frontend. This may take a while the first time, as it will have to compile the backend from scratch.
4. Navigate to `frontend/desmosearch-browser`, run `npm install`, and then run `npm run build` to compile the frontend- in other words, the website that lists the Desmos graphs. 
5. (Optional) Go to the `test_dataset` directory, run `npm install`, and then run `npm start`. This will upload about 130000 graphs to the database (which would otherwise be empty).
6. Navigate to the page `http://localhost:8000/static/index.html` to view the frontend. If you see a page of Desmos graphs appear, you're good to go!

## API Documentation
The Desmosearch API has a single endpoint at `/graphs`. This will- by default- show you a single graph. By appending a querystring to the URL (examples below), you can sort and filter the content to your liking.

### `limit` and `offset`
- The `limit` parameter tells the server how many graphs to return when querying graphs.
- The `offset` parameter tells the server how many graphs to skip before sending back graphs. This is useful if you want to navigate through multiple "pages" of content- e.g. offset could be `0` for the first page, and `10` for the second page, thus skipping the first 10 graphs.

### `id`, `parent_id`, `name`, and `creator`
- The `id` query parameter lets you filter by graph ID (the unique final path segment of a Desmos graph link)
- The `parent_id` parameter lets you filter by the ID of a previous version of a graph.
- The `name` parameter lets you filter by graph name.
- The `creator` parameter lets you filter by graph creator. (Note that, in many cases, this is unknown.)
There are two wildcards you can use with these parameters: `%` (must be escaped as `%25`), which means "0 or more of any character," and `_`, which means "one of any character".

Example: `/graphs?creator=MathEnthusiast314&limit=10&name=%25fractal%25`. This query returns exactly ten graphs which have been created by `MathEnthusiast314`, and contain the string `fractal` somewhere in their name.

### `upload_date_start` and `upload_date_end`
Filters graphs by date range- they must have been uploaded to Desmosearch (not Desmos!) after the value of `upload_date_start` and before the value of `upload_date_end`. **These parameters- if included- *must* be Unix Timestamps!**

### `sort`
Determines the means by which results are sorted. Can be one of `id`, `parent_id`, `name`, `creator`, or `upload_date`.