import logo from './logo.svg';
import './App.css';
import React, {useState, useEffect} from 'react';
import { parse_super_attack } from './pkg/tb_parser';
import { SummonMenu } from './components/Menu';
import { User } from './jsdependencies/User';

function App() {
  const [user, setUser] = useState(new User());
  let a = parse_super_attack("causes huge damage to enemy");
  console.log(a.modifier);
  return (
    <div className="App">
      <SummonMenu></SummonMenu>
    </div>
  );
}

export default App;
