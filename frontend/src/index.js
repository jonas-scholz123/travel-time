import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import axios from 'axios';

const root = ReactDOM.createRoot(document.getElementById('root'));
const CONFIG = require("./config.json");

console.log("Waking up the backend...");
axios.get(encodeURI(CONFIG.backendUrl))
  .then(resp => console.log("Backend says: ", resp.data))
  .catch(e => console.log("Health check failed: ", e));

root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);