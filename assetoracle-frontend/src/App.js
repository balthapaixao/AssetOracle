import React from 'react';
import AssetPrice from './components/AssetPrice';
import PriceSimulator from './components/PriceSimulator';
import ForecastChart from './components/ForecastChart';

function App() {
  const symbol = 'AAPL';

  return (
    <div style={{ padding: '2rem' }}>
      <h1>AssetOracle Dashboard</h1>
      <AssetPrice symbol={symbol} />
      <PriceSimulator symbol={symbol} />
      <ForecastChart symbol={symbol} />
    </div>
  );
}
export default App;
