import logo from './logo.svg';
import './App.css';
import React, {useState, useEffect} from 'react';
import init, {add, parse_super_attack } from './pkg/tb_parser';

function App() {
  const [sa, setSa] = useState(0);
  init().then(() => setSa(add(788, 2)));
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.<br/>
          Number is: {sa}
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
