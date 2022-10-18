import React from 'react';
import { parse_super_attack } from './pkg/tb_parser';
import { SummonMenu } from './components/Menu';

function App() {
  const sa = parse_super_attack("causes immense damage to enemy");
  console.log(sa.modifier);
  return (
    <SummonMenu/>
  );
}

export default App;
