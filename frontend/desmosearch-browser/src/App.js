import logo from './logo.svg';
import './App.css';

import React, { useEffect, useState } from 'react';

function GraphPreview(props) {
  const graph = props.graph;
  return (
    <li className="graph-gallery-item">
      <a href={`https://www.desmos.com/calculator/${graph.id}`}><img src={`https://saved-work.desmos.com/calc_thumbs/production/${graph.id}.png`}></img></a>
      <br></br>
      <span><b>{graph.name}</b> by {graph.creator}</span>
    </li>
  )
}

function isUserCloseToBottom() {
  return window.scrollY + window.innerHeight + 100 > document.body.getBoundingClientRect().height;
}

function GraphGallery(props) {
  let [graphs, setGraphs] = useState({
    graphList: [],
    offset: 0
  });
  let [shouldEraseGraphs, setShouldEraseGraphs] = useState(false);
  let [isLoadingGraphs, setIsLoadingGraphs] = useState(false);

  let additionalQueries = "";
  Object.entries(props.queryState).forEach(([k, v]) => {
    if (v != "") {
      additionalQueries += `&${k}=${encodeURIComponent(v)}`;
    }
  });
  useEffect(() => {
    let addMoreGraphs = (async () => {
      console.log(graphs.offset);
      setIsLoadingGraphs(true);
      let dataset = await ((await fetch(`http://localhost:8000/graphs?limit=10&offset=${graphs.offset}` + additionalQueries)).json());
      let graphsCopy = {
        offset: graphs.offset + 10,
        graphList: graphs.graphList.concat(dataset.data)
      }
      setGraphs(graphsCopy);
      if (dataset.data.length != 0) {
        setIsLoadingGraphs(false);
      }
    });
    if (!isLoadingGraphs && isUserCloseToBottom()) {
      addMoreGraphs();
    } else {
      let scrollHandler = e => {
        if (!isLoadingGraphs && isUserCloseToBottom()) {
          addMoreGraphs();
        }
      };
      window.addEventListener("scroll", scrollHandler)

      return _ => {
        window.removeEventListener("scroll", scrollHandler);
      }
    }
  })

  useEffect(() => {
    if (props.shouldEraseGraphs) {
      // setGraphs({
      //   offset: 0,
      //   graphList: [],
      //   erase: true
      // });
      setShouldEraseGraphs(true);
      props.setShouldEraseGraphs(false);
    }
    if (shouldEraseGraphs) {
      setShouldEraseGraphs(false);
      setIsLoadingGraphs(false);
      setGraphs({
        offset: 0,
        graphList: []
      });
    }
  });

  return (
    <div className="graph-gallery">
      <ul>
        {graphs.graphList.map(g => (<GraphPreview key={g.id} graph={g}></GraphPreview>))}
      </ul>
    </div>
  )
}

function SearchCriteria(props) {
  const [name, setName] = useState("");
  const [creator, setCreator] = useState("");
  const [id, setId] = useState("");
  const [parent_id, setParentId] = useState("");
  const [sort, setSort] = useState("name");

  return (
    <div className="search-criteria">
      <div>
      <label>Name</label><input type="text" value={name} onChange={e => setName(e.target.value)}></input></div>
      <div><label>Creator</label><input type="text" value={creator} onChange={e => setCreator(e.target.value)}></input></div>
      <div><label>ID</label><input type="text" value={id} onChange={e => setId(e.target.value)}></input></div>
      <div><label>Parent ID</label><input type="text" value={parent_id} onChange={e => setParentId(e.target.value)}></input></div>
      <div><label>Sort By</label>
      <select value={sort} onChange={e => setSort(e.target.value)}>
        <option value="id">ID</option>
        <option value="parent_id">Parent ID</option>
        <option value="name">Name</option>
        <option value="creator">Creator</option>
        <option value="upload_date">Upload Date</option>
      </select></div>
      <button onClick={e => props.changeSearchQuery({ name, creator, id, parent_id, sort })}>Search</button>
    </div>
  )
}

function App() {
  const [queryState, setQueryState] = useState({});
  const [shouldEraseGraphs, setShouldEraseGraphs] = useState(false);

  let changeSearchQuery = (query) => {
    setShouldEraseGraphs(true);
    setQueryState(query);
  }

  return (
    <div>
      <header>
        <h1>Desmosearch!</h1>
        <p>(not affiliated with Desmos)</p>
      </header>
      <SearchCriteria changeSearchQuery={changeSearchQuery}></SearchCriteria>
      <GraphGallery queryState={queryState} shouldEraseGraphs={shouldEraseGraphs} setShouldEraseGraphs={setShouldEraseGraphs}></GraphGallery>
    </div>
  )
}

export default App;
