// src/services/api.js
// Using the Fetch API; you can switch to Axios if preferred.
const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8080/api';

export const getAssetPrice = async (symbol) => {
  const response = await fetch(`${API_BASE_URL}/price/${symbol}`);
  if (!response.ok) {
    throw new Error(`Error fetching price for ${symbol}`);
  }
  return response.json();
};

export const simulatePrice = async (symbol, multiplier) => {
  const response = await fetch(`${API_BASE_URL}/simulate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ symbol, expected_multiplier: multiplier })
  });
  if (!response.ok) {
    throw new Error(`Simulation error for ${symbol}`);
  }
  return response.json();
};

export const getForecast = async (symbol) => {
  const response = await fetch(`${API_BASE_URL}/forecast`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ symbol })
  });
  if (!response.ok) {
    throw new Error(`Error retrieving forecast for ${symbol}`);
  }
  return response.json();
};
