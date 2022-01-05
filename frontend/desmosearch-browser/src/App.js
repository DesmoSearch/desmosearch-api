import logo from './logo.svg';
import './App.css';

import React, { useState } from 'react';

// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <img src={logo} className="App-logo" alt="logo" />
//         <p>
//           Edit <code>src/App.js</code> and save to reload.
//         </p>
//         <a
//           className="App-link"
//           href="https://reactjs.org"
//           target="_blank"
//           rel="noopener noreferrer"
//         >
//           Learn React
//         </a>
//       </header>
//     </div>
//   );
// }

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

function GraphGallery(props) {
  let [graphs, setGraphs] = useState({ data: [] });

  (async () => {
    let dataset = await ((await fetch("http://localhost:8000/graphs?limit=100")).json());
    setGraphs(dataset);
  })();

  return (
    <div className="graph-gallery">
      <ul>
        {graphs.data.map(g => (<GraphPreview graph={g}></GraphPreview>))}
      </ul>
    </div>
  )
}

function App() {
  return (
    <div>
      <header>
        <h1>Desmosearch!</h1>
      </header>
      <GraphGallery></GraphGallery>
    </div>
  )
}

export default App;
