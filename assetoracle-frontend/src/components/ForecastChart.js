import React, { useEffect, useState } from 'react';
import { getForecast } from '../services/api';
import { Line } from 'react-chartjs-2';
import { Chart, LineController, LineElement, PointElement, LinearScale, Title, CategoryScale } from 'chart.js';

Chart.register(LineController, LineElement, PointElement, LinearScale, Title, CategoryScale);

const ForecastChart = ({ symbol }) => {
  const [forecastData, setForecastData] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    getForecast(symbol)
      .then(data => setForecastData(data.forecast))
      .catch(err => {
        console.error(err);
        setError('Could not load forecast.');
      });
  }, [symbol]);

  const data = {
    labels: forecastData ? forecastData.map((_, i) => `Day ${i + 1}`) : [],
    datasets: [
      {
        label: '15-Day Forecast',
        data: forecastData || [],
        fill: false,
        borderColor: 'rgba(75,192,192,1)',
      },
    ],
  };

  return (
    <div>
      <h2>{symbol} 15-Day Forecast</h2>
      {error && <p>{error}</p>}
      {forecastData ? (
        <Line data={data} />
      ) : (
        <p>Loading forecast data...</p>
      )}
    </div>
  );
};

export default ForecastChart;
