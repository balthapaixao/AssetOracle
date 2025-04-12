import React, { useEffect, useState } from 'react';
import { getAssetPrice } from '../services/api';

const AssetPrice = ({ symbol }) => {
  const [price, setPrice] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    getAssetPrice(symbol)
      .then(data => setPrice(data.current_price))
      .catch(err => {
        console.error(err);
        setError('Unable to fetch price.');
      });
  }, [symbol]);

  return (
    <div>
      <h2>{symbol} Price</h2>
      {error ? <p>{error}</p> : price ? <p>${price.toFixed(2)}</p> : <p>Loading...</p>}
    </div>
  );
};

export default AssetPrice;
