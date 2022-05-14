import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import axios from 'axios';

const root = ReactDOM.createRoot(document.getElementById('root'));
console.log("Waking up the backend...");
axios.get("https://still-sierra-43714.herokuapp.com/");
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);