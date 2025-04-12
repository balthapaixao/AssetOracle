import React, { useState } from 'react';
import { simulatePrice } from '../services/api';

const PriceSimulator = ({ symbol }) => {
  const [multiplier, setMultiplier] = useState(1.0);
  const [result, setResult] = useState(null);
  const [error, setError] = useState(null);

  const handleSimulate = async () => {
    setError(null);
    try {
      const data = await simulatePrice(symbol, multiplier);
      setResult(data);
    } catch (err) {
      setError('Simulation failed.');
    }
  };

  return (
    <div>
      <h2>Simulate Price for {symbol}</h2>
      <input
        type="number"
        step="0.1"
        value={multiplier}
        onChange={(e) => setMultiplier(parseFloat(e.target.value))}
      />
      <button onClick={handleSimulate}>Simulate</button>
      {error && <p>{error}</p>}
      {result && (
        <div>
          <p>Current Price: ${result.current_price.toFixed(2)}</p>
          <p>Projected Price: ${result.projected_price.toFixed(2)}</p>
        </div>
      )}
    </div>
  );
};

export default PriceSimulator;
